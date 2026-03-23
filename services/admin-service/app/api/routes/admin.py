"""Admin management API routes"""

from fastapi import APIRouter, Depends, HTTPException, status
from sqlalchemy.orm import Session
from app.db.database import get_db, User, AdminLog
from app.core.dependencies import get_admin_user
from app.core.logger import get_logger

router = APIRouter()
logger = get_logger(__name__)

@router.get("/users")
async def list_users(
    skip: int = 0,
    limit: int = 20,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    """List all users (admin only)"""
    users = db.query(User).offset(skip).limit(limit).all()
    total = db.query(User).count()
    
    return {
        "data": [
            {
                "id": u.id,
                "email": u.email,
                "username": u.username,
                "is_admin": u.is_admin,
                "is_active": u.is_active,
            }
            for u in users
        ],
        "total": total,
        "skip": skip,
        "limit": limit,
    }

@router.delete("/users/{user_id}")
async def delete_user(
    user_id: str,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    """Delete user (admin only)"""
    user = db.query(User).filter(User.id == user_id).first()
    if not user:
        raise HTTPException(status_code=404, detail="User not found")
    
    db.delete(user)
    db.commit()
    
    logger.info(f"User deleted: {user_id} by admin {current_user['email']}")
    return {"message": "User deleted"}

@router.get("/memorials")
async def list_memorials(
    skip: int = 0,
    limit: int = 20,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    """List all memorials for admin review"""
    # TODO: Implement memorial fetching from memorial service
    return {
        "data": [],
        "total": 0,
        "skip": skip,
        "limit": limit,
    }

@router.get("/logs")
async def get_admin_logs(
    skip: int = 0,
    limit: int = 50,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    """Get admin activity logs"""
    logs = db.query(AdminLog).offset(skip).limit(limit).all()
    total = db.query(AdminLog).count()
    
    return {
        "data": [
            {
                "id": log.id,
                "user_id": log.user_id,
                "action": log.action,
                "resource_type": log.resource_type,
                "resource_id": log.resource_id,
                "created_at": log.created_at.isoformat(),
            }
            for log in logs
        ],
        "total": total,
    }
