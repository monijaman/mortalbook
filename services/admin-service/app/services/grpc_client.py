"""Async gRPC client for the Go memorial-service.

Manages a single shared channel (created at startup, closed at shutdown) and
exposes an async wrapper around the blocking ``UpdateMemorial`` RPC using
``asyncio.get_event_loop().run_in_executor``.
"""

from __future__ import annotations

import asyncio
import logging
from typing import Optional

import grpc
from grpc import StatusCode

from app.core.config import settings
from app.proto import memorial_pb2 as pb2
from app.proto import memorial_pb2_grpc as pb2_grpc
from app.schemas.update_person import MemorialData, UpdatePersonRequest

logger = logging.getLogger(__name__)

# Maximum number of connection retries with exponential backoff
_MAX_RETRIES = 3


class GRPCError(Exception):
    """Raised when the upstream gRPC call fails."""

    def __init__(self, code: StatusCode, details: str) -> None:
        self.code = code
        self.details = details
        super().__init__(f"gRPC error [{code.name}]: {details}")


class MemorialGRPCClient:
    """Thread-safe async wrapper around the MemorialServiceStub.

    Single instance expected; create via :meth:`create` and close via
    :meth:`close` inside the FastAPI lifespan.
    """

    def __init__(self, channel: grpc.Channel) -> None:
        self._channel = channel
        self._stub = pb2_grpc.MemorialServiceStub(channel)

    # ------------------------------------------------------------------
    # Factory helpers
    # ------------------------------------------------------------------

    @classmethod
    def create(cls) -> "MemorialGRPCClient":
        """Open an insecure channel to the configured memorial-service address."""
        addr = settings.MEMORIAL_GRPC_ADDR
        logger.info("Opening gRPC channel to memorial-service @ %s", addr)
        channel = grpc.insecure_channel(
            addr,
            options=[
                ("grpc.keepalive_time_ms", 10_000),
                ("grpc.keepalive_timeout_ms", 5_000),
                ("grpc.keepalive_permit_without_calls", True),
                ("grpc.http2.max_pings_without_data", 0),
            ],
        )
        return cls(channel)

    def close(self) -> None:
        """Drain and close the underlying channel."""
        logger.info("Closing gRPC channel to memorial-service")
        self._channel.close()

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    async def update_memorial(self, req: UpdatePersonRequest) -> MemorialData:
        """Call ``UpdateMemorial`` RPC on the Go service.

        Runs the blocking stub call in a thread-pool executor so it doesn't
        block the asyncio event loop.

        Raises:
            GRPCError: if the RPC returns a non-OK status.
        """
        grpc_request = pb2.UpdateMemorialRequest(
            id=req.id,
            name=req.name or "",
            biography=req.biography or "",
            status=req.status or "",
        )

        loop = asyncio.get_event_loop()

        for attempt in range(1, _MAX_RETRIES + 1):
            try:
                grpc_response: pb2.UpdateMemorialResponse = await loop.run_in_executor(
                    None,
                    lambda: self._stub.UpdateMemorial(  # noqa: B023
                        grpc_request, timeout=10
                    ),
                )
                m = grpc_response.memorial
                return MemorialData(
                    id=m.id,
                    name=m.name,
                    date_of_birth=m.date_of_birth,
                    date_of_death=m.date_of_death,
                    biography=m.biography,
                    created_by=m.created_by,
                    status=m.status,
                    created_at=m.created_at,
                    updated_at=m.updated_at,
                )
            except grpc.RpcError as exc:
                code: StatusCode = exc.code()  # type: ignore[attr-defined]
                details: str = exc.details()   # type: ignore[attr-defined]
                logger.warning(
                    "gRPC UpdateMemorial attempt %d/%d failed: [%s] %s",
                    attempt,
                    _MAX_RETRIES,
                    code.name,
                    details,
                )
                # Non-retryable statuses — propagate immediately
                if code in (
                    StatusCode.NOT_FOUND,
                    StatusCode.INVALID_ARGUMENT,
                    StatusCode.PERMISSION_DENIED,
                    StatusCode.UNAUTHENTICATED,
                ):
                    raise GRPCError(code, details) from exc

                if attempt == _MAX_RETRIES:
                    raise GRPCError(code, details) from exc

                await asyncio.sleep(0.2 * attempt)  # simple backoff

        # Unreachable — but makes the type-checker happy
        raise GRPCError(StatusCode.UNKNOWN, "Exhausted retries")


# ---------------------------------------------------------------------------
# Module-level singleton — injected by the FastAPI lifespan
# ---------------------------------------------------------------------------
_client: Optional[MemorialGRPCClient] = None


def get_grpc_client() -> MemorialGRPCClient:
    """FastAPI dependency: returns the shared :class:`MemorialGRPCClient`."""
    if _client is None:
        raise RuntimeError(
            "MemorialGRPCClient has not been initialised. "
            "Ensure startup_grpc_client() was called in the app lifespan."
        )
    return _client


def startup_grpc_client() -> None:
    """Called during FastAPI startup to create the shared channel."""
    global _client
    _client = MemorialGRPCClient.create()


def shutdown_grpc_client() -> None:
    """Called during FastAPI shutdown to close the channel gracefully."""
    global _client
    if _client is not None:
        _client.close()
        _client = None
