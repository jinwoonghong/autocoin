# SPEC-TRADING-002: Acceptance Criteria

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-002
Related: spec.md, plan.md
Last Updated: 2026-02-07
```

---

## 1. Acceptance Criteria Overview

본 문서는 SPEC-TRADING-002의 요구사항을 검증하기 위한 인수 테스트 기준을 정의합니다. 모든 시나리오는 Given-When-Then (Gherkin) 형식으로 작성되었습니다.

---

## 2. Dashboard Launch Scenarios

### Scenario 2.1: TUI Dashboard 시작

**Given**: 사용자가 AutoCoin이 설치된 시스템에 있음
**And**: `.env` 파일이 유효한 Upbit API 키를 포함함
**When**: 사용자가 `autocoin --dashboard` 명령을 실행함
**Then**: 터미널에 TUI 대시보드가 표시됨
**And**: 4개의 패널 (Status, Position, Balance, Market)이 보임
**And**: 헤더에 버전 정보와 현재 시간이 표시됨
**And**: 모든 에이전트가 "Running" 상태로 표시됨

### Scenario 2.2: 백그라운드 데몬 모드 시작

**Given**: 사용자가 AutoCoin이 설치된 시스템에 있음
**When**: 사용자가 `autocoin --daemon` 명령을 실행함
**Then**: TUI가 표시되지 않음
**And**: 백그라운드에서 트레이딩 시스템이 실행됨
**And**: 로그가 표준 출력 또는 지정된 로그 파일에 기록됨

### Scenario 2.3: 터미널 크기 부족 - Compact 모드

**Given**: 사용자의 터미널 크기가 80x24 미만임
**When**: 사용자가 `autocoin --dashboard` 명령을 실행함
**Then**: Compact 모드가 활성화됨
**And**: 패널이 최소화된 형태로 표시됨
**And**: "Compact Mode: Terminal too small" 메시지가 표시됨

---

## 3. Real-Time Update Scenarios

### Scenario 3.1: 에이전트 상태 변경 반영

**Given**: TUI 대시보드가 실행 중임
**When**: Market Monitor 에이전트가 "Running"에서 "Error" 상태로 변경됨
**Then**: Status 패널의 Market Monitor 행이 "Error"로 업데이트됨
**And**: 에러 상태가 빨간색으로 표시됨
**And**: 업데이트가 1초 이내에 반영됨

### Scenario 3.2: 포지션 정보 표시

**Given**: TUI 대시보드가 실행 중임
**When**: 사용자가 KRW-BTC 포지션을 보유함
**Then**: Position 패널에 다음이 표시됨:
  - Market: KRW-BTC
  - Entry Price: 진입 가격
  - Current Price: 현재 가격
  - Amount: 보유 수량
  - PnL: 현재 손익 (% 및 KRW)
**And**: PnL이 양수이면 초록색, 음수이면 빨간색으로 표시됨

### Scenario 3.3: 포지션 없을 때 표시

**Given**: TUI 대시보드가 실행 중임
**When**: 사용자가 어떠한 포지션도 보유하지 않음
**Then**: Position 패널에 "No Position" 메시지가 표시됨
**And**: PnL 관련 정보가 표시되지 않음

### Scenario 3.4: WebSocket 연결 끊김 표시

**Given**: TUI 대시보드가 실행 중임
**When**: Upbit WebSocket 연결이 끊어짐
**Then**: Status 패널의 Market Monitor가 "Disconnected"로 표시됨
**And**: 상태가 주황색 또는 빨간색으로 표시됨
**And**: 마지막 연결 시간이 표시됨

### Scenario 3.5: 주문 체결 알림

**Given**: TUI 대시보드가 실행 중임
**When**: 매수 주문이 체결됨
**Then**: Notifications 패널에 체결 알림이 추가됨
**And**: 알림에는 시간, 주문 종류, 코인, 가격 정보가 포함됨
**And**: 최신 알림이 목록 상단에 표시됨

---

## 4. Input Handling Scenarios

### Scenario 4.1: 'q' 키로 정상 종료

**Given**: TUI 대시보드가 실행 중임
**When**: 사용자가 'q' 키를 누름
**Then**: "Shutting down..." 메시지가 표시됨
**And**: 열린 포지션이 안전하게 보존됨
**And**: 데이터베이스에 상태가 저장됨
**And**: 프로그램이 정상 종료됨 (exit code 0)

### Scenario 4.2: 'p' 키로 일시정지

**Given**: TUI 대시보드가 실행 중임
**When**: 사용자가 'p' 키를 누름
**Then**: 트레이딩 활동이 일시중지됨
**And**: Status 패널에 "Paused" 상태가 표시됨
**And**: 노란색으로 일시정지 상태가 표시됨

### Scenario 4.3: 'r' 키로 재개

**Given**: TUI 대시보드가 일시정지 상태임
**When**: 사용자가 'r' 키를 누름
**Then**: 트레이딩 활동이 재개됨
**And**: Status 패널에 "Running" 상태가 표시됨
**And**: 초록색으로 실행 상태가 표시됨

### Scenario 4.4: '?' 키로 도움말

**Given**: TUI 대시보드가 실행 중임
**When**: 사용자가 '?' 키를 누름
**Then**: 키 단축기 도움말 팝업이 표시됨
**And**: 다시 '?' 또는 ESC를 누르면 도움말이 닫힘

---

## 5. Performance Scenarios

### Scenario 5.1: UI가 트레이딩 루프를 차단하지 않음

**Given**: 트레이딩 시스템이 활발하게 실행 중임
**And**: TUI 대시보드가 실행 중임
**When**: UI가 1초 간격으로 업데이트됨
**Then**: 트레이딩 루프가 지연되지 않음
**And**: 주문 실행 지연이 < 500ms로 유지됨

### Scenario 5.2: 대시보드 메모리 사용량

**Given**: TUI 대시보드가 1시간 이상 실행 중임
**When**: 시스템 메모리 사용량을 측정함
**Then**: 대시보드 프로세스의 메모리 사용량이 < 50MB임
**And**: 메모리 누수가 관찰되지 않음

### Scenario 5.3: 렌더링 CPU 사용량

**Given**: TUI 대시보드가 실행 중임
**When**: UI 렌더링 중 CPU 사용량을 측정함
**Then**: 단일 코어 CPU 사용량이 < 5%임

---

## 6. Watchdog Scenarios

### Scenario 6.1: 프로세스 크래시 후 자동 재시작

**Given**: AutoCoin이 워치독과 함께 실행 중임
**When**: AutoCoin 프로세스가 예기치 않게 종료됨
**Then**: 워치독이 종료를 감지함
**And**: 10초 이내에 프로세스가 재시작됨
**And**: 재시작 횟수가 로그에 기록됨

### Scenario 6.2: 최대 재시도 횟수 초과

**Given**: AutoCoin이 워치독과 함께 실행 중임
**When**: AutoCoin 프로세스가 3회 연속으로 크래시함
**Then**: 워치독이 추가 재시작을 중단함
**And**: "Max restart attempts reached" 메시지가 기록됨
**And**: Discord 알림이 전송됨 (설정된 경우)

### Scenario 6.3: PC 재부팅 후 자동 시작 (Linux)

**Given**: AutoCoin이 systemd에 등록되어 있음
**When**: 시스템이 재부팅됨
**Then**: 부팅 완료 후 AutoCoin 서비스가 자동 시작됨
**And**: `systemctl status autocoin`이 "active (running)"을 표시함

### Scenario 6.4: PC 재부팅 후 자동 시작 (Windows)

**Given**: AutoCoin이 Task Scheduler에 등록되어 있음
**When**: 시스템이 재부팅됨
**Then**: 부팅 완료 후 AutoCoin이 자동 시작됨
**And**: 작업 관리자에 프로세스가 표시됨

---

## 7. Error Handling Scenarios

### Scenario 7.1: API 키 마스킹

**Given**: TUI 대시보드가 실행 중임
**When**: 디버그 정보에 API 키가 포함될 수 있음
**Then**: 대시보드에 API 키가 평문으로 표시되지 않음
**And**: 마스킹된 형식 (예: `up***_key***`)으로 표시됨

### Scenario 7.2: 터미널 크기 변경 처리

**Given**: TUI 대시보드가 실행 중임
**When**: 사용자가 터미널 창 크기를 조정함
**Then**: 대시보드가 새 크기에 맞게 재배열됨
**And**: 충돌 없이 정상적으로 계속 실행됨

### Scenario 7.3: UI 렌더링 실패

**Given**: TUI 대시보드가 실행 중임
**When**: 터미널이 UI 렌더링을 지원하지 않음
**Then**: UI가 비활성화됨
**And**: 로그-only 모드로 전환됨
**And**: 트레이딩 기능이 계속 실행됨

---

## 8. Quality Gates

### 8.1 Functional Requirements

| 요구사항 | 검증 방법 | 통과 기준 |
|----------|-----------|-----------|
| REQ-104~105 | CLI 테스트 | `--dashboard`, `--daemon` 플래그 정상 작동 |
| REQ-106 | 실시간 업데이트 테스트 | 상태 변경이 1초 이내 반영 |
| REQ-107 | 포지션 패널 테스트 | PnL 정확히 계산 및 표시 |
| REQ-108 | 종료 테스트 | 'q' 키로 정상 종료 |
| REQ-111~112 | 워치독 테스트 | 크래시 후 자동 재시작 |

### 8.2 Non-Functional Requirements

| 항목 | 측정 방법 | 통과 기준 |
|------|-----------|-----------|
| UI 갱신 주기 | 측정 | 1-2초 |
| 메모리 오버헤드 | 프로파일링 | < 50MB |
| CPU 사용량 | 프로파일링 | < 5% |
| 키 입력 응답 | 측정 | < 100ms |

### 8.3 Code Quality

| 항목 | 도구 | 통과 기준 |
|------|------|-----------|
| Lint | clippy | 0 warnings |
| Format | rustfmt | 0 errors |
| Tests | cargo test | All pass |
| Coverage | tarpaulin | > 80% |

---

## 9. Test Execution Plan

### Phase 1: 단위 테스트

```bash
# 대시보드 모듈 단위 테스트
cargo test dashboard::

