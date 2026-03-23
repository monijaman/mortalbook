// Package handler wires the gRPC transport layer to the use-case layer.
// It contains no business logic — each method translates proto request
// → use-case input, calls the use-case, then maps the output → proto response.
//
// Following the Single Responsibility and Dependency Inversion Principles,
// this file only depends on the usecase.MemorialUseCase interface, never on
// the concrete usecase or repository implementations.
package handler

import (
	"context"
	"errors"

	"go.uber.org/zap"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/timestamppb"

	pb "github.com/yourusername/mortalbook/memorial-service/gen/memorial/v1"
	"github.com/yourusername/mortalbook/memorial-service/internal/domain"
	"github.com/yourusername/mortalbook/memorial-service/internal/usecase"
)

// ─────────────────────────────────────────────────────────────────────────────
// Server struct
// ─────────────────────────────────────────────────────────────────────────────

// GRPCServer implements pb.MemorialServiceServer.
type GRPCServer struct {
	pb.UnimplementedMemorialServiceServer

	uc     usecase.MemorialUseCase
	logger *zap.SugaredLogger
}

// NewGRPCServer creates a GRPCServer.  Accepts the interface so the real
// implementation and any test double are equally valid.
func NewGRPCServer(uc usecase.MemorialUseCase, logger *zap.SugaredLogger) *GRPCServer {
	return &GRPCServer{uc: uc, logger: logger}
}

// ─────────────────────────────────────────────────────────────────────────────
// RPC: GetDailyMemorials
// ─────────────────────────────────────────────────────────────────────────────

func (s *GRPCServer) GetDailyMemorials(
	ctx context.Context,
	req *pb.GetDailyMemorialsRequest,
) (*pb.GetDailyMemorialsResponse, error) {

	out, err := s.uc.GetDailyMemorials(ctx, usecase.GetDailyMemorialsInput{
		Date:  req.GetDate(),
		Limit: req.GetLimit(),
		Page:  req.GetPage(),
	})
	if err != nil {
		return nil, s.mapError(err)
	}

	pbMemorials := make([]*pb.Memorial, 0, len(out.Memorials))
	for _, m := range out.Memorials {
		pbMemorials = append(pbMemorials, domainToProto(m))
	}

	return &pb.GetDailyMemorialsResponse{
		Memorials: pbMemorials,
		Total:     out.Total,
		Page:      out.Page,
		Limit:     out.Limit,
	}, nil
}

// ─────────────────────────────────────────────────────────────────────────────
// RPC: UpdateMemorial
// ─────────────────────────────────────────────────────────────────────────────

func (s *GRPCServer) UpdateMemorial(
	ctx context.Context,
	req *pb.UpdateMemorialRequest,
) (*pb.UpdateMemorialResponse, error) {

	in := usecase.UpdateMemorialInput{ID: req.GetId()}

	// Only set pointer field when the proto field is non-empty — this preserves
	// the "patch" semantics: an absent field means "leave unchanged".
	if v := req.GetName(); v != "" {
		in.Name = &v
	}
	if v := req.GetBiography(); v != "" {
		in.Biography = &v
	}
	if v := req.GetStatus(); v != "" {
		in.Status = &v
	}

	out, err := s.uc.UpdateMemorial(ctx, in)
	if err != nil {
		return nil, s.mapError(err)
	}

	return &pb.UpdateMemorialResponse{
		Memorial: domainToProto(out.Memorial),
	}, nil
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

// domainToProto converts a domain.Memorial to its protobuf representation.
func domainToProto(m *domain.Memorial) *pb.Memorial {
	return &pb.Memorial{
		Id:          m.ID,
		Name:        m.Name,
		DateOfBirth: m.DateOfBirth.Format("2006-01-02"),
		DateOfDeath: m.DateOfDeath.Format("2006-01-02"),
		Biography:   m.Biography,
		CreatedBy:   m.CreatedBy,
		Status:      m.Status,
		CreatedAt:   timestamppb.New(m.CreatedAt),
		UpdatedAt:   timestamppb.New(m.UpdatedAt),
	}
}

// mapError translates domain / sentinel errors to gRPC status codes.
func (s *GRPCServer) mapError(err error) error {
	switch {
	case errors.Is(err, domain.ErrNotFound):
		return status.Errorf(codes.NotFound, "%s", err)
	case errors.Is(err, domain.ErrInvalidInput):
		return status.Errorf(codes.InvalidArgument, "%s", err)
	default:
		s.logger.Errorw("unhandled error", "error", err)
		return status.Errorf(codes.Internal, "internal server error")
	}
}
