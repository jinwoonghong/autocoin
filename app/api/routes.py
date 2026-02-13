from datetime import datetime, timezone
from fastapi import APIRouter

from app.core.state import EngineState
from app.engine.controller import EngineController
from app.infra.repository import recent_events, recent_orders


def build_router(state: EngineState, controller: EngineController) -> APIRouter:
    router = APIRouter(prefix="/api")

    @router.post("/engine/start")
    def start_engine():
        ok, msg = controller.start()
        return {"ok": ok, "data": {"status": state.status, "message": msg}, "error": None if ok else msg, "ts": datetime.now(timezone.utc)}

    @router.post("/engine/stop")
    def stop_engine():
        ok, msg = controller.stop()
        return {"ok": ok, "data": {"status": state.status, "message": msg}, "error": None if ok else msg, "ts": datetime.now(timezone.utc)}

    @router.post("/engine/reset")
    def reset_engine():
        ok, msg = controller.reset()
        return {"ok": ok, "data": {"status": state.status, "message": msg}, "error": None if ok else msg, "ts": datetime.now(timezone.utc)}

    @router.get("/engine/status")
    def engine_status():
        return {
            "ok": True,
            "data": {
                "status": state.status,
                "last_error": state.last_error,
                "started_at": state.started_at,
                "cycle_count": state.cycle_count,
            },
            "error": None,
            "ts": datetime.now(timezone.utc),
        }

    @router.get("/orders/recent")
    def orders():
        return {"ok": True, "data": {"orders": recent_orders()}, "error": None, "ts": datetime.now(timezone.utc)}

    @router.get("/logs/recent")
    def logs():
        return {"ok": True, "data": {"logs": recent_events()}, "error": None, "ts": datetime.now(timezone.utc)}

    return router