# CLI 모듈 단위 테스트
cargo test cli::

# 워치독 모듈 단위 테스트
cargo test execution::watchdog
```

### Phase 2: 통합 테스트

```bash
# 전체 통합 테스트
cargo test --test integration

# 데이터 채널 통합 테스트
cargo test test_agent_to_dashboard_channel
```

### Phase 3: 수동 테스트 체크리스트

- [ ] TUI가 다양한 터미널에서 정상 표시됨 (Terminal.app, iTerm2, Windows Terminal, Alacritty)
- [ ] 'q' 키로 정상 종료됨
- [ ] 백그라운드 모드에서 TUI가 표시되지 않음
- [ ] 포지션 PnL이 정확하게 계산됨
- [ ] 터미널 크기 조정 시 재배열됨
- [ ] 크래시 후 워치독이 재시작함
- [ ] systemd 서비스가 부팅 시 시작됨

---

## 10. Sign-off Criteria

SPEC-TRADING-002이 완료되었다고 승인하기 위해:

1. **모든 필수 시나리오 통과**: Scenario 2.1 ~ 7.3 (선택 사항 제외)
2. **성능 기준 충족**: 모든 NFR 측정값이 목표치 이내
3. **코드 품질**: clippy 0 warnings, rustfmt 통과
4. **테스트 커버리지**: > 80%
5. **문서화**: README.md에 새로운 실행 옵션 설명 포함
6. **수동 테스트**: Phase 3 체크리스트 모든 항목 통과

---

## 11. Test Data Samples

### 테스트용 포지션 데이터

```rust
let test_position = PositionData {
    market: "KRW-BTC".to_string(),
    entry_price: 95_000_000.0,
    current_price: 96_500_000.0,
    amount: 0.001,
    pnl_percent: 1.58,
    pnl_krw: 1_500.0,
    target_percent: 10.0,
    stop_loss_percent: -5.0,
};
```

### 테스트용 에이전트 상태

```rust
let test_agent_states = HashMap::from([
    ("Market Monitor".to_string(), AgentState::Running {
        last_update: Utc::now(),
    }),
    ("Signal Detector".to_string(), AgentState::Running {
        last_update: Utc::now(),
    }),
    ("Executor".to_string(), AgentState::Idle {
        since: Utc::now() - Duration::from_secs(300),
    }),
]);
```
