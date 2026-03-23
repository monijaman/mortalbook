package service

import (
	"context"

	"github.com/google/uuid"
	"github.com/segmentio/kafka-go"
	"go.uber.org/zap"

	"github.com/yourusername/mortalbook/memorial-service/pkg/domain"
	"github.com/yourusername/mortalbook/memorial-service/pkg/repository"
)

type MemorialService interface {
	GetMemorial(ctx context.Context, id string) (*domain.Memorial, error)
	ListMemorials(ctx context.Context, filter *domain.MemorialFilter) ([]*domain.Memorial, int64, error)
	CreateMemorial(ctx context.Context, memorial *domain.Memorial) (string, error)
	UpdateMemorial(ctx context.Context, memorial *domain.Memorial) error
	DeleteMemorial(ctx context.Context, id string) error
	GetMemorialMedia(ctx context.Context, memorialID string) ([]*domain.MemorialMedia, error)
}

type memorialService struct {
	memorialRepo repository.MemorialRepository
	mediaRepo    repository.MediaRepository
	kafka        *kafka.Conn
	logger       *zap.SugaredLogger
}

func NewMemorialService(
	memorialRepo repository.MemorialRepository,
	mediaRepo repository.MediaRepository,
	kafka *kafka.Conn,
	logger *zap.SugaredLogger,
) MemorialService {
	return &memorialService{
		memorialRepo: memorialRepo,
		mediaRepo:    mediaRepo,
		kafka:        kafka,
		logger:       logger,
	}
}

func (s *memorialService) GetMemorial(ctx context.Context, id string) (*domain.Memorial, error) {
	memorial, err := s.memorialRepo.GetByID(ctx, id)
	if err != nil {
		s.logger.Errorw("failed to get memorial", "id", id, "error", err)
		return nil, err
	}
	return memorial, nil
}

func (s *memorialService) ListMemorials(ctx context.Context, filter *domain.MemorialFilter) ([]*domain.Memorial, int64, error) {
	memorials, total, err := s.memorialRepo.List(ctx, filter)
	if err != nil {
		s.logger.Errorw("failed to list memorials", "error", err)
		return nil, 0, err
	}
	return memorials, total, nil
}

func (s *memorialService) CreateMemorial(ctx context.Context, memorial *domain.Memorial) (string, error) {
	memorial.ID = uuid.New().String()

	id, err := s.memorialRepo.Create(ctx, memorial)
	if err != nil {
		s.logger.Errorw("failed to create memorial", "error", err)
		return "", err
	}

	// Publish event to Kafka
	s.publishEvent(ctx, "memorial.created", map[string]interface{}{
		"id":   id,
		"name": memorial.Name,
	})

	s.logger.Infow("memorial created", "id", id)
	return id, nil
}

func (s *memorialService) UpdateMemorial(ctx context.Context, memorial *domain.Memorial) error {
	err := s.memorialRepo.Update(ctx, memorial)
	if err != nil {
		s.logger.Errorw("failed to update memorial", "id", memorial.ID, "error", err)
		return err
	}

	s.publishEvent(ctx, "memorial.updated", map[string]interface{}{
		"id":   memorial.ID,
		"name": memorial.Name,
	})

	s.logger.Infow("memorial updated", "id", memorial.ID)
	return nil
}

func (s *memorialService) DeleteMemorial(ctx context.Context, id string) error {
	err := s.memorialRepo.Delete(ctx, id)
	if err != nil {
		s.logger.Errorw("failed to delete memorial", "id", id, "error", err)
		return err
	}

	s.publishEvent(ctx, "memorial.deleted", map[string]interface{}{
		"id": id,
	})

	s.logger.Infow("memorial deleted", "id", id)
	return nil
}

func (s *memorialService) GetMemorialMedia(ctx context.Context, memorialID string) ([]*domain.MemorialMedia, error) {
	medias, err := s.mediaRepo.GetByMemorialID(ctx, memorialID)
	if err != nil {
		s.logger.Errorw("failed to get memorial media", "memorial_id", memorialID, "error", err)
		return nil, err
	}
	return medias, nil
}

func (s *memorialService) publishEvent(ctx context.Context, eventType string, data map[string]interface{}) {
	// TODO: Implement Kafka event publishing
	s.logger.Debugw("event published", "type", eventType, "data", data)
}
