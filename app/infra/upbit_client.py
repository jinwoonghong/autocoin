import requests


class UpbitClient:
    BASE_URL = "https://api.upbit.com/v1"

    def get_ticker_trade_price(self, market: str) -> float:
        resp = requests.get(f"{self.BASE_URL}/ticker", params={"markets": market}, timeout=5)
        resp.raise_for_status()
        data = resp.json()
        return float(data[0]["trade_price"])
