# Acceptance Criteria: SPEC-TRADING-001

**TAG BLOCK**: `SPEC-ID: SPEC-TRADING-001`

---

## 1. Overview (개요)

본 문서는 Upbit Automated Trading Agent System의 인수 기준을 정의합니다. 모든 기능은 Gherkin 형식(Given-When-Then)으로 명시되어 있으며, 각 기능은 자동화된 테스트로 검증될 수 있습니다.

---

## 2. Functional Acceptance Criteria (기능적 인수 기준)

### AC-001: Market Monitor - WebSocket 연결

**Feature**: 실시간 시계 모니터링

**Scenario**: WebSocket 연결 성립

```
Given 시스템이 시작되었을 때
And Upbit API 자격증명이 유효할 때
When WebSocket 연결을 시도하면
Then 연결이 성공적으로 맺어져야 한다
And Top 20 코인의 시계 채널을 구독해야 한다
```

**Scenario**: WebSocket 연결 끊김 시 재연결

```
Given WebSocket이 연결되어 있을 때
When 네트워크 오류로 연결이 끊어지면
Then 5초 이내에 자동 재연결을 시도해야 한다
And 최대 5회까지 재시도해야 한다
And 재연결 성공 로그를 기록해야 한다
```

**Scenario**: 시계 데이터 수신

```
Given WebSocket이 연결되어 있을 때
When KRW-BTC의 시계 데이터를 수신하면
Then 데이터를 파싱하여 PriceTick으로 변환해야 한다
And 타임스탬프를 검증해야 한다
And Signal Detector에게 데이터를 전달해야 한다
```

---

### AC-002: Signal Detector - 모멘텀 감지

**Feature**: 가격 급등 신호 감지

**Scenario**: 5% 이상 상승 시 매수 신호 생성

```
Given 특정 코인이 1시간 동안 5% 이상 상승했을 때
And 거래량이 평균의 2배 이상일 때
When Signal Detector가 분석을 수행하면
Then BuySignal을 생성해야 한다
And 신뢰도 점수를 0.7 이상으로 부여해야 한다
And Decision Maker에게 신호를 전달해야 한다
```

**Scenario**: 미달 시 미신호

```
Given 특정 코인이 3%만 상승했을 때
When Signal Detector가 분석을 수행하면
Then BuySignal을 생성하지 않아야 한다
And 현재 상태를 유지해야 한다
```

**Scenario**: 이동평균 교차 감지

```
Given 5분 이동평균이 1시간 이동평균을 상향 돌파할 때
When Signal Detector가 분석을 수행하면
Then 추가 매수 신호를 생성해야 한다
And 신호 신뢰도를 증가시켜야 한다
```

---

### AC-003: Decision Maker - 거래 결정

**Feature**: 매수 결정

**Scenario**: 포지션 없을 때 매수 신호 수신

```
Given 현재 보유 중인 포지션이 없을 때
And BTC에 대한 BuySignal을 수신했을 때
And 계정 잔고가 충분할 때 (최소 5,000 KRW 이상)
When Decision Maker가 결정을 내리면
Then BTC 매수 주문을 생성해야 한다
And 주문 금액은 잔고의 50% 이하여야 한다
And Execution Agent에게 주문을 전달해야 한다
```

**Scenario**: 포지션 있을 때 매수 신호 무시

```
Given 현재 ETH를 보유 중일 때
And BTC에 대한 BuySignal을 수신했을 때
When Decision Maker가 결정을 내리면
Then 새로운 매수를 하지 않아야 한다
And "포지션 있음" 로그를 기록해야 한다
```

**Scenario**: 잔고 부족 시 주문 스킵

```
Given 계정 잔고가 3,000 KRW일 때
And BuySignal을 수신했을 때
When Decision Maker가 결정을 내리면
Then 주문을 생성하지 않아야 한다
And "잔고 부족" 경고를 기록해야 한다
```

---

### AC-004: Execution Agent - 주문 실행

**Feature**: 매수 주문 실행

```
Given Decision Maker로부터 BTC 매수 주문을 받았을 때
When Execution Agent가 주문을 제출하면
Then Upbit API에 지정가 주문을 전송해야 한다
And 주문 ID를 저장해야 한다
And 주문 상태를 조회하여 체결을 확인해야 한다
```

**Scenario**: 주문 체결 확인

