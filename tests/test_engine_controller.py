from unittest.mock import patch

from app.engine.controller import EngineController
from app.engine.state import EngineStatus


def test_engine_start_and_stop():
    controller = EngineController(market="KRW-BTC", loop_interval_sec=0.05, paper_mode=True)

    with patch.object(controller, "_fetch_price", side_effect=[100.0, 101.0, 102.0, 103.0]):
        controller.start()
        import time

        time.sleep(0.12)
        snapshot = controller.status()
        assert snapshot.status in {EngineStatus.RUNNING, EngineStatus.STOPPING, EngineStatus.IDLE}
        assert snapshot.iteration >= 1
        assert snapshot.thread_alive is True

        controller.stop()
        time.sleep(0.05)
        stopped = controller.status()
        assert stopped.status == EngineStatus.IDLE
        assert stopped.thread_alive is False


def test_engine_error_transition_with_failure_threshold():
    controller = EngineController(market="KRW-BTC", loop_interval_sec=0.01, paper_mode=True, max_consecutive_failures=2)
    with patch.object(controller, "_fetch_price", side_effect=RuntimeError("boom")):
        controller.start()
        import time

        time.sleep(0.08)
        snapshot = controller.status()
        assert snapshot.status == EngineStatus.ERROR
        assert snapshot.consecutive_failures >= 2
        assert "boom" in (snapshot.last_error or "")


def test_reset_ignored_while_running():
    controller = EngineController(market="KRW-BTC", loop_interval_sec=0.05, paper_mode=True)
    with patch.object(controller, "_fetch_price", side_effect=[100.0, 100.0, 100.0]):
        controller.start()
        snapshot = controller.reset()
        assert snapshot.status == EngineStatus.RUNNING
        controller.stop()
