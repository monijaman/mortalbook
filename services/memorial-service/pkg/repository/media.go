package repository

import (
	"context"
	"database/sql"

	"github.com/yourusername/mortalbook/memorial-service/pkg/domain"
)

type MediaRepository interface {
	GetByMemorialID(ctx context.Context, memorialID string) ([]*domain.MemorialMedia, error)
	Create(ctx context.Context, media *domain.MemorialMedia) error
	Delete(ctx context.Context, id string) error
}

type mediaRepo struct {
	db *sql.DB
}

func NewMediaRepository(db *sql.DB) MediaRepository {
	return &mediaRepo{db: db}
}

func (r *mediaRepo) GetByMemorialID(ctx context.Context, memorialID string) ([]*domain.MemorialMedia, error) {
	query := `
		SELECT id, memorial_id, url, media_type, description, created_at
		FROM memorial_media
		WHERE memorial_id = $1
		ORDER BY created_at DESC
	`

	rows, err := r.db.QueryContext(ctx, query, memorialID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	medias := []*domain.MemorialMedia{}
	for rows.Next() {
		media := &domain.MemorialMedia{}
		err := rows.Scan(
			&media.ID,
			&media.MemorialID,
			&media.URL,
			&media.MediaType,
			&media.Description,
			&media.CreatedAt,
		)
		if err != nil {
			return nil, err
		}
		medias = append(medias, media)
	}

	return medias, nil
}

func (r *mediaRepo) Create(ctx context.Context, media *domain.MemorialMedia) error {
	query := `
		INSERT INTO memorial_media (id, memorial_id, url, media_type, description, created_at)
		VALUES ($1, $2, $3, $4, $5, $6)
	`

	_, err := r.db.ExecContext(ctx, query,
		media.ID,
		media.MemorialID,
		media.URL,
		media.MediaType,
		media.Description,
		media.CreatedAt,
	)

	return err
}

func (r *mediaRepo) Delete(ctx context.Context, id string) error {
	query := "DELETE FROM memorial_media WHERE id = $1"
	_, err := r.db.ExecContext(ctx, query, id)
	return err
}
