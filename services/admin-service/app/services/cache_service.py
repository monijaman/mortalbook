"""Redis cache service — invalidates cached memorial entries.

The Go memorial-service caches individual memorials under the key
``memorial:v1:{id}`` with a 5-minute TTL.  When an admin updates a memorial
we must delete that key so the next read goes to PostgreSQL and repopulates
the cache with fresh data.
"""

from __future__ import annotations

import logging
from typing import Optional

import redis.asyncio as aioredis

from app.core.config import settings

logger = logging.getLogger(__name__)

# Key pattern must match `memorial-service/internal/repository/memorial_repository.go`
_CACHE_KEY_PREFIX = "memorial:v1"


class CacheService:
    """Async Redis client wrapper for cache invalidation operations."""

    def __init__(self, client: aioredis.Redis) -> None:
        self._redis = client

    # ------------------------------------------------------------------
    # Factory helpers
    # ------------------------------------------------------------------

    @classmethod
    def create(cls) -> "CacheService":
        """Build a service backed by the configured Redis URL."""
        logger.info("Connecting to Redis @ %s", settings.REDIS_URL)
        client = aioredis.from_url(
            settings.REDIS_URL,
            encoding="utf-8",
            decode_responses=True,
        )
        return cls(client)

    async def close(self) -> None:
        """Close the underlying aioredis connection pool."""
        await self._redis.aclose()

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    def _memorial_key(self, memorial_id: str) -> str:
        return f"{_CACHE_KEY_PREFIX}:{memorial_id}"

    async def invalidate_memorial(self, memorial_id: str) -> bool:
        """Delete the cached memorial entry.

        Returns ``True`` if a key was actually deleted, ``False`` if the key
        was not present (cache-miss — safe to ignore).
        """
        key = self._memorial_key(memorial_id)
        try:
            deleted: int = await self._redis.delete(key)
            if deleted:
                logger.debug("Cache invalidated: %s", key)
            else:
                logger.debug("Cache miss on invalidation (key not found): %s", key)
            return bool(deleted)
        except Exception as exc:
            # Cache failure must NOT fail the write operation; log and continue.
            logger.error("Failed to invalidate cache key %s: %s", key, exc)
            return False

    async def ping(self) -> bool:
        """Return True if Redis is reachable."""
        try:
            return await self._redis.ping()
        except Exception:
            return False


# ---------------------------------------------------------------------------
# Module-level singleton
# ---------------------------------------------------------------------------
_service: Optional[CacheService] = None


def get_cache_service() -> CacheService:
    """FastAPI dependency: returns the shared :class:`CacheService`."""
    if _service is None:
        raise RuntimeError(
            "CacheService has not been initialised. "
            "Ensure startup_cache_service() was called in the app lifespan."
        )
    return _service


async def startup_cache_service() -> None:
    global _service
    _service = CacheService.create()


async def shutdown_cache_service() -> None:
    global _service
    if _service is not None:
        await _service.close()
        _service = None
