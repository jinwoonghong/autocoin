from __future__ import annotations

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

from app.api.routes import router
from app.core.config import load_settings
from app.engine.controller import EngineController

settings = load_settings()

app = FastAPI(title="autocoin")
app.mount("/static", StaticFiles(directory="app/static"), name="static")
app.include_router(router)
app.state.engine = EngineController(
    market=settings.trading_market,
    loop_interval_sec=settings.loop_interval_sec,
    paper_mode=settings.paper_mode,
    max_consecutive_failures=settings.max_consecutive_failures,
)
