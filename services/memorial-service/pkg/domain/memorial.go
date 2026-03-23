package domain

import (
	"time"
)

// Memorial represents a person's memorial
type Memorial struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	DateOfBirth time.Time `json:"date_of_birth"`
	DateOfDeath time.Time `json:"date_of_death"`
	Biography   string    `json:"biography"`
	CreatedBy   string    `json:"created_by"`
	CreatedAt   time.Time `json:"created_at"`
	UpdatedAt   time.Time `json:"updated_at"`
	Status      string    `json:"status"` // active, inactive, archived
}

// MemorialMedia represents media associated with a memorial
type MemorialMedia struct {
	ID          string    `json:"id"`
	MemorialID  string    `json:"memorial_id"`
	URL         string    `json:"url"`
	MediaType   string    `json:"media_type"` // photo, video, document
	Description string    `json:"description"`
	CreatedAt   time.Time `json:"created_at"`
}

// MemorialFilter for querying memorials
type MemorialFilter struct {
	Limit  int
	Offset int
	Search string
	Status string
	Sort   string
}
