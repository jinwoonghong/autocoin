from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates

from app.api.routes import build_router
from app.core.config import load_settings
from app.core.state import EngineState
from app.engine.controller import EngineController
from app.infra.repository import init_db

settings = load_settings()
state = EngineState()
controller = EngineController(state=state, settings=settings)

app = FastAPI(title="autocoin dashboard")
app.mount("/static", StaticFiles(directory="app/static"), name="static")
templates = Jinja2Templates(directory="app/templates")
app.include_router(build_router(state, controller))


@app.on_event("startup")
def on_startup() -> None:
    init_db()


@app.get("/", response_class=HTMLResponse)
def dashboard(request: Request):
    return templates.TemplateResponse(
        request,
        "dashboard.html",
        {"market": settings.trading_market, "paper_mode": settings.paper_mode},
    )
