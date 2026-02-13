from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime, timezone
from enum import Enum


class EngineStatus(str, Enum):
    IDLE = "IDLE"
    RUNNING = "RUNNING"
    STOPPING = "STOPPING"
    ERROR = "ERROR"


@dataclass
class EngineEvent:
    ts: str
    level: str
    message: str


@dataclass
class EngineSnapshot:
    status: EngineStatus = EngineStatus.IDLE
    market: str = "KRW-BTC"
    paper_mode: bool = True
    iteration: int = 0
    last_price: float | None = None
    last_signal: str = "HOLD"
    last_error: str | None = None
    consecutive_failures: int = 0
    max_consecutive_failures: int = 3
    thread_alive: bool = False
    updated_at: str = field(default_factory=lambda: datetime.now(timezone.utc).isoformat())



def utc_now_iso() -> str:
    return datetime.now(timezone.utc).isoformat()
