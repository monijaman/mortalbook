"""Pydantic schemas for the POST /admin/update-person endpoint."""

from __future__ import annotations

from typing import Literal, Optional
from pydantic import BaseModel, Field, model_validator


class UpdatePersonRequest(BaseModel):
    """Validated request body for updating a memorial person record.

    All fields except ``id`` are optional; at least one non-id field must be
    present so the request is not a no-op.
    """

    id: str = Field(..., min_length=1, description="Memorial record UUID")
    name: Optional[str] = Field(
        None,
        min_length=1,
        max_length=256,
        description="Full display name",
    )
    biography: Optional[str] = Field(
        None,
        max_length=10_000,
        description="Biography text",
    )
    status: Optional[Literal["active", "archived"]] = Field(
        None,
        description="Visibility status",
    )

    @model_validator(mode="after")
    def at_least_one_update_field(self) -> "UpdatePersonRequest":
        if not any([self.name, self.biography, self.status]):
            raise ValueError(
                "At least one of name, biography, or status must be provided."
            )
        return self


class MemorialData(BaseModel):
    """Read-side representation of a Memorial returned from the gRPC service."""

    id: str
    name: str
    date_of_birth: str
    date_of_death: str
    biography: str
    created_by: str
    status: str
    created_at: int
    updated_at: int


class UpdatePersonResponse(BaseModel):
    """Response returned to the API caller after a successful update."""

    success: bool = True
    memorial: MemorialData
    message: str = "Memorial updated successfully"


class ErrorResponse(BaseModel):
    """Generic error response shape."""

    success: bool = False
    error: str
    detail: Optional[str] = None