```
Given 매수 주문이 제출되었을 때
When 주문이 체결되면
Then 포지션을 생성해야 한다
And 평균 단가를 기록해야 한다
And Risk Manager에게 포지션을 통지해야 한다
And Discord에 매수 체결 알림을 전송해야 한다
```

**Scenario**: 주문 실패 처리

```
Given 매수 주문을 제출했을 때
When API 오류가 발생하면
Then 에러를 기록해야 한다
And 최대 3회까지 재시도해야 한다
And 실패 시 Discord에 에러 알림을 전송해야 한다
```

**Feature**: Rate Limit 준수

```
Given 1초 동안 10회의 API 호출을 수행했을 때
When 11번째 호출을 시도하면
Then Rate Limit 에러를 반환해야 한다
And Exponential Backoff로 대기해야 한다
And 5초 후 재시도해야 한다
```

---

### AC-005: Risk Manager - 손절/익절

**Feature**: 익절 실행

```
Given BTC를 100,000 KRW에 매수했을 때
And 목표 수익률이 10%로 설정되어 있을 때
When 현재가가 110,000 KRW에 도달하면
Then 즉시 전량 매도 주문을 실행해야 한다
And Discord에 익절 알림을 전송해야 한다
And 포지션을 종료해야 한다
```

**Feature**: 손절 실행

```
Given BTC를 100,000 KRW에 매수했을 때
And 손절률이 5%로 설정되어 있을 때
When 현재가가 95,000 KRW로 하락하면
Then 즉시 전량 매도 주문을 실행해야 한다
And Discord에 손절 알림을 전송해야 한다 (빨간색)
And 포지션을 종료해야 한다
```

**Scenario**: 실시간 PnL 계산

```
Given 포지션이 존재할 때
When 현재가가 변경될 때마다
Then PnL(손익)을 계산해야 한다
And PnL 백분율을 기록해야 한다
```

---

### AC-006: Notification - Discord 알림

**Feature**: 매수 알림

```
Given 매수 주문이 체결되었을 때
When Notification Agent가 알림을 생성하면
Then Discord Webhook을 호출해야 한다
And Embed 메시지에 다음을 포함해야 한다:
  - 제목: "매수 체결"
  - 색상: 초록 (3066993)
  - 마켓 정보
  - 가격
  - 수량
  - 신뢰도
```

**Feature**: 매도 알림

```
Given 매도 주문이 체결되었을 때
When Notification Agent가 알림을 생성하면
Then Discord Webhook을 호출해야 한다
And Embed 메시지에 다음을 포함해야 한다:
  - 제목: "매도 체결"
  - 색상: 빨강 (15158332)
  - 마켓 정보
  - 가격
  - 수량
  - 총 수익/손실
```

**Scenario**: Discord 전송 실패

```
Given Discord Webhook URL이 잘못되었을 때
When 알림 전송을 시도하면
Then 에러를 기록해야 한다
And 주요 거래 기능은 계속 실행되어야 한다
```

---

### AC-007: State Persistence - 상태 복구

**Feature**: 포지션 저장

```
Given 포지션이 생성되었을 때
When 데이터베이스에 저장하면
Then 마켓, 진입가, 수량, 진입시간을 저장해야 한다
And SQLite 트랜잭션으로 ACID를 보장해야 한다
```

**Feature**: 시스템 재시작 후 복구

```
Given BTC 포지션을 보유 중일 때
When 시스템이 재시작되면
Then 데이터베이스에서 포지션을 복구해야 한다
And Risk Manager가 복구된 포지션을 인지해야 한다
And 손절/익절 모니터링을 재개해야 한다
```

---

### AC-008: Security - API Key 관리

**Feature**: API Key 로드

```
Given .env 파일에 UPBIT_ACCESS_KEY가 설정되어 있을 때
When 시스템이 시작되면
Then .env에서 API Key를 로드해야 한다
And 소스 코드에 API Key를 하드코딩하지 않아야 한다
```

**Feature**: 로그에서 API Key 마스킹

```
Given API 요청을 로깅할 때
When Authorization 헤더를 기록하면
Then API Key를 완전히 마스킹해야 한다 (예: "***")
And 어떤 로그에도 평문 API Key가 포함되지 않아야 한다
```

---

## 3. Non-Functional Acceptance Criteria (비기능적 인수 기준)

### AC-NFR-001: Performance

**Scenario**: 시계 처리 지연

