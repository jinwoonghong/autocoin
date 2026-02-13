from __future__ import annotations

import os
from dataclasses import dataclass


@dataclass(frozen=True)
class Settings:
    upbit_access_key: str
    upbit_secret_key: str
    app_host: str = "0.0.0.0"
    app_port: int = 8000
    trading_market: str = "KRW-BTC"
    loop_interval_sec: float = 5.0
    paper_mode: bool = True
    max_order_krw: int = 10000
    max_daily_loss_krw: int = 50000
    max_consecutive_failures: int = 3


def _to_bool(value: str | None, default: bool) -> bool:
    if value is None:
        return default
    return value.strip().lower() in {"1", "true", "yes", "on"}


def load_settings() -> Settings:
    access_key = os.getenv("UPBIT_ACCESS_KEY", "")
    secret_key = os.getenv("UPBIT_SECRET_KEY", "")
    if not access_key or not secret_key:
        raise RuntimeError("UPBIT_ACCESS_KEY and UPBIT_SECRET_KEY must be set")

    return Settings(
        upbit_access_key=access_key,
        upbit_secret_key=secret_key,
        app_host=os.getenv("APP_HOST", "0.0.0.0"),
        app_port=int(os.getenv("APP_PORT", "8000")),
        trading_market=os.getenv("TRADING_MARKET", "KRW-BTC"),
        loop_interval_sec=float(os.getenv("LOOP_INTERVAL_SEC", "5")),
        paper_mode=_to_bool(os.getenv("PAPER_MODE"), True),
        max_order_krw=int(os.getenv("MAX_ORDER_KRW", "10000")),
        max_daily_loss_krw=int(os.getenv("MAX_DAILY_LOSS_KRW", "50000")),
        max_consecutive_failures=int(os.getenv("MAX_CONSECUTIVE_FAILURES", "3")),
    )
