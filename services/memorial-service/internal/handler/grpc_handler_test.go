package handler_test

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"

	pb "github.com/yourusername/mortalbook/memorial-service/gen/memorial/v1"
	"github.com/yourusername/mortalbook/memorial-service/internal/domain"
	grpchandler "github.com/yourusername/mortalbook/memorial-service/internal/handler"
	"github.com/yourusername/mortalbook/memorial-service/internal/usecase"
)

// ─────────────────────────────────────────────────────────────────────────────
// Mock use-case
// ─────────────────────────────────────────────────────────────────────────────

type mockUC struct {
	getDailyFn func(ctx context.Context, in usecase.GetDailyMemorialsInput) (usecase.GetDailyMemorialsOutput, error)
	updateFn   func(ctx context.Context, in usecase.UpdateMemorialInput) (usecase.UpdateMemorialOutput, error)
}

func (m *mockUC) GetDailyMemorials(ctx context.Context, in usecase.GetDailyMemorialsInput) (usecase.GetDailyMemorialsOutput, error) {
	return m.getDailyFn(ctx, in)
}
func (m *mockUC) UpdateMemorial(ctx context.Context, in usecase.UpdateMemorialInput) (usecase.UpdateMemorialOutput, error) {
	return m.updateFn(ctx, in)
}

// ─────────────────────────────────────────────────────────────────────────────
// GetDailyMemorials
// ─────────────────────────────────────────────────────────────────────────────

func TestGRPCHandler_GetDailyMemorials_OK(t *testing.T) {
	now := time.Now()
	uc := &mockUC{
		getDailyFn: func(_ context.Context, in usecase.GetDailyMemorialsInput) (usecase.GetDailyMemorialsOutput, error) {
			assert.Equal(t, "2024-01-15", in.Date)
			assert.Equal(t, int32(10), in.Limit)
			assert.Equal(t, int32(1), in.Page)

			return usecase.GetDailyMemorialsOutput{
				Memorials: []*domain.Memorial{
					{ID: "m1", Name: "Alice", DateOfBirth: now, DateOfDeath: now, Status: "active", CreatedAt: now, UpdatedAt: now},
				},
				Total: 1,
				Page:  1,
				Limit: 10,
			}, nil
		},
	}

	srv := grpchandler.NewGRPCServer(uc, zap.NewNop().Sugar())
	resp, err := srv.GetDailyMemorials(context.Background(), &pb.GetDailyMemorialsRequest{
		Date:  "2024-01-15",
		Limit: 10,
		Page:  1,
	})

	require.NoError(t, err)
	assert.Equal(t, int64(1), resp.Total)
	assert.Len(t, resp.Memorials, 1)
	assert.Equal(t, "m1", resp.Memorials[0].Id)
	assert.Equal(t, "Alice", resp.Memorials[0].Name)
}

func TestGRPCHandler_GetDailyMemorials_InvalidInput(t *testing.T) {
	uc := &mockUC{
		getDailyFn: func(_ context.Context, _ usecase.GetDailyMemorialsInput) (usecase.GetDailyMemorialsOutput, error) {
			return usecase.GetDailyMemorialsOutput{}, domain.ErrInvalidInput
		},
	}

	srv := grpchandler.NewGRPCServer(uc, zap.NewNop().Sugar())
	_, err := srv.GetDailyMemorials(context.Background(), &pb.GetDailyMemorialsRequest{Date: ""})

	require.Error(t, err)
	st, ok := status.FromError(err)
	require.True(t, ok)
	assert.Equal(t, codes.InvalidArgument, st.Code())
}

func TestGRPCHandler_GetDailyMemorials_InternalError(t *testing.T) {
	uc := &mockUC{
		getDailyFn: func(_ context.Context, _ usecase.GetDailyMemorialsInput) (usecase.GetDailyMemorialsOutput, error) {
			return usecase.GetDailyMemorialsOutput{}, errors.New("unexpected db error")
		},
	}

	srv := grpchandler.NewGRPCServer(uc, zap.NewNop().Sugar())
	_, err := srv.GetDailyMemorials(context.Background(), &pb.GetDailyMemorialsRequest{Date: "2024-01-15"})

	require.Error(t, err)
	st, _ := status.FromError(err)
	assert.Equal(t, codes.Internal, st.Code())
}

// ─────────────────────────────────────────────────────────────────────────────
// UpdateMemorial
// ─────────────────────────────────────────────────────────────────────────────

func TestGRPCHandler_UpdateMemorial_OK(t *testing.T) {
	now := time.Now()
	uc := &mockUC{
		updateFn: func(_ context.Context, in usecase.UpdateMemorialInput) (usecase.UpdateMemorialOutput, error) {
			assert.Equal(t, "m1", in.ID)
			require.NotNil(t, in.Name)
			assert.Equal(t, "New Name", *in.Name)
			assert.Nil(t, in.Status) // not provided in request

			return usecase.UpdateMemorialOutput{
				Memorial: &domain.Memorial{
					ID: "m1", Name: "New Name",
					DateOfBirth: now, DateOfDeath: now,
					Status: "active", CreatedAt: now, UpdatedAt: now,
				},
			}, nil
		},
	}

	srv := grpchandler.NewGRPCServer(uc, zap.NewNop().Sugar())
	resp, err := srv.UpdateMemorial(context.Background(), &pb.UpdateMemorialRequest{
		Id:   "m1",
		Name: "New Name",
		// Status intentionally omitted → nil in use-case input
	})

	require.NoError(t, err)
	assert.Equal(t, "m1", resp.Memorial.Id)
	assert.Equal(t, "New Name", resp.Memorial.Name)
}

func TestGRPCHandler_UpdateMemorial_NotFound(t *testing.T) {
	uc := &mockUC{
		updateFn: func(_ context.Context, _ usecase.UpdateMemorialInput) (usecase.UpdateMemorialOutput, error) {
			return usecase.UpdateMemorialOutput{}, domain.ErrNotFound
		},
	}

	srv := grpchandler.NewGRPCServer(uc, zap.NewNop().Sugar())
	_, err := srv.UpdateMemorial(context.Background(), &pb.UpdateMemorialRequest{Id: "ghost"})

	require.Error(t, err)
	st, _ := status.FromError(err)
	assert.Equal(t, codes.NotFound, st.Code())
}

func TestGRPCHandler_UpdateMemorial_InvalidArgument(t *testing.T) {
	uc := &mockUC{
		updateFn: func(_ context.Context, _ usecase.UpdateMemorialInput) (usecase.UpdateMemorialOutput, error) {
			return usecase.UpdateMemorialOutput{}, domain.ErrInvalidInput
		},
	}

	srv := grpchandler.NewGRPCServer(uc, zap.NewNop().Sugar())
	_, err := srv.UpdateMemorial(context.Background(), &pb.UpdateMemorialRequest{Id: "m1", Status: "deleted"})

	require.Error(t, err)
	st, _ := status.FromError(err)
	assert.Equal(t, codes.InvalidArgument, st.Code())
}
