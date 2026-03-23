"""Memorial management API routes — full CRUD"""

from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from pydantic import BaseModel
from typing import Optional
from uuid import uuid4
from datetime import datetime

from app.db.database import get_db, Memorial
from app.core.dependencies import get_admin_user
from app.core.logger import get_logger

router = APIRouter()
logger = get_logger(__name__)


# ── Schemas ──────────────────────────────────────────────────────────────────

class CreateMemorialRequest(BaseModel):
    name: str
    date_of_birth: Optional[str] = None
    date_of_death: Optional[str] = None
    biography: Optional[str] = None
    status: Optional[str] = "active"


class PatchMemorialRequest(BaseModel):
    name: Optional[str] = None
    date_of_birth: Optional[str] = None
    date_of_death: Optional[str] = None
    biography: Optional[str] = None
    status: Optional[str] = None


class StatusBody(BaseModel):
    status: str


# ── Helpers ───────────────────────────────────────────────────────────────────

def _to_dict(m: Memorial) -> dict:
    return {
        "id": m.id,
        "name": m.name,
        "date_of_birth": m.date_of_birth,
        "date_of_death": m.date_of_death,
        "biography": m.biography,
        "created_by": m.created_by,
        "status": m.status,
        "created_at": m.created_at.isoformat() if m.created_at else None,
        "updated_at": m.updated_at.isoformat() if m.updated_at else None,
    }


def _get_or_404(db: Session, memorial_id: str) -> Memorial:
    m = db.query(Memorial).filter(
        Memorial.id == memorial_id,
        Memorial.deleted_at == None,  # noqa: E711
    ).first()
    if not m:
        raise HTTPException(status_code=404, detail="Memorial not found")
    return m


# ── Routes ────────────────────────────────────────────────────────────────────

@router.get("/")
async def list_memorials(
    skip: int = 0,
    limit: int = 20,
    search: str = "",
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    q = db.query(Memorial).filter(Memorial.deleted_at == None)  # noqa: E711
    if search.strip():
        q = q.filter(Memorial.name.ilike(f"%{search.strip()}%"))
    total = q.count()
    items = q.order_by(Memorial.created_at.desc()).offset(skip).limit(limit).all()
    return {"data": [_to_dict(m) for m in items], "total": total, "skip": skip, "limit": limit}


@router.post("/", status_code=201)
async def create_memorial(
    request: CreateMemorialRequest,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    m = Memorial(
        id=str(uuid4()),
        name=request.name,
        date_of_birth=request.date_of_birth,
        date_of_death=request.date_of_death,
        biography=request.biography,
        status=request.status or "active",
        created_by=current_user.get("sub"),
    )
    db.add(m)
    db.commit()
    db.refresh(m)
    logger.info("Memorial created: %s by %s", m.id, current_user.get("email"))
    return _to_dict(m)


@router.get("/{memorial_id}")
async def get_memorial(
    memorial_id: str,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    return _to_dict(_get_or_404(db, memorial_id))


@router.patch("/{memorial_id}")
async def update_memorial(
    memorial_id: str,
    request: PatchMemorialRequest,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    m = _get_or_404(db, memorial_id)
    if request.name is not None:
        m.name = request.name
    if request.date_of_birth is not None:
        m.date_of_birth = request.date_of_birth
    if request.date_of_death is not None:
        m.date_of_death = request.date_of_death
    if request.biography is not None:
        m.biography = request.biography
    if request.status is not None:
        m.status = request.status
    m.updated_at = datetime.utcnow()
    db.commit()
    db.refresh(m)
    logger.info("Memorial updated: %s by %s", memorial_id, current_user.get("email"))
    return _to_dict(m)


@router.delete("/{memorial_id}")
async def delete_memorial(
    memorial_id: str,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    m = _get_or_404(db, memorial_id)
    m.deleted_at = datetime.utcnow()
    db.commit()
    logger.info("Memorial soft-deleted: %s by %s", memorial_id, current_user.get("email"))
    return {"message": "Memorial deleted"}


@router.put("/{memorial_id}/status")
async def update_memorial_status(
    memorial_id: str,
    body: StatusBody,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    m = _get_or_404(db, memorial_id)
    m.status = body.status
    m.updated_at = datetime.utcnow()
    db.commit()
    db.refresh(m)
    logger.info("Memorial status -> %s: %s", body.status, memorial_id)
    return _to_dict(m)
    return _to_dict(m)
