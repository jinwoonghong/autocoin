from __future__ import annotations

import threading
import time
from collections import deque

import httpx

from app.engine.state import EngineEvent, EngineSnapshot, EngineStatus, utc_now_iso


class EngineController:
    def __init__(
        self,
        market: str,
        loop_interval_sec: float,
        paper_mode: bool,
        max_consecutive_failures: int = 3,
    ) -> None:
        self._market = market
        self._loop_interval_sec = loop_interval_sec
        self._paper_mode = paper_mode
        self._max_consecutive_failures = max_consecutive_failures
        self._lock = threading.Lock()
        self._stop_event = threading.Event()
        self._thread: threading.Thread | None = None
        self._snapshot = EngineSnapshot(
            market=market,
            paper_mode=paper_mode,
            max_consecutive_failures=max_consecutive_failures,
        )
        self._events: deque[EngineEvent] = deque(maxlen=500)
        self._record("INFO", "Engine initialized")

    def start(self) -> EngineSnapshot:
        with self._lock:
            if self._snapshot.status in {EngineStatus.RUNNING, EngineStatus.STOPPING}:
                return self._clone_snapshot_locked()
            self._stop_event.clear()
            self._snapshot.status = EngineStatus.RUNNING
            self._snapshot.last_error = None
            self._snapshot.consecutive_failures = 0
            self._snapshot.thread_alive = True
            self._snapshot.updated_at = utc_now_iso()
            self._thread = threading.Thread(target=self._run_loop, daemon=True)
            self._thread.start()
            self._record("INFO", "Engine started")
            return self._clone_snapshot_locked()

    def stop(self) -> EngineSnapshot:
        with self._lock:
            if self._snapshot.status == EngineStatus.IDLE:
                return self._clone_snapshot_locked()
            if self._snapshot.status == EngineStatus.ERROR:
                self._snapshot.status = EngineStatus.IDLE
                self._snapshot.thread_alive = False
                self._snapshot.updated_at = utc_now_iso()
                self._record("INFO", "Engine reset to IDLE from ERROR")
                return self._clone_snapshot_locked()
            self._snapshot.status = EngineStatus.STOPPING
            self._snapshot.updated_at = utc_now_iso()
            self._stop_event.set()
            self._record("INFO", "Stop requested")
            thread = self._thread

        if thread is not None:
            thread.join(timeout=max(1.0, self._loop_interval_sec * 3))

        with self._lock:
            self._snapshot.thread_alive = bool(self._thread and self._thread.is_alive())
            if self._snapshot.status == EngineStatus.STOPPING and not self._snapshot.thread_alive:
                self._snapshot.status = EngineStatus.IDLE
            self._snapshot.updated_at = utc_now_iso()
            return self._clone_snapshot_locked()

    def reset(self) -> EngineSnapshot:
        with self._lock:
            if self._snapshot.status == EngineStatus.RUNNING:
                self._record("WARN", "Reset ignored while RUNNING. Use stop first.")
                return self._clone_snapshot_locked()
            self._snapshot.status = EngineStatus.IDLE
            self._snapshot.last_error = None
            self._snapshot.consecutive_failures = 0
            self._snapshot.thread_alive = False
            self._snapshot.updated_at = utc_now_iso()
            self._record("INFO", "Engine reset")
            return self._clone_snapshot_locked()

    def status(self) -> EngineSnapshot:
        with self._lock:
            return self._clone_snapshot_locked()

    def _clone_snapshot_locked(self) -> EngineSnapshot:
        snap = EngineSnapshot(**self._snapshot.__dict__)
        snap.thread_alive = bool(self._thread and self._thread.is_alive())
        return snap

    def recent_events(self, limit: int = 100) -> list[EngineEvent]:
        return list(self._events)[-limit:]

    def _record(self, level: str, message: str) -> None:
        self._events.append(EngineEvent(ts=utc_now_iso(), level=level, message=message))

    def _run_loop(self) -> None:
        while not self._stop_event.is_set():
            try:
                price = self._fetch_price(self._market)
                with self._lock:
                    previous_price = self._snapshot.last_price
                signal = self._make_signal(previous_price, price)

                with self._lock:
                    self._snapshot.iteration += 1
                    self._snapshot.last_price = price
                    self._snapshot.last_signal = signal
                    self._snapshot.consecutive_failures = 0
                    self._snapshot.updated_at = utc_now_iso()
                self._record("INFO", f"tick={self._snapshot.iteration} price={price:.0f} signal={signal}")
            except Exception as exc:  # noqa: BLE001
                with self._lock:
                    self._snapshot.consecutive_failures += 1
                    failures = self._snapshot.consecutive_failures
                    self._snapshot.last_error = str(exc)
                    self._snapshot.updated_at = utc_now_iso()

                self._record("ERROR", f"Loop error ({failures}/{self._max_consecutive_failures}): {exc}")

                if failures >= self._max_consecutive_failures:
                    with self._lock:
                        self._snapshot.status = EngineStatus.ERROR
                        self._snapshot.thread_alive = False
                        self._snapshot.updated_at = utc_now_iso()
                    self._record("ERROR", "Engine switched to ERROR due to repeated failures")
                    return

                time.sleep(min(self._loop_interval_sec, 1.0))
                continue

            time.sleep(self._loop_interval_sec)

        with self._lock:
            self._snapshot.status = EngineStatus.IDLE
            self._snapshot.thread_alive = False
            self._snapshot.updated_at = utc_now_iso()
        self._record("INFO", "Engine stopped")

    @staticmethod
    def _fetch_price(market: str) -> float:
        response = httpx.get("https://api.upbit.com/v1/ticker", params={"markets": market}, timeout=10.0)
        response.raise_for_status()
        data = response.json()
        if not data:
            raise RuntimeError("No ticker data returned")
        return float(data[0]["trade_price"])

    @staticmethod
    def _make_signal(previous_price: float | None, current_price: float) -> str:
        if previous_price is None:
            return "HOLD"
        if current_price > previous_price:
            return "BUY"
        if current_price < previous_price:
            return "SELL"
        return "HOLD"
