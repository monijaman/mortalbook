"""User management API routes"""

from fastapi import APIRouter, Depends
from sqlalchemy.orm import Session
from app.db.database import get_db, User
from app.core.dependencies import get_current_user
from app.core.logger import get_logger

router = APIRouter()
logger = get_logger(__name__)

@router.get("/me")
async def get_current_user_profile(
    current_user: dict = Depends(get_current_user),
    db: Session = Depends(get_db),
):
    """Get current user profile"""
    user = db.query(User).filter(User.id == current_user["sub"]).first()
    if not user:
        return {"error": "User not found"}
    
    return {
        "id": user.id,
        "email": user.email,
        "username": user.username,
        "full_name": user.full_name,
        "is_admin": user.is_admin,
    }

@router.put("/me")
async def update_current_user_profile(
    updates: dict,
    current_user: dict = Depends(get_current_user),
    db: Session = Depends(get_db),
):
    """Update current user profile"""
    user = db.query(User).filter(User.id == current_user["sub"]).first()
    if not user:
        return {"error": "User not found"}
    
    if "full_name" in updates:
        user.full_name = updates["full_name"]
    
    db.commit()
    logger.info(f"User profile updated: {user.email}")
    
    return {"message": "Profile updated"}
