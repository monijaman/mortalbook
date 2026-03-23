"""Analytics API routes"""

from fastapi import APIRouter, Depends
from sqlalchemy.orm import Session
from app.db.database import get_db, AnalyticsEvent
from app.core.dependencies import get_admin_user
from app.core.logger import get_logger
from datetime import datetime, timedelta

router = APIRouter()
logger = get_logger(__name__)

@router.get("/dashboard")
async def get_analytics_dashboard(
    days: int = 30,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    """Get analytics dashboard data"""
    from_date = datetime.utcnow() - timedelta(days=days)
    
    events = db.query(AnalyticsEvent).filter(
        AnalyticsEvent.created_at >= from_date
    ).all()
    
    return {
        "total_events": len(events),
        "period_days": days,
        "event_breakdown": {},
        "daily_counts": [],
    }

@router.get("/events")
async def get_analytics_events(
    skip: int = 0,
    limit: int = 100,
    db: Session = Depends(get_db),
    current_user: dict = Depends(get_admin_user),
):
    """Get analytics events"""
    events = db.query(AnalyticsEvent).offset(skip).limit(limit).all()
    total = db.query(AnalyticsEvent).count()
    
    return {
        "data": [
            {
                "id": e.id,
                "event_type": e.event_type,
                "created_at": e.created_at.isoformat(),
            }
            for e in events
        ],
        "total": total,
    }
