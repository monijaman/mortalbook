// Package usecase contains the application use-cases — one file per use-case.
// This layer ONLY depends on the domain.  It never imports net/http, gRPC or
// any storage package; it receives its dependencies through interfaces
// (Dependency Inversion Principle).
package usecase

import (
	"context"
	"fmt"
	"time"

	"go.uber.org/zap"

	"github.com/yourusername/mortalbook/memorial-service/internal/domain"
)

// ─────────────────────────────────────────────────────────────────────────────
// Port (interface for the use-case — lets handlers depend on abstractions)
// ─────────────────────────────────────────────────────────────────────────────

// MemorialUseCase is the inbound port consumed by the gRPC handler.
type MemorialUseCase interface {
	GetDailyMemorials(ctx context.Context, in GetDailyMemorialsInput) (GetDailyMemorialsOutput, error)
	UpdateMemorial(ctx context.Context, in UpdateMemorialInput) (UpdateMemorialOutput, error)
}

// ─────────────────────────────────────────────────────────────────────────────
// Input / Output DTOs
// ─────────────────────────────────────────────────────────────────────────────

// GetDailyMemorialsInput carries the caller's intent.
type GetDailyMemorialsInput struct {
	// Date in "YYYY-MM-DD" or "MM-DD" format.  Day+Month are extracted.
	Date  string
	Limit int32
	Page  int32
}

// GetDailyMemorialsOutput is the use-case result.
type GetDailyMemorialsOutput struct {
	Memorials []*domain.Memorial
	Total     int64
	Page      int32
	Limit     int32
}

// UpdateMemorialInput carries the patch parameters.
type UpdateMemorialInput struct {
	ID        string
	Name      *string // nil = do not update
	Biography *string
	Status    *string
}

// UpdateMemorialOutput wraps the updated entity.
type UpdateMemorialOutput struct {
	Memorial *domain.Memorial
}

// ─────────────────────────────────────────────────────────────────────────────
// Implementation
// ─────────────────────────────────────────────────────────────────────────────

type memorialUseCase struct {
	repo   domain.MemorialRepository
	logger *zap.SugaredLogger
}

// New creates a MemorialUseCase with the supplied repository and logger.
// Accepts the domain.MemorialRepository interface — fully mockable for tests.
func New(repo domain.MemorialRepository, logger *zap.SugaredLogger) MemorialUseCase {
	return &memorialUseCase{repo: repo, logger: logger}
}

// ── GetDailyMemorials ─────────────────────────────────────────────────────────

func (u *memorialUseCase) GetDailyMemorials(ctx context.Context, in GetDailyMemorialsInput) (GetDailyMemorialsOutput, error) {
	// ── Input validation ──────────────────────────────────────────────────────
	if in.Date == "" {
		return GetDailyMemorialsOutput{}, fmt.Errorf("%w: date is required", domain.ErrInvalidInput)
	}

	month, day, err := parseMonthDay(in.Date)
	if err != nil {
		return GetDailyMemorialsOutput{}, fmt.Errorf("%w: %s", domain.ErrInvalidInput, err)
	}

	limit := int(in.Limit)
	if limit <= 0 {
		limit = 50
	}
	if limit > 200 {
		limit = 200
	}

	page := int(in.Page)
	if page <= 0 {
		page = 1
	}
	offset := (page - 1) * limit

	// ── Repository call ───────────────────────────────────────────────────────
	memorials, total, err := u.repo.ListByDeathDay(ctx, month, day, limit, offset)
	if err != nil {
		u.logger.Errorw("GetDailyMemorials failed",
			"month", month, "day", day, "error", err)
		return GetDailyMemorialsOutput{}, fmt.Errorf("GetDailyMemorials: %w", err)
	}

	u.logger.Infow("GetDailyMemorials",
		"month", month, "day", day,
		"returned", len(memorials), "total", total)

	return GetDailyMemorialsOutput{
		Memorials: memorials,
		Total:     total,
		Page:      int32(page),
		Limit:     int32(limit),
	}, nil
}

// ── UpdateMemorial ───────────────────────────────────────────────────────────

func (u *memorialUseCase) UpdateMemorial(ctx context.Context, in UpdateMemorialInput) (UpdateMemorialOutput, error) {
	// ── Input validation ──────────────────────────────────────────────────────
	if in.ID == "" {
		return UpdateMemorialOutput{}, fmt.Errorf("%w: id is required", domain.ErrInvalidInput)
	}
	if in.Name != nil && len(*in.Name) == 0 {
		return UpdateMemorialOutput{}, fmt.Errorf("%w: name cannot be empty", domain.ErrInvalidInput)
	}
	if in.Status != nil {
		if *in.Status != "active" && *in.Status != "archived" {
			return UpdateMemorialOutput{}, fmt.Errorf("%w: status must be 'active' or 'archived'", domain.ErrInvalidInput)
		}
	}

	// ── Repository call ───────────────────────────────────────────────────────
	updated, err := u.repo.Update(ctx, domain.UpdateMemorialParams{
		ID:        in.ID,
		Name:      in.Name,
		Biography: in.Biography,
		Status:    in.Status,
	})
	if err != nil {
		u.logger.Errorw("UpdateMemorial failed", "id", in.ID, "error", err)
		return UpdateMemorialOutput{}, fmt.Errorf("UpdateMemorial: %w", err)
	}

	u.logger.Infow("UpdateMemorial succeeded", "id", in.ID)
	return UpdateMemorialOutput{Memorial: updated}, nil
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

// parseMonthDay accepts both "YYYY-MM-DD" and "MM-DD" date strings.
func parseMonthDay(s string) (month, day int, err error) {
	var t time.Time

	// Try full RFC-3339 date first.
	t, err = time.Parse("2006-01-02", s)
	if err == nil {
		return int(t.Month()), t.Day(), nil
	}

	// Fallback to MM-DD
	t, err = time.Parse("01-02", s)
	if err == nil {
		return int(t.Month()), t.Day(), nil
	}

	return 0, 0, fmt.Errorf("unsupported date format %q (expected YYYY-MM-DD or MM-DD)", s)
}
