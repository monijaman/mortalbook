// Package repository implements the domain.MemorialRepository port using
// PostgreSQL as the durable store and Redis as a Cache-Aside layer.
//
// Cache-Aside pattern (per GetByID):
//  1. Look up key in Redis.
//  2. On hit  → deserialise and return.
//  3. On miss → query PostgreSQL, serialise result, write to Redis with TTL,
//     then return.
//
// On writes (Update) the cache entry is invalidated so subsequent reads see
// the freshest data.
package repository

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/redis/go-redis/v9"
	"go.uber.org/zap"

	"github.com/yourusername/mortalbook/memorial-service/internal/domain"
)

const (
	cacheTTL       = 5 * time.Minute
	cacheKeyPrefix = "memorial:v1:"
	dailyCacheKey  = "memorial:daily:%02d-%02d:p%d:l%d" // month-day-page-limit
	dailyCacheTTL  = 10 * time.Minute
)

// ─────────────────────────────────────────────────────────────────────────────
// Constructor
// ─────────────────────────────────────────────────────────────────────────────

type memorialRepository struct {
	db     *sql.DB
	cache  *redis.Client
	logger *zap.SugaredLogger
}

// New returns a domain.MemorialRepository backed by PostgreSQL + Redis.
// Satisfies the Dependency Inversion Principle — callers depend only on the
// domain interface, not this concrete type.
func New(db *sql.DB, cache *redis.Client, logger *zap.SugaredLogger) domain.MemorialRepository {
	return &memorialRepository{
		db:     db,
		cache:  cache,
		logger: logger,
	}
}

// ─────────────────────────────────────────────────────────────────────────────
// MemorialReader
// ─────────────────────────────────────────────────────────────────────────────

// GetByID implements Cache-Aside: Redis → PostgreSQL.
func (r *memorialRepository) GetByID(ctx context.Context, id string) (*domain.Memorial, error) {
	key := cacheKeyPrefix + id

	// ── 1. Cache probe ────────────────────────────────────────────────────────
	if cached, err := r.cache.Get(ctx, key).Bytes(); err == nil {
		m := &domain.Memorial{}
		if jsonErr := json.Unmarshal(cached, m); jsonErr == nil {
			r.logger.Debugw("cache hit", "key", key)
			return m, nil
		}
	}

	// ── 2. Database fallback ──────────────────────────────────────────────────
	r.logger.Debugw("cache miss", "key", key)

	const q = `
		SELECT id, name, date_of_birth, date_of_death, biography,
		       created_by, status, created_at, updated_at
		FROM   memorials
		WHERE  id = $1 AND deleted_at IS NULL
	`

	m := &domain.Memorial{}
	err := r.db.QueryRowContext(ctx, q, id).Scan(
		&m.ID, &m.Name, &m.DateOfBirth, &m.DateOfDeath,
		&m.Biography, &m.CreatedBy, &m.Status, &m.CreatedAt, &m.UpdatedAt,
	)
	if errors.Is(err, sql.ErrNoRows) {
		return nil, domain.ErrNotFound
	}
	if err != nil {
		return nil, fmt.Errorf("repository.GetByID: %w", err)
	}

	// ── 3. Populate cache ─────────────────────────────────────────────────────
	r.setCache(ctx, key, m, cacheTTL)

	return m, nil
}

