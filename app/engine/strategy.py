from random import random


def generate_signal() -> str:
    r = random()
    if r < 0.2:
        return "BUY"
    if r < 0.4:
        return "SELL"
    return "HOLD"
