"""Structured logging configuration"""

import logging
import json
from pythonjsonlogger import jsonlogger
from app.core.config import settings

def get_logger(name: str) -> logging.Logger:
    """Get configured logger instance"""
    logger = logging.getLogger(name)
    
    # Only configure once
    if not logger.handlers:
        # Set level
        level = getattr(logging, settings.LOG_LEVEL, logging.INFO)
        logger.setLevel(level)
        
        # Console handler with JSON formatter
        handler = logging.StreamHandler()
        formatter = jsonlogger.JsonFormatter(
            '%(timestamp)s %(level)s %(name)s %(message)s'
        )
        handler.setFormatter(formatter)
        logger.addHandler(handler)
    
    return logger
