"""Redis-backed sliding-window rate limiter.

Algorithm
---------
For each unique key (typically ``f"rate:{endpoint}:{client_ip}"``):

1. Remove all timestamps older than ``window_seconds`` from the sorted set.
2. Count the remaining members.
3. If the count >= ``max_requests`` → raise :class:`RateLimitExceeded`.
4. Otherwise, add the current timestamp with score = timestamp.
5. Set the TTL on the key to ``window_seconds`` + 1 so stale keys expire.

This gives a true sliding window (not a fixed-period bucket) and is safe for
concurrent requests because all steps are executed inside a single MULTI/EXEC
Redis transaction.
"""

from __future__ import annotations

import logging
import time
from typing import Optional

import redis.asyncio as aioredis
from fastapi import Request

from app.core.config import settings

logger = logging.getLogger(__name__)


class RateLimitExceeded(Exception):
    """Raised when the caller has exceeded the allowed request rate."""

    def __init__(self, key: str, limit: int, window: int) -> None:
        self.key = key
        self.limit = limit
        self.window = window
        super().__init__(
            f"Rate limit exceeded for {key}: max {limit} requests per {window}s"
        )


class RateLimiter:
    """Sliding-window rate limiter backed by a Redis sorted set."""

    def __init__(self, client: aioredis.Redis) -> None:
        self._redis = client

    # ------------------------------------------------------------------
    # Factory helpers
    # ------------------------------------------------------------------

    @classmethod
    def create(cls) -> "RateLimiter":
        client = aioredis.from_url(
            settings.REDIS_URL,
            encoding="utf-8",
            decode_responses=True,
        )
        return cls(client)

    async def close(self) -> None:
        await self._redis.aclose()

    # ------------------------------------------------------------------
    # Core logic
    # ------------------------------------------------------------------

    async def check(
        self,
        key: str,
        max_requests: Optional[int] = None,
        window_seconds: Optional[int] = None,
    ) -> None:
        """Check and record a request for ``key``.

        Args:
            key: Unique identifier for the rate-limit bucket (e.g. IP + endpoint).
            max_requests: Override the default from settings.
            window_seconds: Override the default from settings.

        Raises:
            RateLimitExceeded: if the bucket is full.
        """
        limit = max_requests if max_requests is not None else settings.RATE_LIMIT_REQUESTS
        window = window_seconds if window_seconds is not None else settings.RATE_LIMIT_WINDOW_SECONDS

        now = time.time()
        window_start = now - window

        async with self._redis.pipeline(transaction=True) as pipe:
            try:
                (
                    pipe
                    .zremrangebyscore(key, "-inf", window_start)
                    .zcard(key)
                    .zadd(key, {str(now): now})
                    .expire(key, window + 1)
                )
                _, current_count, *_ = await pipe.execute()
            except Exception as exc:
                # Redis failure must NOT block the request — log and pass through.
                logger.error("Rate limiter Redis error for key %s: %s", key, exc)
                return

        if current_count >= limit:
            raise RateLimitExceeded(key, limit, window)


def _client_ip(request: Request) -> str:
    """Extract the real client IP, respecting X-Forwarded-For."""
    forwarded_for = request.headers.get("X-Forwarded-For")
    if forwarded_for:
        return forwarded_for.split(",")[0].strip()
    if request.client:
        return request.client.host
    return "unknown"


def build_rate_limit_key(endpoint: str, request: Request) -> str:
    """Build a per-IP-per-endpoint key for the rate limiter."""
    ip = _client_ip(request)
    return f"rate:{endpoint}:{ip}"


# ---------------------------------------------------------------------------
# Module-level singleton
# ---------------------------------------------------------------------------
_limiter: Optional[RateLimiter] = None


def get_rate_limiter() -> RateLimiter:
    """FastAPI dependency: returns the shared :class:`RateLimiter`."""
    if _limiter is None:
        raise RuntimeError(
            "RateLimiter has not been initialised. "
            "Ensure startup_rate_limiter() was called in the app lifespan."
        )
    return _limiter


async def startup_rate_limiter() -> None:
    global _limiter
    _limiter = RateLimiter.create()


async def shutdown_rate_limiter() -> None:
    global _limiter
    if _limiter is not None:
        await _limiter.close()
        _limiter = None
