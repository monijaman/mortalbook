package usecase_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/yourusername/mortalbook/memorial-service/internal/domain"
	"github.com/yourusername/mortalbook/memorial-service/internal/usecase"
	"go.uber.org/zap"
)

// ─────────────────────────────────────────────────────────────────────────────
// In-process mock repository — no external dependencies needed.
// ─────────────────────────────────────────────────────────────────────────────

type mockRepo struct {
	// stubs: set per test
	getByIDFn      func(ctx context.Context, id string) (*domain.Memorial, error)
	listByDeathDay func(ctx context.Context, month, day, limit, offset int) ([]*domain.Memorial, int64, error)
	updateFn       func(ctx context.Context, params domain.UpdateMemorialParams) (*domain.Memorial, error)
}

func (m *mockRepo) GetByID(ctx context.Context, id string) (*domain.Memorial, error) {
	return m.getByIDFn(ctx, id)
}
func (m *mockRepo) ListByDeathDay(ctx context.Context, month, day, limit, offset int) ([]*domain.Memorial, int64, error) {
	return m.listByDeathDay(ctx, month, day, limit, offset)
}
func (m *mockRepo) Update(ctx context.Context, params domain.UpdateMemorialParams) (*domain.Memorial, error) {
	return m.updateFn(ctx, params)
}

// ─────────────────────────────────────────────────────────────────────────────
// GetDailyMemorials
// ─────────────────────────────────────────────────────────────────────────────

func TestGetDailyMemorials_OK(t *testing.T) {
	now := time.Now()
	repo := &mockRepo{
		listByDeathDay: func(_ context.Context, month, day, limit, offset int) ([]*domain.Memorial, int64, error) {
			assert.Equal(t, 1, month)
			assert.Equal(t, 15, day)
			assert.Equal(t, 50, limit)
			assert.Equal(t, 0, offset)

			return []*domain.Memorial{
				{ID: "m1", Name: "Alice", DateOfDeath: now, Status: "active"},
			}, 1, nil
		},
	}

	uc := usecase.New(repo, zap.NewNop().Sugar())
	out, err := uc.GetDailyMemorials(context.Background(), usecase.GetDailyMemorialsInput{
		Date:  "2024-01-15",
		Limit: 50,
		Page:  1,
	})

	require.NoError(t, err)
	assert.Equal(t, int64(1), out.Total)
	assert.Len(t, out.Memorials, 1)
	assert.Equal(t, "Alice", out.Memorials[0].Name)
}

func TestGetDailyMemorials_DefaultPagination(t *testing.T) {
	repo := &mockRepo{
		listByDeathDay: func(_ context.Context, _, _, limit, offset int) ([]*domain.Memorial, int64, error) {
			assert.Equal(t, 50, limit) // default
			assert.Equal(t, 0, offset) // page 1 → offset 0
			return nil, 0, nil
		},
	}

	uc := usecase.New(repo, zap.NewNop().Sugar())
	_, err := uc.GetDailyMemorials(context.Background(), usecase.GetDailyMemorialsInput{
		Date: "01-15", // MM-DD format
	})
	require.NoError(t, err)
}

func TestGetDailyMemorials_EmptyDate(t *testing.T) {
	uc := usecase.New(&mockRepo{}, zap.NewNop().Sugar())
	_, err := uc.GetDailyMemorials(context.Background(), usecase.GetDailyMemorialsInput{})

	require.Error(t, err)
	assert.True(t, errors.Is(err, domain.ErrInvalidInput))
}

func TestGetDailyMemorials_InvalidDate(t *testing.T) {
	uc := usecase.New(&mockRepo{}, zap.NewNop().Sugar())
	_, err := uc.GetDailyMemorials(context.Background(), usecase.GetDailyMemorialsInput{
		Date: "not-a-date",
	})

	require.Error(t, err)
	assert.True(t, errors.Is(err, domain.ErrInvalidInput))
}

func TestGetDailyMemorials_RepoError(t *testing.T) {
	repo := &mockRepo{
		listByDeathDay: func(_ context.Context, _, _, _, _ int) ([]*domain.Memorial, int64, error) {
			return nil, 0, errors.New("db timeout")
		},
	}

	uc := usecase.New(repo, zap.NewNop().Sugar())
	_, err := uc.GetDailyMemorials(context.Background(), usecase.GetDailyMemorialsInput{
		Date: "2024-01-15",
	})

	require.Error(t, err)
	// domain errors are not wrapped at repo level here; original error propagates
	assert.Contains(t, err.Error(), "db timeout")
}

// ─────────────────────────────────────────────────────────────────────────────
// UpdateMemorial
// ─────────────────────────────────────────────────────────────────────────────

func TestUpdateMemorial_OK(t *testing.T) {
	name := "Bob Updated"
	bio := "New bio"

	repo := &mockRepo{
		updateFn: func(_ context.Context, p domain.UpdateMemorialParams) (*domain.Memorial, error) {
			assert.Equal(t, "m1", p.ID)
			require.NotNil(t, p.Name)
			assert.Equal(t, "Bob Updated", *p.Name)
			require.NotNil(t, p.Biography)
			assert.Equal(t, "New bio", *p.Biography)
			assert.Nil(t, p.Status)

			return &domain.Memorial{ID: "m1", Name: *p.Name, Biography: *p.Biography, Status: "active"}, nil
		},
	}

	uc := usecase.New(repo, zap.NewNop().Sugar())
	out, err := uc.UpdateMemorial(context.Background(), usecase.UpdateMemorialInput{
		ID:        "m1",
		Name:      &name,
		Biography: &bio,
	})

	require.NoError(t, err)
	assert.Equal(t, "Bob Updated", out.Memorial.Name)
}

func TestUpdateMemorial_MissingID(t *testing.T) {
	uc := usecase.New(&mockRepo{}, zap.NewNop().Sugar())
	_, err := uc.UpdateMemorial(context.Background(), usecase.UpdateMemorialInput{})

	require.Error(t, err)
	assert.True(t, errors.Is(err, domain.ErrInvalidInput))
}

func TestUpdateMemorial_EmptyName(t *testing.T) {
	empty := ""
	uc := usecase.New(&mockRepo{}, zap.NewNop().Sugar())
	_, err := uc.UpdateMemorial(context.Background(), usecase.UpdateMemorialInput{
		ID:   "m1",
		Name: &empty,
	})

	require.Error(t, err)
	assert.True(t, errors.Is(err, domain.ErrInvalidInput))
}

func TestUpdateMemorial_InvalidStatus(t *testing.T) {
	bad := "deleted"
	uc := usecase.New(&mockRepo{}, zap.NewNop().Sugar())
	_, err := uc.UpdateMemorial(context.Background(), usecase.UpdateMemorialInput{
		ID:     "m1",
		Status: &bad,
	})

	require.Error(t, err)
	assert.True(t, errors.Is(err, domain.ErrInvalidInput))
}

func TestUpdateMemorial_NotFound(t *testing.T) {
	repo := &mockRepo{
		updateFn: func(_ context.Context, _ domain.UpdateMemorialParams) (*domain.Memorial, error) {
			return nil, domain.ErrNotFound
		},
	}

	uc := usecase.New(repo, zap.NewNop().Sugar())
	_, err := uc.UpdateMemorial(context.Background(), usecase.UpdateMemorialInput{ID: "ghost"})

	require.Error(t, err)
	assert.True(t, errors.Is(err, domain.ErrNotFound))
}
