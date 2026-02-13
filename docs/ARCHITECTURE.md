# 아키텍처 문서 (Upbit 수동 시작형 자동매매)

## 1. 아키텍처 목표

- 스케줄러 기반 정기 실행이 아닌 **사용자 수동 시작형** 운영.
- 실행 즉시 모니터링 UI 제공.
- 거래 로직과 UI를 분리하여 안정적으로 제어.
- 단일 PC 상시 가동 환경에서 장애 복구/중단 제어가 쉬운 구조.

---

## 2. 컨텍스트 다이어그램

```text
[사용자]
   │ (브라우저)
   ▼
[Dashboard UI / API 서버] ────────────────┐
   │ Start/Stop/Status                    │
   │                                      │
   ▼                                      │
[Trading Engine Controller]               │
   │                                      │
   ▼                                      │
[Strategy + Risk + Execution Pipeline]    │
   │            │           │             │
   │            │           ├──▶ [Upbit REST API]
   │            └──▶ [Market Data Adapter]
   │
   ├──▶ [SQLite (orders, fills, pnl, logs)]
   └──▶ [Notifier (Telegram/Discord, optional)]
```

---

## 3. 논리 구성요소

### 3.1 API / UI Layer

- 역할
  - 대시보드 페이지 렌더링
  - Start/Stop/Status HTTP API 제공
  - 최근 로그/주문/잔고 조회
- 요구사항
  - 단일 엔진만 실행 가능(중복 실행 방지)
  - 요청 지연 시에도 UI 응답성 보장

### 3.2 Engine Controller

- 역할
  - 엔진 라이프사이클 관리 (`IDLE`, `RUNNING`, `STOPPING`, `ERROR`)
  - 엔진 시작/중지 신호 처리
- 요구사항
  - thread-safe 상태 관리(lock/event)
  - graceful stop(진행 중 사이클 완료 후 중단)

### 3.3 Trading Pipeline

1. Market Data Fetch
2. Signal Generation (전략)
3. Risk Gate (주문 가능 여부 판단)
4. Order Execution
5. Fill Reconciliation
6. State/Log Persist

### 3.4 Persistence

- DB: SQLite(초기)
- 저장 대상
  - 시스템 이벤트 로그
  - 전략 신호
  - 주문 요청/응답
  - 체결 내역
  - 잔고 스냅샷
  - 실현/미실현 손익

### 3.5 Notification

- 경고 조건
  - 연속 API 실패
  - 일 손실 한도 초과
  - 주문 실패/부분 체결 장기 미해결

---

## 4. 런타임 흐름

1. 사용자가 앱 실행
2. 웹 서버 구동 및 대시보드 진입 가능
3. 사용자가 Start 클릭
4. Controller가 엔진 시작 (중복 실행 방지 검사)
5. 엔진 루프 수행
   - 시세 조회 → 전략 판단 → 리스크 검사 → 주문 실행 → 저장/알림
6. Stop 클릭 시 STOPPING 전환 후 안전 종료
7. 상태/로그는 UI에서 지속 확인

---

## 5. 상태 모델

- `IDLE`: 엔진 미실행
- `RUNNING`: 엔진 동작 중
- `STOPPING`: 중단 신호 수신, 사이클 종료 대기
- `ERROR`: 복구 불가능 오류로 정지

상태 전이 규칙:
- `IDLE -> RUNNING`: Start 성공
- `RUNNING -> STOPPING`: Stop 요청
- `STOPPING -> IDLE`: 정상 종료
- `RUNNING/STOPPING -> ERROR`: 치명적 예외
- `ERROR -> IDLE`: 수동 Reset

---

## 6. 보안/운영 설계

- 인증정보
  - `UPBIT_ACCESS_KEY`, `UPBIT_SECRET_KEY` 환경변수 사용
  - 코드/로그에 키 값 노출 금지
- 운영
  - 프로세스 watchdog 권장
  - 재부팅 후 자동 실행 여부는 옵션화
- 감사추적
  - 모든 주문 요청/응답 원문 일부 보관(민감정보 마스킹)

---

## 7. 확장 포인트

- 전략 모듈 플러그인화 (`Strategy` 인터페이스)
- 거래소 어댑터 추상화 (`ExchangeClient`)로 다중 거래소 지원
- SQLite -> PostgreSQL 마이그레이션
- 실시간 WebSocket 데이터 연결

---

## 8. 비기능 요구사항

- 안정성: 장애 시 자동 중단/복구 가이드 제공
- 관측성: UI + 로그 + 알림 3중 관측
- 성능: 단일 사용자/단일 전략 기준 저부하 운영
- 유지보수성: 전략/실행/리스크 모듈 분리
