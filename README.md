# autocoin

Upbit API 기반의 **수동 시작형 자동매매 서비스**입니다.  
프로세스를 실행하면 웹 대시보드가 열리고, 사용자가 Start/Stop/Reset으로 엔진을 직접 제어합니다.

---

## 1) 기획서 (Project Plan)

### 1.1 문제 정의
- 스케줄러/클라우드 없이, 내 PC를 상시 켜둔 환경에서 자동매매를 안정적으로 운영하고 싶다.
- 운영자가 즉시 상태를 확인하고 수동 개입(Start/Stop/Reset)할 수 있어야 한다.
- API 장애나 반복 실패 시 시스템이 자동으로 위험 상태(`ERROR`)로 전환되어야 한다.

### 1.2 목표
- 브라우저 기반 모니터링 대시보드 제공
- 수동 시작형 엔진 제어
- 엔진 상태머신 기반 안전 운영 (`IDLE`, `RUNNING`, `STOPPING`, `ERROR`)
- 연속 실패 임계치 기반 자동 보호

### 1.3 현재 범위 (MVP)
- [x] FastAPI 서버 + 대시보드(`/`)
- [x] Start/Stop/Reset API
- [x] Upbit ticker polling
- [x] 기본 신호(`BUY`/`SELL`/`HOLD`) 계산
- [x] 연속 실패 카운트 + 임계치 초과 시 `ERROR`
- [x] 테스트 코드(pytest)
- [ ] 실제 주문 실행 모듈
- [ ] 잔고/체결 저장소(DB)
- [ ] 손실 한도/리스크 정책 고도화

### 1.4 아키텍처 개요
- Web/API: FastAPI + Jinja2
- Engine: 백그라운드 스레드 루프
- Data Source: Upbit ticker API
- UI: 상태/실패횟수/로그/최근 오류 표시

상세 설계 문서:
- `docs/ARCHITECTURE.md`
- `docs/TECHNICAL_DESIGN.md`

### 1.5 운영 상태 모델
- `IDLE`: 엔진 중지
- `RUNNING`: 엔진 동작
- `STOPPING`: 중지 요청 후 종료 대기
- `ERROR`: 반복 실패/치명 오류 상태

### 1.6 기본 운영 원칙
- 실제 주문 기능이 완성되기 전까지는 모니터링/엔진 안정화에 집중
- `ERROR` 상태에서는 원인 확인 후 `Reset`으로 복구
- 운영 중 장애 추적을 위해 로그를 우선 확인

---

## 2) 사용 방법 (How to Run)

### 2.1 요구사항
- Python 3.11+
- 유효한 Upbit API 키 (환경변수)

### 2.2 설치
```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

### 2.3 환경변수 설정
```bash
export UPBIT_ACCESS_KEY="..."
export UPBIT_SECRET_KEY="..."

# 선택값
export APP_HOST="0.0.0.0"
export APP_PORT="8000"
export TRADING_MARKET="KRW-BTC"
export LOOP_INTERVAL_SEC="5"
export PAPER_MODE="true"
export MAX_CONSECUTIVE_FAILURES="3"
```

샘플 파일은 `.env.example` 참고.

### 2.4 실행
```bash
uvicorn app.main:app --host 0.0.0.0 --port 8000
```

브라우저 접속:
- `http://localhost:8000`

### 2.5 대시보드 조작
- **Start**: 엔진 시작 (`RUNNING`)
- **Stop**: 엔진 안전 중지 (`STOPPING -> IDLE`)
- **Reset**: 오류 상태 초기화 (`ERROR -> IDLE`)

화면에서 확인 가능한 항목:
- Status, Paper Mode, Thread Alive
- Market, Last Price, Signal, Iteration
- Failures / Max Failures, Last Error
- Recent Logs

---

## 3) API 요약

- `GET /` : Dashboard
- `GET /api/engine/status` : 엔진 상태
- `POST /api/engine/start` : 시작
- `POST /api/engine/stop` : 중지
- `POST /api/engine/reset` : 리셋
- `GET /api/logs/recent` : 최근 로그

---

## 4) 테스트

```bash
UPBIT_ACCESS_KEY=dummy UPBIT_SECRET_KEY=dummy pytest -q
```

---

## 5) 다음 개발 로드맵

1. Paper 주문 레이어(가상 체결 기록)
2. 리스크 정책(1회 주문 상한, 일 손실 한도, 킬스위치)
3. 주문/체결/잔고 DB 저장
4. 실주문 연동(소액) + 알림(텔레그램/디스코드)

---

## 6) 주의사항

- 본 프로젝트는 투자 수익을 보장하지 않습니다.
- API 키는 절대 코드/저장소에 커밋하지 마세요.
- 실주문 기능 도입 전에는 반드시 paper mode 및 소액 검증 단계를 거치세요.
