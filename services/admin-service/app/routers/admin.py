"""Admin router — POST /admin/update-person.

Request lifecycle
-----------------
1. JWT authentication gate (existing dependency).
2. Per-IP sliding-window rate limit check (Redis).
3. Pydantic validation of the request body.
4. gRPC ``UpdateMemorial`` call to the Go memorial-service.
5. Redis cache invalidation for ``memorial:v1:{id}``.
6. Kafka ``memorial.updated`` event publish via aiokafka.
7. Return the updated memorial to the caller.

Each step is independent in terms of error handling:
- Rate limit failure → 429
- Pydantic validation failure → 422 (handled automatically by FastAPI)
- gRPC NOT_FOUND / INVALID_ARGUMENT → 404 / 400
- gRPC other errors → 502 Bad Gateway
- Cache invalidation failure → logged, does NOT abort the response
- Kafka publish failure → logged, does NOT abort the response
"""

from __future__ import annotations

import logging
from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException, Request, status
from grpc import StatusCode

from app.core.dependencies import get_admin_user
from app.schemas.update_person import (
    UpdatePersonRequest,
    UpdatePersonResponse,
)
from app.services.cache_service import CacheService, get_cache_service
from app.services.grpc_client import GRPCError, MemorialGRPCClient, get_grpc_client
from app.services.kafka_service import KafkaEventService, get_kafka_service
from app.services.rate_limiter import (
    RateLimitExceeded,
    RateLimiter,
    build_rate_limit_key,
    get_rate_limiter,
)

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/admin", tags=["admin-v2"])


# ---------------------------------------------------------------------------
# Dependency aliases (improves readability in route signatures)
# ---------------------------------------------------------------------------
AdminUserDep = Annotated[dict, Depends(get_admin_user)]
GRPCClientDep = Annotated[MemorialGRPCClient, Depends(get_grpc_client)]
CacheServiceDep = Annotated[CacheService, Depends(get_cache_service)]
KafkaServiceDep = Annotated[KafkaEventService, Depends(get_kafka_service)]
RateLimiterDep = Annotated[RateLimiter, Depends(get_rate_limiter)]


# ---------------------------------------------------------------------------
# Endpoint
# ---------------------------------------------------------------------------

@router.post(
    "/update-person",
    response_model=UpdatePersonResponse,
    status_code=status.HTTP_200_OK,
    summary="Update a memorial person record",
    description=(
        "Validates the request, calls the Go gRPC memorial-service to persist "
        "the update, invalidates the Redis cache for the record, and publishes "
        "a `memorial.updated` event to Kafka."
    ),
    responses={
        400: {"description": "Invalid input (gRPC INVALID_ARGUMENT)"},
        401: {"description": "Not authenticated"},
        403: {"description": "Admin access required"},
        404: {"description": "Memorial not found"},
        429: {"description": "Rate limit exceeded"},
        502: {"description": "Upstream gRPC service error"},
    },
)
async def update_person(
    request: Request,
    body: UpdatePersonRequest,
    _admin: AdminUserDep,
    grpc_client: GRPCClientDep,
    cache_svc: CacheServiceDep,
    kafka_svc: KafkaServiceDep,
    rate_limiter: RateLimiterDep,
) -> UpdatePersonResponse:
    # ------------------------------------------------------------------
    # 1.  Rate limiting (checked AFTER auth to avoid enumeration)
    # ------------------------------------------------------------------
    rl_key = build_rate_limit_key("admin:update-person", request)
    try:
        await rate_limiter.check(rl_key)
    except RateLimitExceeded as exc:
        logger.warning(
            "Rate limit exceeded for %s (limit=%d, window=%ds)",
            exc.key, exc.limit, exc.window,
        )
        raise HTTPException(
            status_code=status.HTTP_429_TOO_MANY_REQUESTS,
            detail=(
                f"Too many requests. Max {exc.limit} requests "
                f"per {exc.window} seconds."
            ),
            headers={"Retry-After": str(exc.window)},
        )

    # ------------------------------------------------------------------
    # 2.  gRPC call → Go memorial-service
    # ------------------------------------------------------------------
    logger.info(
        "Admin %s updating memorial id=%s",
        _admin.get("sub", "unknown"),
        body.id,
    )
    try:
        memorial_data = await grpc_client.update_memorial(body)
    except GRPCError as exc:
        if exc.code == StatusCode.NOT_FOUND:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Memorial not found: {body.id}",
            )
        if exc.code == StatusCode.INVALID_ARGUMENT:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail=exc.details,
            )
        logger.error(
            "gRPC UpdateMemorial failed for id=%s: [%s] %s",
            body.id, exc.code.name, exc.details,
        )
        raise HTTPException(
            status_code=status.HTTP_502_BAD_GATEWAY,
            detail="Upstream memorial service error. Please try again later.",
        )

    # ------------------------------------------------------------------
    # 3.  Redis cache invalidation (best-effort, non-fatal)
    # ------------------------------------------------------------------
    await cache_svc.invalidate_memorial(body.id)

    # ------------------------------------------------------------------
    # 4.  Kafka event publish (best-effort, non-fatal)
    # ------------------------------------------------------------------
    try:
        await kafka_svc.publish_memorial_updated(
            memorial_id=body.id,
            updated_data={
                "name": memorial_data.name,
                "biography": memorial_data.biography,
                "status": memorial_data.status,
                "updated_at": memorial_data.updated_at,
            },
        )
    except Exception as exc:  # noqa: BLE001
        logger.error(
            "Failed to publish Kafka event for memorial id=%s: %s",
            body.id, exc,
        )
        # Continue — event publish failure must NOT block the API response.

    # ------------------------------------------------------------------
    # 5.  Respond
    # ------------------------------------------------------------------
    logger.info("Successfully updated memorial id=%s", body.id)
    return UpdatePersonResponse(memorial=memorial_data)
