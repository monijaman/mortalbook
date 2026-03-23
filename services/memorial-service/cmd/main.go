package main

import (
	"context"
	"net"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/joho/godotenv"
	"go.uber.org/zap"
	"google.golang.org/grpc"
	"google.golang.org/grpc/reflection"

	pb "github.com/yourusername/mortalbook/memorial-service/gen/memorial/v1"
	grpchandler "github.com/yourusername/mortalbook/memorial-service/internal/handler"
	memrepo "github.com/yourusername/mortalbook/memorial-service/internal/repository"
	"github.com/yourusername/mortalbook/memorial-service/internal/usecase"
	"github.com/yourusername/mortalbook/memorial-service/pkg/db"
	httphandler "github.com/yourusername/mortalbook/memorial-service/pkg/handler"
	"github.com/yourusername/mortalbook/memorial-service/pkg/repository"
	"github.com/yourusername/mortalbook/memorial-service/pkg/service"
)

func main() {
	// ── Environment & Logger ──────────────────────────────────────────────────
	godotenv.Load()

	logger, _ := zap.NewProduction()
	defer logger.Sync()
	sugar := logger.Sugar()

	// ── Database config ───────────────────────────────────────────────────────
	pgConfig := &db.Config{
		Host:     getEnv("DB_HOST", "localhost"),
		Port:     getEnv("DB_PORT", "5432"),
		Database: getEnv("DB_NAME", "mortalbook_db"),
		User:     getEnv("DB_USER", "postgres"),
		Password: getEnv("DB_PASSWORD", "postgres"),
	}
	redisConfig := &db.RedisConfig{
		Host: getEnv("REDIS_HOST", "localhost"),
		Port: getEnv("REDIS_PORT", "6379"),
	}
	kafkaConfig := &db.KafkaConfig{
		Brokers: getEnv("KAFKA_BROKERS", "kafka:9092"),
	}

	// ── Connections ───────────────────────────────────────────────────────────
	pgDB, err := db.NewPostgresConnection(pgConfig)
	if err != nil {
		sugar.Fatalf("PostgreSQL: %v", err)
	}
	defer pgDB.Close()
	sugar.Info("✓ PostgreSQL connected")

	redisClient, err := db.NewRedisConnection(redisConfig)
	if err != nil {
		sugar.Fatalf("Redis: %v", err)
	}
	defer redisClient.Close()
	sugar.Info("✓ Redis connected")

	kafkaConn, err := db.NewKafkaConnection(kafkaConfig)
	if err != nil {
		sugar.Warnf("Kafka connection failed (optional): %v", err)
		kafkaConn = nil
	} else {
		defer kafkaConn.Close()
		sugar.Info("✓ Kafka connected")
	}

	// ── Dependency injection (Clean Architecture) ─────────────────────────────
	//
	//   handler  →  usecase.MemorialUseCase  (interface)
	//   usecase  →  domain.MemorialRepository (interface)
	//   repo     →  *sql.DB + *redis.Client   (infrastructure)
	//
	memorialRepo := memrepo.New(pgDB, redisClient, sugar)
	memorialUC := usecase.New(memorialRepo, sugar)
	grpcSrv := grpchandler.NewGRPCServer(memorialUC, sugar)

	// ── Legacy REST layer (existing pkg/ code — kept intact) ──────────────────
	legacyMemorialRepo := repository.NewMemorialRepository(pgDB, redisClient)
	legacyMediaRepo := repository.NewMediaRepository(pgDB)
	legacySvc := service.NewMemorialService(legacyMemorialRepo, legacyMediaRepo, kafkaConn, sugar)
	restHandlers := httphandler.NewHandlers(legacySvc, sugar)

	// ── gRPC server ───────────────────────────────────────────────────────────
	grpcPort := getEnv("GRPC_PORT", "50001")
	go startGRPCServer(grpcSrv, grpcPort, sugar)

	// ── REST server ───────────────────────────────────────────────────────────
	restPort := getEnv("PORT", "5001")
	mux := http.NewServeMux()
	registerRESTRoutes(mux, restHandlers)

	httpServer := &http.Server{
		Addr:         ":" + restPort,
		Handler:      corsMiddleware(mux),
		ReadTimeout:  15 * time.Second,
		WriteTimeout: 15 * time.Second,
		IdleTimeout:  60 * time.Second,
	}

	go func() {
		sugar.Infof("🚀 REST server starting on :%s", restPort)
		if err := httpServer.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			sugar.Fatalf("REST server: %v", err)
		}
	}()

	// ── Graceful shutdown ─────────────────────────────────────────────────────
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	sugar.Info("Shutting down gracefully…")
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	if err := httpServer.Shutdown(ctx); err != nil {
		sugar.Errorw("REST shutdown error", "error", err)
	}
	sugar.Info("✓ Shutdown complete")
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

func startGRPCServer(srv *grpchandler.GRPCServer, port string, sugar *zap.SugaredLogger) {
	lis, err := net.Listen("tcp", ":"+port)
	if err != nil {
		sugar.Fatalf("gRPC listen: %v", err)
	}

	s := grpc.NewServer()
	pb.RegisterMemorialServiceServer(s, srv)
	reflection.Register(s) // enables grpcurl / grpc-gateway discovery

	sugar.Infof("🚀 gRPC server starting on :%s", port)
	if err = s.Serve(lis); err != nil {
		sugar.Fatalf("gRPC serve: %v", err)
	}
}

func registerRESTRoutes(mux *http.ServeMux, h *httphandler.Handlers) {
	mux.HandleFunc("GET /api/v1/memorials", h.ListMemorials)
	mux.HandleFunc("GET /api/v1/memorials/{id}", h.GetMemorial)
	mux.HandleFunc("POST /api/v1/memorials", h.CreateMemorial)
	mux.HandleFunc("PUT /api/v1/memorials/{id}", h.UpdateMemorial)
	mux.HandleFunc("DELETE /api/v1/memorials/{id}", h.DeleteMemorial)
	mux.HandleFunc("GET /api/v1/memorials/{id}/media", h.GetMemorialMedia)
	mux.HandleFunc("GET /health", h.Health)
}

func corsMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")
		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusOK)
			return
		}
		next.ServeHTTP(w, r)
	})
}

func getEnv(key, def string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return def
}