// ListByDeathDay returns memorials whose death anniversary (month+day) matches.
// Results are cached per (month, day, limit, offset) tuple.
func (r *memorialRepository) ListByDeathDay(ctx context.Context, month, day, limit, offset int) ([]*domain.Memorial, int64, error) {
	page := offset/limit + 1
	key := fmt.Sprintf(dailyCacheKey, month, day, page, limit)

	// ── 1. Cache probe ────────────────────────────────────────────────────────
	type cachePayload struct {
		Items []*domain.Memorial `json:"items"`
		Total int64              `json:"total"`
	}
	if raw, err := r.cache.Get(ctx, key).Bytes(); err == nil {
		var p cachePayload
		if jsonErr := json.Unmarshal(raw, &p); jsonErr == nil {
			r.logger.Debugw("daily cache hit", "key", key)
			return p.Items, p.Total, nil
		}
	}

	// ── 2. Query ──────────────────────────────────────────────────────────────
	const q = `
		SELECT id, name, date_of_birth, date_of_death, biography,
		       created_by, status, created_at, updated_at
		FROM   memorials
		WHERE  EXTRACT(MONTH FROM date_of_death) = $1
		  AND  EXTRACT(DAY   FROM date_of_death) = $2
		  AND  status     = 'active'
		  AND  deleted_at IS NULL
		ORDER  BY date_of_death ASC
		LIMIT  $3 OFFSET $4
	`
	rows, err := r.db.QueryContext(ctx, q, month, day, limit, offset)
	if err != nil {
		return nil, 0, fmt.Errorf("repository.ListByDeathDay: %w", err)
	}
	defer rows.Close()

	var items []*domain.Memorial
	for rows.Next() {
		m := &domain.Memorial{}
		if scanErr := rows.Scan(
			&m.ID, &m.Name, &m.DateOfBirth, &m.DateOfDeath,
			&m.Biography, &m.CreatedBy, &m.Status, &m.CreatedAt, &m.UpdatedAt,
		); scanErr != nil {
			return nil, 0, fmt.Errorf("repository.ListByDeathDay scan: %w", scanErr)
		}
		items = append(items, m)
	}
	if err = rows.Err(); err != nil {
		return nil, 0, fmt.Errorf("repository.ListByDeathDay rows: %w", err)
	}

	// COUNT query
	var total int64
	const cq = `
		SELECT COUNT(*)
		FROM   memorials
		WHERE  EXTRACT(MONTH FROM date_of_death) = $1
		  AND  EXTRACT(DAY   FROM date_of_death) = $2
		  AND  status     = 'active'
		  AND  deleted_at IS NULL
	`
	if cErr := r.db.QueryRowContext(ctx, cq, month, day).Scan(&total); cErr != nil {
		return nil, 0, fmt.Errorf("repository.ListByDeathDay count: %w", cErr)
	}

	// ── 3. Populate cache ─────────────────────────────────────────────────────
	r.setCache(ctx, key, cachePayload{Items: items, Total: total}, dailyCacheTTL)

	return items, total, nil
}

// ─────────────────────────────────────────────────────────────────────────────
// MemorialWriter
// ─────────────────────────────────────────────────────────────────────────────

// Update applies only the non-nil fields from params, invalidates cache.
func (r *memorialRepository) Update(ctx context.Context, params domain.UpdateMemorialParams) (*domain.Memorial, error) {
	// Build a dynamic SET clause — only touch fields that are non-nil.
	setClauses := []string{"updated_at = NOW()"}
	args := []interface{}{}
	argIdx := 1

	if params.Name != nil {
		setClauses = append(setClauses, fmt.Sprintf("name = $%d", argIdx))
		args = append(args, *params.Name)
		argIdx++
	}
	if params.Biography != nil {
		setClauses = append(setClauses, fmt.Sprintf("biography = $%d", argIdx))
		args = append(args, *params.Biography)
		argIdx++
	}
	if params.Status != nil {
		setClauses = append(setClauses, fmt.Sprintf("status = $%d", argIdx))
		args = append(args, *params.Status)
		argIdx++
	}

	// WHERE id = $N
	args = append(args, params.ID)
	query := fmt.Sprintf(`
		UPDATE memorials
		SET    %s
		WHERE  id = $%d AND deleted_at IS NULL
		RETURNING id, name, date_of_birth, date_of_death, biography,
		          created_by, status, created_at, updated_at
	`, strings.Join(setClauses, ", "), argIdx)

	m := &domain.Memorial{}
	err := r.db.QueryRowContext(ctx, query, args...).Scan(
		&m.ID, &m.Name, &m.DateOfBirth, &m.DateOfDeath,
		&m.Biography, &m.CreatedBy, &m.Status, &m.CreatedAt, &m.UpdatedAt,
	)
	if errors.Is(err, sql.ErrNoRows) {
		return nil, domain.ErrNotFound
	}
	if err != nil {
		return nil, fmt.Errorf("repository.Update: %w", err)
	}

	// Invalidate single-record cache entry.
	r.deleteCache(ctx, cacheKeyPrefix+params.ID)

	return m, nil
}

// ─────────────────────────────────────────────────────────────────────────────
// Cache helpers (private)
// ─────────────────────────────────────────────────────────────────────────────

func (r *memorialRepository) setCache(ctx context.Context, key string, v interface{}, ttl time.Duration) {
	b, err := json.Marshal(v)
	if err != nil {
		r.logger.Warnw("cache marshal failed", "key", key, "error", err)
		return
	}
	if err = r.cache.Set(ctx, key, b, ttl).Err(); err != nil {
		r.logger.Warnw("cache write failed", "key", key, "error", err)
	}
}

func (r *memorialRepository) deleteCache(ctx context.Context, key string) {
	if err := r.cache.Del(ctx, key).Err(); err != nil {
		r.logger.Warnw("cache delete failed", "key", key, "error", err)
	}
}
