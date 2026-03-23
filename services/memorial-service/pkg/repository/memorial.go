package repository

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/yourusername/mortalbook/memorial-service/pkg/domain"
)

type MemorialRepository interface {
	GetByID(ctx context.Context, id string) (*domain.Memorial, error)
	List(ctx context.Context, filter *domain.MemorialFilter) ([]*domain.Memorial, int64, error)
	Create(ctx context.Context, memorial *domain.Memorial) (string, error)
	Update(ctx context.Context, memorial *domain.Memorial) error
	Delete(ctx context.Context, id string) error
}

type memorialRepo struct {
	db    *sql.DB
	redis *redis.Client
}

func NewMemorialRepository(db *sql.DB, redis *redis.Client) MemorialRepository {
	return &memorialRepo{db: db, redis: redis}
}

const cacheTTL = 10 * time.Minute

func (r *memorialRepo) GetByID(ctx context.Context, id string) (*domain.Memorial, error) {
	cacheKey := fmt.Sprintf("memorial:%s", id)

	// Try Redis cache first.
	if r.redis != nil {
		if raw, err := r.redis.Get(ctx, cacheKey).Result(); err == nil {
			var cached domain.Memorial
			if jsonErr := json.Unmarshal([]byte(raw), &cached); jsonErr == nil {
				return &cached, nil
			}
		}
	}

	query := `
		SELECT id, name, date_of_birth, date_of_death, biography, created_by, created_at, updated_at, status
		FROM memorials
		WHERE id = $1
	`

	memorial := &domain.Memorial{}
	err := r.db.QueryRowContext(ctx, query, id).Scan(
		&memorial.ID,
		&memorial.Name,
		&memorial.DateOfBirth,
		&memorial.DateOfDeath,
		&memorial.Biography,
		&memorial.CreatedBy,
		&memorial.CreatedAt,
		&memorial.UpdatedAt,
		&memorial.Status,
	)

	if err == sql.ErrNoRows {
		return nil, fmt.Errorf("memorial not found")
	}
	if err != nil {
		return nil, err
	}

	// Populate cache for subsequent reads.
	if r.redis != nil {
		if data, jsonErr := json.Marshal(memorial); jsonErr == nil {
			_ = r.redis.Set(ctx, cacheKey, data, cacheTTL).Err()
		}
	}

	return memorial, nil
}

func (r *memorialRepo) List(ctx context.Context, filter *domain.MemorialFilter) ([]*domain.Memorial, int64, error) {
	query := `
		SELECT id, name, date_of_birth, date_of_death, biography, created_by, created_at, updated_at, status
		FROM memorials
		WHERE status = $1
		ORDER BY created_at DESC
		LIMIT $2 OFFSET $3
	`

	rows, err := r.db.QueryContext(ctx, query, "active", filter.Limit, filter.Offset)
	if err != nil {
		return nil, 0, err
	}
	defer rows.Close()

	memorials := []*domain.Memorial{}
	for rows.Next() {
		memorial := &domain.Memorial{}
		err := rows.Scan(
			&memorial.ID,
			&memorial.Name,
			&memorial.DateOfBirth,
			&memorial.DateOfDeath,
			&memorial.Biography,
			&memorial.CreatedBy,
			&memorial.CreatedAt,
			&memorial.UpdatedAt,
			&memorial.Status,
		)
		if err != nil {
			return nil, 0, err
		}
		memorials = append(memorials, memorial)
	}

	// Get total count
	countQuery := "SELECT COUNT(*) FROM memorials WHERE status = $1"
	var total int64
	r.db.QueryRowContext(ctx, countQuery, "active").Scan(&total)

	return memorials, total, nil
}

func (r *memorialRepo) Create(ctx context.Context, memorial *domain.Memorial) (string, error) {
	query := `
		INSERT INTO memorials (id, name, date_of_birth, date_of_death, biography, created_by, created_at, updated_at, status)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	`

	id := memorial.ID
	err := r.db.QueryRowContext(ctx, query,
		id,
		memorial.Name,
		memorial.DateOfBirth,
		memorial.DateOfDeath,
		memorial.Biography,
		memorial.CreatedBy,
		time.Now(),
		time.Now(),
		"active",
	).Err()

	if err != nil {
		return "", err
	}

	return id, nil
}

func (r *memorialRepo) Update(ctx context.Context, memorial *domain.Memorial) error {
	query := `
		UPDATE memorials
		SET name = $1, biography = $2, updated_at = $3
		WHERE id = $4
	`

	_, err := r.db.ExecContext(ctx, query,
		memorial.Name,
		memorial.Biography,
		time.Now(),
		memorial.ID,
	)

	return err
}

func (r *memorialRepo) Delete(ctx context.Context, id string) error {
	query := "DELETE FROM memorials WHERE id = $1"
	_, err := r.db.ExecContext(ctx, query, id)
	return err
}
