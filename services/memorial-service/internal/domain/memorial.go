// Package domain contains pure business entities — no framework, no I/O.
// Follows the Dependency Inversion Principle: all outer layers depend inward
// on these types, never the other way around.
package domain

import "time"

// ─── Entities ────────────────────────────────────────────────────────────────

// Memorial is the core aggregate root.
type Memorial struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	DateOfBirth time.Time `json:"date_of_birth"`
	DateOfDeath time.Time `json:"date_of_death"`
	Biography   string    `json:"biography"`
	CreatedBy   string    `json:"created_by"`
	Status      string    `json:"status"` // active | archived
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// MemorialMedia is a value-object attached to a Memorial.
type MemorialMedia struct {
	ID          string
	MemorialID  string
	URL         string
	MediaType   string // photo | video | document
	Description string
	CreatedAt   time.Time
}

// ─── Value Objects / Filter ───────────────────────────────────────────────────

// MemorialFilter carries pagination and query parameters.
type MemorialFilter struct {
	Limit  int
	Offset int
	Search string
	Status string
	Sort   string
}

// UpdateMemorialParams carries only the mutable fields that a caller is
// allowed to change.  Pointer fields express "set if non-nil" semantics
// so callers never accidentally zero-out a field they didn't touch.
type UpdateMemorialParams struct {
	ID        string
	Name      *string
	Biography *string
	Status    *string
}
