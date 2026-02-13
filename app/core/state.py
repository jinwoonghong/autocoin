from dataclasses import dataclass, field
from enum import Enum
from threading import Event, Lock
from datetime import datetime


class EngineStatus(str, Enum):
    IDLE = "IDLE"
    RUNNING = "RUNNING"
    STOPPING = "STOPPING"
    ERROR = "ERROR"


@dataclass
class EngineState:
    status: EngineStatus = EngineStatus.IDLE
    last_error: str | None = None
    started_at: datetime | None = None
    cycle_count: int = 0
    lock: Lock = field(default_factory=Lock)
    stop_event: Event = field(default_factory=Event)