```
Given WebSocket에서 시계 데이터를 수신했을 때
When 데이터를 처리할 때
Then 지연 시간이 100ms 이내여야 한다
```

**Scenario**: 주문 실행 지연

```
Given 매수/매도 결정이 내려졌을 때
When 주문을 제출할 때
Then API 호출까지 500ms 이내여야 한다
```

---

### AC-NFR-002: Reliability

**Scenario**: 24시간 연속 가동

```
Given 시스템이 시작되었을 때
When 24시간이 경과하면
Then 크래시 없이 계속 실행되어야 한다
And 모든 거래가 정상 처리되어야 한다
```

**Scenario**: 데이터 손실 방지

```
Given 시스템이 강제 종료될 때
When 재시작되면
Then 마지막 포지션 상태가 복구되어야 한다
And 거래 기록이 누락되지 않아야 한다
```

---

### AC-NFR-003: Observability

**Scenario**: 구조화된 로그

```
Given 시스템이 실행 중일 때
When 어떤 이벤트가 발생하면
Then JSON 형식의 구조화된 로그를 출력해야 한다
And 로그에 타임스탬프, 레벨, 메시지가 포함되어야 한다
```

**Log Format Example**:
```json
{
  "timestamp": "2026-02-06T10:30:45.123Z",
  "level": "INFO",
  "event": "order_executed",
  "market": "KRW-BTC",
  "side": "buy",
  "price": 95000000,
  "amount": 0.001,
  "order_id": "uuid-here"
}
```

---

## 4. Quality Gates (품질 게이트)

### QG-001: Code Coverage

- [ ] 단위 테스트 커버리지: 85% 이상
- [ ] 통합 테스트 커버리지: 70% 이상
- [ ] 에이전트별 테스트: 모든 에이전트 테스트 존재

### QG-002: Linting

- [ ] `cargo clippy` 통과 (warning 0개)
- [ ] `cargo fmt --check` 통과

### QG-003: Security

- [ ] API Key 하드코딩 검사 통과
- [ ] `.env`가 `.gitignore`에 포함
- [ ] 민감 정보 로깅 검사 통과

### QG-004: Documentation

- [ ] 모든 공개 함수에 doc comment 존재
- [ ] README에 실행 방법 포함
- [ ] .env.example 제공

---

## 5. Definition of Done (완료 정의)

각 Milestone은 다음 조건을 모두 충족해야 "완료"로 간주됩니다:

### DoD-Common (모든 Milestone 공통)

- [ ] 해당 Milestone의 모든 AC 통과
- [ ] 단위 테스트 작성 및 통과
- [ ] 코드 리뷰 완료
- [ ] 커밋 메시지가 Conventional Commits 준수
- [ ] 배포 가능한 상태 (컴파일 에러 없음)

### DoD-Specific (주요 Milestone)

**M1 (Foundation)**:
- [ ] 프로젝트 빌드 가능
- [ ] `cargo run`으로 실행 가능
- [ ] 로그가 정상 출력됨

**M3 (Market Monitor)**:
- [ ] Upbit WebSocket 연결 성공
- [ ] 최소 1개 코인 시계 수신

**M5 (Decision Maker)**:
- [ ] 엔드투엔드 시뮬레이션 통과
- [ ] Mock 데이터로 결정 로직 검증

**M7 (Risk Manager)**:
- [ ] 손절/익절 시뮬레이션 통과
- [ ] PnL 계산 정확성 검증

**M10 (Testing)**:
- [ ] 모든 테스트 통과 (`cargo test`)
- [ ] 커버리지 85% 달성
- [ ] 스트레스 테스트 통과

---

## 6. Test Scenarios Summary

| ID | Feature | Priority | Test Type |
|----|---------|----------|-----------|
| AC-001 | Market Monitor | High | Integration |
| AC-002 | Signal Detector | High | Unit |
| AC-003 | Decision Maker | High | Unit |
| AC-004 | Execution | High | Integration |
| AC-005 | Risk Manager | High | Unit |
| AC-006 | Notification | Medium | Integration |
| AC-007 | State Persistence | Medium | Integration |
| AC-008 | Security | High | Unit |
| AC-NFR-001 | Performance | Medium | Performance |
| AC-NFR-002 | Reliability | High | Stress |
| AC-NFR-003 | Observability | Low | Manual |

---

**Traceability**: `SPEC-ID: SPEC-TRADING-001` → Acceptance Phase Complete
