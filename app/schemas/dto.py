from datetime import datetime
from pydantic import BaseModel


class ApiResponse(BaseModel):
    ok: bool
    data: dict
    error: str | None = None
    ts: datetime
