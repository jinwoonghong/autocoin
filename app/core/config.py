from pydantic import BaseModel, Field
import os


class Settings(BaseModel):
    upbit_access_key: str = Field(default="")
    upbit_secret_key: str = Field(default="")
    app_host: str = Field(default="0.0.0.0")
    app_port: int = Field(default=8000)
    trading_market: str = Field(default="KRW-BTC")
    loop_interval_sec: float = Field(default=2.0)
    paper_mode: bool = Field(default=True)
    max_order_krw: float = Field(default=10000)


def _as_bool(value: str | None, default: bool) -> bool:
    if value is None:
        return default
    return value.lower() in {"1", "true", "yes", "on"}


def load_settings() -> Settings:
    return Settings(
        upbit_access_key=os.getenv("UPBIT_ACCESS_KEY", ""),
        upbit_secret_key=os.getenv("UPBIT_SECRET_KEY", ""),
        app_host=os.getenv("APP_HOST", "0.0.0.0"),
        app_port=int(os.getenv("APP_PORT", "8000")),
        trading_market=os.getenv("TRADING_MARKET", "KRW-BTC"),
        loop_interval_sec=float(os.getenv("LOOP_INTERVAL_SEC", "2")),
        paper_mode=_as_bool(os.getenv("PAPER_MODE"), True),
        max_order_krw=float(os.getenv("MAX_ORDER_KRW", "10000")),
    )
