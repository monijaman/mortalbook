"""FastAPI Admin Service Main Entry Point"""

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from contextlib import asynccontextmanager

from app.core.config import settings
from app.core.logger import get_logger
from app.api.routes import auth, admin, users, memorials, analytics
from app.routers import admin as admin_v2
from app.db.database import init_db, get_db
from app.core import dependencies

# Service lifecycle helpers
from app.services.grpc_client import startup_grpc_client, shutdown_grpc_client
from app.services.cache_service import startup_cache_service, shutdown_cache_service
from app.services.kafka_service import startup_kafka_service, shutdown_kafka_service
from app.services.rate_limiter import startup_rate_limiter, shutdown_rate_limiter

logger = get_logger(__name__)

@asynccontextmanager
async def lifespan(app: FastAPI):
    # ------------------------------------------------------------------ startup
    logger.info("Starting Admin Service...")

    await init_db()
    logger.info("✓ Database initialized")

    # gRPC channel to Go memorial-service (synchronous connect, fast)
    try:
        startup_grpc_client()
        logger.info("✓ gRPC client ready (%s)", settings.MEMORIAL_GRPC_ADDR)
    except Exception as e:
        logger.warning("gRPC client startup failed (optional): %s", e)

    # Shared Redis connection for cache invalidation
    await startup_cache_service()
    logger.info("✓ Cache service ready")

    # Shared Redis connection for rate limiting
    await startup_rate_limiter()
    logger.info("✓ Rate limiter ready")

    # aiokafka producer
    try:
        await startup_kafka_service()
        logger.info("✓ Kafka producer ready (brokers: %s)", settings.KAFKA_BROKERS)
    except Exception as e:
        logger.warning("Kafka startup failed (optional in dev): %s", e)

    yield

    # ----------------------------------------------------------------- shutdown
    logger.info("Shutting down Admin Service...")
    await shutdown_kafka_service()
    await shutdown_rate_limiter()
    await shutdown_cache_service()
    shutdown_grpc_client()
    logger.info("Admin Service stopped cleanly.")

app = FastAPI(
    title="Mortalbook Admin Service",
    description="Admin backend service for managing memorials",
    version="1.0.0",
    lifespan=lifespan,
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure in production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(auth.router, prefix="/api/v1/auth", tags=["auth"])
app.include_router(admin.router, prefix="/api/v1/admin", tags=["admin"])
app.include_router(users.router, prefix="/api/v1/users", tags=["users"])
app.include_router(memorials.router, prefix="/api/v1/memorials", tags=["memorials"])
app.include_router(analytics.router, prefix="/api/v1/analytics", tags=["analytics"])

# Clean-architecture admin router (gRPC + Redis + Kafka)
app.include_router(admin_v2.router, prefix="/api/v2", tags=["admin-v2"])

# Health check endpoint
@app.get("/health")
async def health_check():
    return {
        "status": "ok",
        "service": "admin-service",
        "environment": settings.ENV,
    }

@app.get("/")
async def root():
    return {
        "name": "Mortalbook Admin Service",
        "version": "1.0.0",
        "docs": "/docs",
    }

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        app,
        host="0.0.0.0",
        port=5002,
        reload=settings.DEBUG,
    )
