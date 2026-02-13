# 기술 설계서 (Technical Design)

## 1. 목적

본 문서는 Upbit API 기반 수동 시작형 자동매매 서비스를 구현하기 위한 상세 기술 기준을 정의한다.

핵심 요구:
- 사용자가 앱을 실행하면 즉시 모니터링 화면 접근 가능
- Start/Stop으로 엔진을 제어
- API 키는 환경변수 사용
- 거래/오류/손익을 추적 가능

---

## 2. 기술 스택 제안

- Language: Python 3.11+
- Web: FastAPI + Jinja2(초기)
- Runtime: Uvicorn
- Data: SQLite + SQLAlchemy
- HTTP Client: httpx
- Optional Exchange SDK: pyupbit
- Logging: structlog 또는 표준 logging(JSON formatter)
- Tests: pytest

선정 이유:
- 빠른 MVP 구축
- 비동기/동기 선택 유연성
- 로컬 PC 상시 운영에 적합한 단순 배포

---

## 3. 디렉터리 설계

```text
autocoin/
  app/
    main.py                  # FastAPI entry
    api/
      routes.py              # start/stop/status/endpoints
    core/
      config.py              # env loading
      logger.py              # logging setup
      state.py               # engine shared state
    engine/
      controller.py          # lifecycle control
      loop.py                # trading loop
      risk.py                # risk checks
      strategy.py            # signal generation
      execution.py           # order placement
    infra/
      upbit_client.py        # API adapter
      repository.py          # DB access
    schemas/
      dto.py
    templates/
      dashboard.html
    static/
      app.css
      app.js
  docs/
    ARCHITECTURE.md
    TECHNICAL_DESIGN.md
  tests/
    test_state_machine.py
    test_risk.py
  .env.example
  requirements.txt
  README.md
```

---

## 4. 설정(환경변수) 설계

필수:
- `UPBIT_ACCESS_KEY`
- `UPBIT_SECRET_KEY`

선택:
- `APP_HOST` (default: `0.0.0.0`)
- `APP_PORT` (default: `8000`)
- `TRADING_MARKET` (default: `KRW-BTC`)
- `LOOP_INTERVAL_SEC` (default: `5`)
- `PAPER_MODE` (default: `true`)
- `MAX_ORDER_KRW` (default: `10000`)
- `MAX_DAILY_LOSS_KRW` (default: `50000`)

검증 규칙:
- 필수 키 누락 시 부팅 실패
- 숫자형 설정은 범위 검증

---

## 5. API 설계

### 5.1 엔드포인트

- `GET /` : 대시보드 페이지
- `POST /api/engine/start` : 엔진 시작
- `POST /api/engine/stop` : 엔진 중지
- `POST /api/engine/reset` : ERROR -> IDLE 복구
- `GET /api/engine/status` : 현재 상태/최근 오류
- `GET /api/account/balance` : 잔고 요약
- `GET /api/orders/recent` : 최근 주문/체결
- `GET /api/logs/recent` : 최근 시스템 로그

### 5.2 응답 표준

```json
{
  "ok": true,
  "data": {},
  "error": null,
  "ts": "2026-02-13T12:00:00Z"
}
```

---

## 6. 엔진 루프 상세

주기(`LOOP_INTERVAL_SEC`)마다 수행:
1. 시장 데이터 수집
2. 전략 신호 계산 (`BUY`/`SELL`/`HOLD`)
3. 리스크 규칙 검사
4. 주문 실행(또는 paper fill)
5. 주문/체결/잔고/PnL 저장
6. 대시보드/알림 업데이트

예외 처리 원칙:
- 네트워크 오류: 지수 backoff 후 재시도
- 인증 오류: 즉시 `ERROR` 전환
- 주문 거절: 로그 기록 + 전략 입력으로 피드백

---

## 7. 리스크 정책 (초기값)

- 1회 주문 상한: `MAX_ORDER_KRW`
- 일 손실 상한: `MAX_DAILY_LOSS_KRW`
- 동일 방향 연속 주문 쿨다운: 예) 30초
- 보유 비중 상한: 예) 총 자산의 30%
- Kill-switch 조건
  - 일 손실 초과
  - 연속 주문 실패 N회 초과
  - API 연속 장애 임계치 초과

---

## 8. 데이터 모델(초안)

### 8.1 engine_events
- id, ts, level, state, message, meta_json

### 8.2 signals
- id, ts, market, signal, score, meta_json

### 8.3 orders
- id, ts, market, side, ord_type, price, volume, status, exchange_order_id, raw_json

### 8.4 fills
- id, ts, order_id, price, volume, fee, raw_json

### 8.5 balance_snapshots
- id, ts, krw, coin_symbol, coin_volume, avg_buy_price, total_eval_krw

### 8.6 pnl_daily
- id, date, realized_krw, unrealized_krw, max_drawdown_krw

---

## 9. 모니터링 UI 설계

화면 구성:
- 엔진 상태 배지 (`IDLE/RUNNING/STOPPING/ERROR`)
- Start/Stop/Reset 버튼
- 잔고 카드(KRW, 보유수량, 평가금액)
- 최근 신호 카드
- 최근 주문/체결 테이블
- 실시간 로그 패널

UX 원칙:
- 버튼 중복 클릭 방지(disabled 처리)
- 위험 상태(`ERROR`) 시 시각적 강조

---

## 10. 테스트 전략

1. 단위 테스트
   - 상태 전이 로직
   - 리스크 게이트 로직
   - 전략 함수 deterministic 검증
2. 통합 테스트
   - start -> running -> stop 시나리오
   - paper mode 주문 lifecycle
3. 회귀 테스트
   - 주요 API 응답 스키마 고정

---

## 11. 릴리스 단계 제안

- Milestone 1: 대시보드 + 엔진 상태 제어 + paper mode
- Milestone 2: 실 API 주문 연동(소액) + 리스크 제한
- Milestone 3: 고도화(전략 플러그인, 알림, 리포트)

---

## 12. 운영 체크리스트

- [ ] API 키 환경변수 정상 주입
- [ ] `PAPER_MODE=true`로 사전 검증
- [ ] 일 손실/주문 상한 확인
- [ ] 로그 파일/DB 백업 경로 확인
- [ ] 비상 중단 절차(Stop/Kill-switch) 숙지
