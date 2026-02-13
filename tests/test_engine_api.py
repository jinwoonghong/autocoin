import time
from fastapi.testclient import TestClient

from app.main import app, controller


def test_start_and_stop(monkeypatch):
    monkeypatch.setattr(controller.client, "get_ticker_trade_price", lambda market: 100.0)
    with TestClient(app) as client:
        start = client.post('/api/engine/start').json()
        assert start['ok'] is True

        time.sleep(0.2)

        status = client.get('/api/engine/status').json()
        assert status['data']['status'] in {'RUNNING', 'STOPPING', 'IDLE'}

        stop = client.post('/api/engine/stop').json()
        assert stop['ok'] is True


def test_dashboard_page():
    with TestClient(app) as client:
        res = client.get('/')
        assert res.status_code == 200
        assert 'Autocoin Dashboard' in res.text
