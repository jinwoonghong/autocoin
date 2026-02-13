from __future__ import annotations

from fastapi import APIRouter, Request
from fastapi.responses import HTMLResponse
from fastapi.templating import Jinja2Templates

router = APIRouter()
templates = Jinja2Templates(directory="app/templates")


@router.get("/", response_class=HTMLResponse)
def dashboard(request: Request) -> HTMLResponse:
    return templates.TemplateResponse("dashboard.html", {"request": request})


@router.get("/api/engine/status")
def engine_status(request: Request) -> dict:
    snapshot = request.app.state.engine.status()
    return {"ok": True, "data": snapshot.__dict__, "error": None}


@router.get("/api/logs/recent")
def recent_logs(request: Request, limit: int = 100) -> dict:
    events = request.app.state.engine.recent_events(limit=limit)
    return {"ok": True, "data": [e.__dict__ for e in events], "error": None}


@router.post("/api/engine/start")
def engine_start(request: Request) -> dict:
    snapshot = request.app.state.engine.start()
    return {"ok": True, "data": snapshot.__dict__, "error": None}


@router.post("/api/engine/stop")
def engine_stop(request: Request) -> dict:
    snapshot = request.app.state.engine.stop()
    return {"ok": True, "data": snapshot.__dict__, "error": None}


@router.post("/api/engine/reset")
def engine_reset(request: Request) -> dict:
    snapshot = request.app.state.engine.reset()
    return {"ok": True, "data": snapshot.__dict__, "error": None}
