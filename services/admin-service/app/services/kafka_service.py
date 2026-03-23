"""Kafka event publishing via aiokafka.

Publishes ``memorial_updated`` domain events to the ``memorial-events`` topic
whenever a memorial record is successfully updated through the admin service.

Event envelope follows a simple CloudEvents-inspired schema:
    {
        "specversion": "1.0",
        "type": "memorial.updated",
        "source": "admin-service",
        "id": "<uuid>",
        "time": "<ISO 8601>",
        "data": { ... }
    }
"""

from __future__ import annotations

import json
import logging
import uuid
from datetime import datetime, timezone
from typing import Any, Dict, Optional

from aiokafka import AIOKafkaProducer
from aiokafka.errors import KafkaError

from app.core.config import settings

logger = logging.getLogger(__name__)


class KafkaEventService:
    """Async Kafka producer wrapper.

    The producer is started during FastAPI startup and stopped at shutdown.
    """

    def __init__(self, producer: AIOKafkaProducer) -> None:
        self._producer = producer

    # ------------------------------------------------------------------
    # Factory helpers
    # ------------------------------------------------------------------

    @classmethod
    def create(cls) -> "KafkaEventService":
        """Create an :class:`AIOKafkaProducer` — caller must await :meth:`start`."""
        logger.info("Creating Kafka producer for brokers: %s", settings.KAFKA_BROKERS)
        producer = AIOKafkaProducer(
            bootstrap_servers=settings.KAFKA_BROKERS,
            value_serializer=lambda v: json.dumps(v).encode("utf-8"),
            key_serializer=lambda k: k.encode("utf-8") if k else None,
            # Durability: wait for leader + one in-sync replica
            acks="all",
            enable_idempotence=True,
            # Compression reduces bandwidth for JSON payloads
            compression_type="gzip",
            # retry_backoff controls delay between reconnect attempts
            retry_backoff_ms=200,
        )
        return cls(producer)

    async def start(self) -> None:
        await self._producer.start()
        logger.info("Kafka producer started")

    async def stop(self) -> None:
        await self._producer.stop()
        logger.info("Kafka producer stopped")

    # ------------------------------------------------------------------
    # Event builders
    # ------------------------------------------------------------------

    def _build_envelope(
        self,
        event_type: str,
        data: Dict[str, Any],
    ) -> Dict[str, Any]:
        return {
            "specversion": "1.0",
            "type": event_type,
            "source": "admin-service",
            "id": str(uuid.uuid4()),
            "time": datetime.now(tz=timezone.utc).isoformat(),
            "data": data,
        }

    # ------------------------------------------------------------------
    # Public API
    # ------------------------------------------------------------------

    async def publish_memorial_updated(
        self,
        memorial_id: str,
        updated_data: Dict[str, Any],
    ) -> None:
        """Send a ``memorial.updated`` event to the configured topic.

        The message key is the memorial UUID so that all events for the same
        memorial are routed to the same Kafka partition (ordering guarantee).

        Args:
            memorial_id: UUID of the updated memorial (used as message key).
            updated_data: Snapshot of updated fields to embed in the event.

        Raises:
            KafkaError: on unrecoverable publish failure (logged + re-raised).
        """
        topic = settings.KAFKA_MEMORIAL_TOPIC
        envelope = self._build_envelope(
            "memorial.updated",
            {"memorial_id": memorial_id, **updated_data},
        )
        try:
            record_metadata = await self._producer.send_and_wait(
                topic,
                value=envelope,
                key=memorial_id,
            )
            logger.debug(
                "Published memorial.updated [id=%s] → %s partition=%d offset=%d",
                memorial_id,
                topic,
                record_metadata.partition,
                record_metadata.offset,
            )
        except KafkaError as exc:
            logger.error(
                "Failed to publish memorial.updated event [id=%s]: %s",
                memorial_id,
                exc,
            )
            raise


# ---------------------------------------------------------------------------
# Module-level singleton
# ---------------------------------------------------------------------------
_service: Optional[KafkaEventService] = None


def get_kafka_service() -> KafkaEventService:
    """FastAPI dependency: returns the shared :class:`KafkaEventService`."""
    if _service is None:
        raise RuntimeError(
            "KafkaEventService has not been initialised. "
            "Ensure startup_kafka_service() was called in the app lifespan."
        )
    return _service


async def startup_kafka_service() -> None:
    global _service
    _service = KafkaEventService.create()
    await _service.start()


async def shutdown_kafka_service() -> None:
    global _service
    if _service is not None:
        await _service.stop()
        _service = None
