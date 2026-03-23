// Package domain – sentinel errors used across all layers.
package domain

import "errors"

// ErrNotFound is returned by the repository when an entity doesn't exist.
var ErrNotFound = errors.New("memorial: not found")

// ErrInvalidInput is returned when caller-supplied data fails validation.
var ErrInvalidInput = errors.New("memorial: invalid input")
