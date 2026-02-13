import sqlite3
from pathlib import Path
from datetime import datetime, timezone

DB_PATH = Path("data/autocoin.db")


def _connect() -> sqlite3.Connection:
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    return sqlite3.connect(DB_PATH)


def init_db() -> None:
    conn = _connect()
    conn.execute(
        """
        CREATE TABLE IF NOT EXISTS engine_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ts TEXT NOT NULL,
            level TEXT NOT NULL,
            message TEXT NOT NULL
        )
        """
    )
    conn.execute(
        """
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ts TEXT NOT NULL,
            market TEXT NOT NULL,
            side TEXT NOT NULL,
            amount_krw REAL NOT NULL,
            mode TEXT NOT NULL,
            status TEXT NOT NULL
        )
        """
    )
    conn.commit()
    conn.close()


def log_event(level: str, message: str) -> None:
    conn = _connect()
    conn.execute(
        "INSERT INTO engine_events(ts, level, message) VALUES (?, ?, ?)",
        (datetime.now(timezone.utc).isoformat(), level, message),
    )
    conn.commit()
    conn.close()


def insert_order(market: str, side: str, amount_krw: float, mode: str, status: str) -> None:
    conn = _connect()
    conn.execute(
        "INSERT INTO orders(ts, market, side, amount_krw, mode, status) VALUES (?, ?, ?, ?, ?, ?)",
        (datetime.now(timezone.utc).isoformat(), market, side, amount_krw, mode, status),
    )
    conn.commit()
    conn.close()


def recent_events(limit: int = 50) -> list[dict]:
    conn = _connect()
    conn.row_factory = sqlite3.Row
    rows = conn.execute(
        "SELECT ts, level, message FROM engine_events ORDER BY id DESC LIMIT ?", (limit,)
    ).fetchall()
    conn.close()
    return [dict(r) for r in rows]


def recent_orders(limit: int = 20) -> list[dict]:
    conn = _connect()
    conn.row_factory = sqlite3.Row
    rows = conn.execute(
        "SELECT ts, market, side, amount_krw, mode, status FROM orders ORDER BY id DESC LIMIT ?",
        (limit,),
    ).fetchall()
    conn.close()
    return [dict(r) for r in rows]
