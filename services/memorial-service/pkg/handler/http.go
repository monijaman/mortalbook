package handler

import (
	"encoding/json"
	"net/http"
	"time"

	"go.uber.org/zap"

	"github.com/yourusername/mortalbook/memorial-service/pkg/domain"
	"github.com/yourusername/mortalbook/memorial-service/pkg/service"
)

type Handlers struct {
	memorialService service.MemorialService
	logger          *zap.SugaredLogger
}

func NewHandlers(memorialService service.MemorialService, logger *zap.SugaredLogger) *Handlers {
	return &Handlers{
		memorialService: memorialService,
		logger:          logger,
	}
}

// ListMemorials GET /api/v1/memorials
func (h *Handlers) ListMemorials(w http.ResponseWriter, r *http.Request) {
	filter := &domain.MemorialFilter{
		Limit:  20,
		Offset: 0,
	}

	memorials, total, err := h.memorialService.ListMemorials(r.Context(), filter)
	if err != nil {
		h.error(w, http.StatusInternalServerError, "Failed to list memorials")
		return
	}

	h.json(w, http.StatusOK, map[string]interface{}{
		"data":  memorials,
		"total": total,
	})
}

// GetMemorial GET /api/v1/memorials/{id}
func (h *Handlers) GetMemorial(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")

	memorial, err := h.memorialService.GetMemorial(r.Context(), id)
	if err != nil {
		h.error(w, http.StatusNotFound, "Memorial not found")
		return
	}

	h.json(w, http.StatusOK, memorial)
}

// CreateMemorial POST /api/v1/memorials
func (h *Handlers) CreateMemorial(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Name        string `json:"name"`
		Biography   string `json:"biography"`
		DateOfBirth string `json:"date_of_birth"`
		DateOfDeath string `json:"date_of_death"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		h.error(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	dob, _ := time.Parse("2006-01-02", req.DateOfBirth)
	dod, _ := time.Parse("2006-01-02", req.DateOfDeath)

	memorial := &domain.Memorial{
		Name:        req.Name,
		Biography:   req.Biography,
		DateOfBirth: dob,
		DateOfDeath: dod,
		CreatedBy:   r.Header.Get("X-User-ID"),
		Status:      "active",
	}

	id, err := h.memorialService.CreateMemorial(r.Context(), memorial)
	if err != nil {
		h.error(w, http.StatusInternalServerError, "Failed to create memorial")
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(map[string]string{"id": id})
}

// UpdateMemorial PUT /api/v1/memorials/{id}
func (h *Handlers) UpdateMemorial(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")

	var req struct {
		Name      string `json:"name"`
		Biography string `json:"biography"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		h.error(w, http.StatusBadRequest, "Invalid request body")
		return
	}

	memorial := &domain.Memorial{
		ID:        id,
		Name:      req.Name,
		Biography: req.Biography,
	}

	err := h.memorialService.UpdateMemorial(r.Context(), memorial)
	if err != nil {
		h.error(w, http.StatusInternalServerError, "Failed to update memorial")
		return
	}

	h.json(w, http.StatusOK, map[string]string{"message": "Memorial updated"})
}

// DeleteMemorial DELETE /api/v1/memorials/{id}
func (h *Handlers) DeleteMemorial(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")

	err := h.memorialService.DeleteMemorial(r.Context(), id)
	if err != nil {
		h.error(w, http.StatusInternalServerError, "Failed to delete memorial")
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

// GetMemorialMedia GET /api/v1/memorials/{id}/media
func (h *Handlers) GetMemorialMedia(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")

	media, err := h.memorialService.GetMemorialMedia(r.Context(), id)
	if err != nil {
		h.error(w, http.StatusInternalServerError, "Failed to get media")
		return
	}

	h.json(w, http.StatusOK, media)
}

// Health GET /health
func (h *Handlers) Health(w http.ResponseWriter, r *http.Request) {
	h.json(w, http.StatusOK, map[string]string{"status": "ok"})
}

// Helper methods
func (h *Handlers) json(w http.ResponseWriter, code int, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(code)
	json.NewEncoder(w).Encode(data)
}

func (h *Handlers) error(w http.ResponseWriter, code int, message string) {
	h.json(w, code, map[string]string{"error": message})
}
