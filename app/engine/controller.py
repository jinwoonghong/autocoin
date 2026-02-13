from threading import Thread
from datetime import datetime, timezone
import time

from app.core.state import EngineState, EngineStatus
from app.core.config import Settings
from app.engine.strategy import generate_signal
from app.infra.repository import log_event, insert_order
from app.infra.upbit_client import UpbitClient


class EngineController:
    def __init__(self, state: EngineState, settings: Settings):
        self.state = state
        self.settings = settings
        self.client = UpbitClient()
        self._thread: Thread | None = None

    def start(self) -> tuple[bool, str]:
        with self.state.lock:
            if self.state.status in {EngineStatus.RUNNING, EngineStatus.STOPPING}:
                return False, "engine already running"
            self.state.stop_event.clear()
            self.state.status = EngineStatus.RUNNING
            self.state.started_at = datetime.now(timezone.utc)
            self.state.last_error = None
            self._thread = Thread(target=self._run_loop, daemon=True)
            self._thread.start()
            log_event("INFO", "engine started")
        return True, "started"

    def stop(self) -> tuple[bool, str]:
        with self.state.lock:
            if self.state.status == EngineStatus.IDLE:
                return False, "engine already idle"
            if self.state.status == EngineStatus.ERROR:
                return False, "engine in error; reset needed"
            self.state.status = EngineStatus.STOPPING
            self.state.stop_event.set()
            log_event("INFO", "stop requested")
        return True, "stopping"

    def reset(self) -> tuple[bool, str]:
        with self.state.lock:
            if self.state.status != EngineStatus.ERROR:
                return False, "reset only allowed in ERROR"
            self.state.status = EngineStatus.IDLE
            self.state.last_error = None
            log_event("INFO", "engine reset")
        return True, "reset"

    def _run_loop(self) -> None:
        try:
            while not self.state.stop_event.is_set():
                _ = self.client.get_ticker_trade_price(self.settings.trading_market)
                signal = generate_signal()
                if signal in {"BUY", "SELL"}:
                    insert_order(
                        market=self.settings.trading_market,
                        side=signal,
                        amount_krw=self.settings.max_order_krw,
                        mode="PAPER" if self.settings.paper_mode else "LIVE",
                        status="FILLED" if self.settings.paper_mode else "REQUESTED",
                    )
                    log_event("INFO", f"{signal} signal executed")
                else:
                    log_event("DEBUG", "HOLD signal")
                with self.state.lock:
                    self.state.cycle_count += 1
                time.sleep(self.settings.loop_interval_sec)

            with self.state.lock:
                self.state.status = EngineStatus.IDLE
            log_event("INFO", "engine stopped")
        except Exception as exc:  # noqa: BLE001
            with self.state.lock:
                self.state.status = EngineStatus.ERROR
                self.state.last_error = str(exc)
            log_event("ERROR", f"engine crashed: {exc}")
