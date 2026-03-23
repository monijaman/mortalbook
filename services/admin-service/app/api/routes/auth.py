"""Authentication API routes"""

from fastapi import APIRouter, HTTPException, status, Depends
from fastapi.security import HTTPBearer
from pydantic import BaseModel, EmailStr
from sqlalchemy.orm import Session
from uuid import uuid4
from datetime import timedelta

from app.db.database import get_db, User, SessionLocal
from app.core.security import (
    get_password_hash,
    verify_password,
    create_access_token,
    create_refresh_token,
)
from app.core.logger import get_logger

router = APIRouter()
logger = get_logger(__name__)

# Request/Response models
class LoginRequest(BaseModel):
    email: str
    password: str

class RegisterRequest(BaseModel):
    email: EmailStr
    username: str
    password: str
    full_name: str

class TokenResponse(BaseModel):
    access_token: str
    refresh_token: str
    token_type: str = "bearer"
    expires_in: int = 3600

class UserResponse(BaseModel):
    id: str
    email: str
    username: str
    full_name: str
    is_admin: bool

@router.post("/login", response_model=TokenResponse)
async def login(request: LoginRequest, db: Session = Depends(get_db)):
    """User login endpoint"""
    user = db.query(User).filter(User.email == request.email).first()
    
    if not user or not verify_password(request.password, user.hashed_password):
        logger.warning(f"Failed login attempt for {request.email}")
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Invalid credentials",
        )
    
    if not user.is_active:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail="User account is inactive",
        )
    
    access_token = create_access_token({
        "sub": user.id,
        "email": user.email,
        "is_admin": user.is_admin,
    })
    
    refresh_token = create_refresh_token({
        "sub": user.id,
        "email": user.email,
    })
    
    logger.info(f"User logged in: {user.email}")
    
    return TokenResponse(
        access_token=access_token,
        refresh_token=refresh_token,
        expires_in=3600,
    )

@router.post("/register", response_model=TokenResponse, status_code=status.HTTP_201_CREATED)
async def register(request: RegisterRequest, db: Session = Depends(get_db)):
    """User registration endpoint"""
    # Check if user exists
    existing = db.query(User).filter(
        (User.email == request.email) | (User.username == request.username)
    ).first()
    
    if existing:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="User already exists",
        )
    
    # Create new user
    user = User(
        id=str(uuid4()),
        email=request.email,
        username=request.username,
        full_name=request.full_name,
        hashed_password=get_password_hash(request.password),
        is_active=True,
        is_admin=False,
    )
    
    db.add(user)
    db.commit()
    db.refresh(user)
    
    access_token = create_access_token({
        "sub": user.id,
        "email": user.email,
        "is_admin": user.is_admin,
    })
    
    refresh_token = create_refresh_token({
        "sub": user.id,
        "email": user.email,
    })
    
    logger.info(f"New user registered: {user.email}")
    
    return TokenResponse(
        access_token=access_token,
        refresh_token=refresh_token,
        expires_in=3600,
    )

@router.post("/refresh", response_model=TokenResponse)
async def refresh_token(request: dict):
    """Refresh access token"""
    # TODO: Implement token refresh logic
    raise HTTPException(
        status_code=status.HTTP_501_NOT_IMPLEMENTED,
        detail="Not implemented",
    )
