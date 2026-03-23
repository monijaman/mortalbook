// Package domain – repository port (interface).
// Defined here so the domain layer owns the contract; the repository layer
// (adapter) satisfies it.  No concrete type is touched in this file.
package domain

import "context"

// MemorialReader is the read-side port.
type MemorialReader interface {
	// GetByID returns a single Memorial by primary key.
	// Returns ErrNotFound when no row matches.
	GetByID(ctx context.Context, id string) (*Memorial, error)

	// ListByDeathDay returns memorials whose death anniversary (day + month)
	// matches the supplied day/month values, with pagination.
	ListByDeathDay(ctx context.Context, month, day int, limit, offset int) ([]*Memorial, int64, error)
}

// MemorialWriter is the write-side port.
type MemorialWriter interface {
	// Update persists only the non-nil fields in params.
	// Returns ErrNotFound if id doesn't exist.
	Update(ctx context.Context, params UpdateMemorialParams) (*Memorial, error)
}

// MemorialRepository is the full port used by the usecase layer.
// Embeds both read and write so they can be composed or mocked independently.
type MemorialRepository interface {
	MemorialReader
	MemorialWriter
}
