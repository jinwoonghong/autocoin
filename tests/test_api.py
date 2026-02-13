from fastapi.testclient import TestClient


def test_status_endpoint():
    import os

    os.environ.setdefault("UPBIT_ACCESS_KEY", "dummy")
    os.environ.setdefault("UPBIT_SECRET_KEY", "dummy")

    from app.main import app

    client = TestClient(app)
    resp = client.get("/api/engine/status")
    assert resp.status_code == 200
    payload = resp.json()
    assert payload["ok"] is True
    data = payload["data"]
    assert data["status"] in {"IDLE", "RUNNING", "STOPPING", "ERROR"}
    assert "consecutive_failures" in data
    assert "max_consecutive_failures" in data


def test_start_stop_endpoints():
    import os
    from unittest.mock import patch

    os.environ.setdefault("UPBIT_ACCESS_KEY", "dummy")
    os.environ.setdefault("UPBIT_SECRET_KEY", "dummy")

    from app.main import app

    client = TestClient(app)
    with patch("app.engine.controller.EngineController._fetch_price", return_value=100.0):
        start_resp = client.post("/api/engine/start")
        assert start_resp.status_code == 200
        stop_resp = client.post("/api/engine/stop")
        assert stop_resp.status_code == 200
