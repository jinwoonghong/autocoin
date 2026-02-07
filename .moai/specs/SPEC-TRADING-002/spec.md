# SPEC-TRADING-002: CLI Dashboard and 24h Local Execution

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-002
Title: CLI Dashboard and 24h Local Execution
Created: 2026-02-07
Status: Planned
Priority: High
Assigned: TBD
Lifecycle: spec-anchored
Language: Rust
Related: SPEC-TRADING-001
```

---

## 1. Overview (개요)

### 1.1 Purpose (목적)

AutoCoin 트레이딩 시스템에 터미널 기반 대시보드와 24시간 상시 실행 지원 기능을 추가하여, 사용자가 실시간으로 시스템 상태를 모니터링하고 PC 항시 켜짐 시나리오에서 안정적으로 운영할 수 있도록 합니다.

### 1.2 Scope (범위)

**포함**:
- ratatui 기반 TUI (Terminal User Interface) 대시보드
- 실시간 에이전트 상태 모니터링
- 포지션/잔고 시각화
- 백그라운드 데몬 모드 지원
- 워치독(Watchdog) 기반 자동 재시작
- PC 재부팅 후 자동 시작 (Task Scheduler)

**제외**:
- 웹 기반 대시보드
- 원격 모니터링 (별도 SPEC)
- 모바일 앱 지원

### 1.3 Target Use Cases (대상 사용 시나리오)

- 개인 투자자가 항상 켜진 PC에서 트레이딩 봇 운영
- 실시간 상태 모니터링이 필요한 사용자
- SSH 원격 접속으로 상태 확인이 필요한 사용자

---

## 2. Environment (환경)

### 2.1 System Environment

| 항목 | 내용 |
|------|------|
| 구현 언어 | Rust 1.85+ |
| TUI 프레임워크 | ratatui 0.28+ |
| 터미널 처리 | crossterm 0.28+ |
| 대상 OS | Linux, macOS, Windows |
| 데몬 지원 | systemd (Linux), launchd (macOS), Task Scheduler (Windows) |

### 2.2 New Dependencies

| 의존성 | 버전 | 용도 |
|--------|------|------|
| ratatui | 0.28+ | TUI 렌더링 |
| crossterm | 0.28+ | 크로스 플랫폼 터미널 처리 |
| chrono | 0.4+ | 시간 표시 (기존 의존성) |

### 2.3 Terminal Requirements

| 항목 | 최소 사양 |
|------|-----------|
| 터미널 크기 | 80x24 문자 (추천: 120x40) |
| 색상 지원 | 256색 (필수), True Color (권장) |
| UTF-8 | 필수 |

---

## 3. Assumptions (가정)

### 3.1 Technical Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| ratatui가 크로스 플랫폼 TUI를 안정적으로 제공함 | 높음 | ratatui 문서 및 실제 테스트 |
| 터미널이 ANSI 이스케이프 시퀀스를 지원함 | 높음 | 현대 터미널 표준 |
| 1초 간격 UI 업데이트가 트레이딩 루프에 영향을 주지 않음 | 중간 | 비동기 채널 격리 검증 |
| 사용자 PC가 24시간 켜져 있음을 보장할 수 있음 | 중간 | 불가능, 워치독으로 완화 |

### 3.2 Business Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| 터미널 대시보드로도 실시간 모니터링이 충분함 | 높음 | 사용자 설문 |
| 백그라운드 실행이 일반 사용자 시나리오임 | 중간 | Discord 알림으로 대체 가능 |
| 워치독이 시스템 안정성을 크게 향상시킴 | 높음 | 크래시 복구 테스트 |

### 3.3 Risk if Wrong

- **UI 성능**: 트레이딩 루프 차단 시 손실 발생 → 비동기 채널로 완화
- **터미널 호환성**: 일부 터미널에서 깨짐 → Compact 모드로 대응

---

## 4. Requirements (요구사항 - EARS Format)

### 4.1 Ubiquitous Requirements (항시 활성 요구사항)

**REQ-101**: 시스템은 **항상** 트레이딩 루프가 UI 렌더링에 의해 차단되지 않아야 한다.

**REQ-102**: 시스템은 **항상** UI 상태를 실제 상태와 비동기로 동기화해야 한다.

**REQ-103**: 시스템은 **항상** 에이전트 상태 변경을 채널을 통해 대시보드에 전달해야 한다.

### 4.2 Event-Driven Requirements (이벤트 기반 요구사항)

**REQ-104**: **WHEN** 시스템이 `--dashboard` 플래그로 시작되면 **THEN** 터미널 TUI를 표시해야 한다.

**REQ-105**: **WHEN** 시스템이 `--daemon` 플래그로 시작되면 **THEN** 백그라운드 모드로 실행해야 한다.

**REQ-106**: **WHEN** 에이전트 상태가 변경되면 **THEN** 대시보드를 실시간으로 업데이트해야 한다.

**REQ-107**: **WHEN** 포지션이 존재하면 **THEN** 현재 PnL (손익)을 표시해야 한다.

**REQ-108**: **WHEN** 사용자가 'q' 키를 누르면 **THEN** 정상 종료 절차를 실행해야 한다.

**REQ-109**: **WHEN** WebSocket 연결이 끊어지면 **THEN** 대시보드에 에러 상태를 표시해야 한다.

**REQ-110**: **WHEN** 주문이 체결되면 **THEN** 대시보드 알림 영역에 메시지를 표시해야 한다.

**REQ-111**: **WHEN** PC가 재부팅되면 **THEN** Task Scheduler/Systemd를 통해 자동으로 재시작해야 한다.

**REQ-112**: **WHEN** 프로세스가 크래시되면 **THEN** 워치독이 자동으로 재시작해야 한다.

### 4.3 State-Driven Requirements (상태 기반 요구사항)

**REQ-113**: **IF** 터미널 크기가 최소 크기(80x24)보다 작으면 **THEN** Compact 모드를 표시해야 한다.

**REQ-114**: **IF** 터미널이 True Color를 지원하면 **THEN** 24비트 색상을 사용해야 한다.

**REQ-115**: **IF** 포지션이 없으면 **THEN** 포지션 패널에 "No Position"을 표시해야 한다.

**REQ-116**: **IF** 데몬 모드에서 실행 중이면 **THEN** TUI를 비활성화하고 로그만 출력해야 한다.

### 4.4 Unwanted Requirements (금지 사항)

**REQ-117**: 시스템은 **절대로** UI 렌더링을 위해 트레이딩 루프를 블록해서는 안 된다.

**REQ-118**: 시스템은 **절대로** API 키를 대시보드에 평문으로 표시해서는 안 된다.

**REQ-119**: 시스템은 **절대로** 터미널 크기 조정 중에 충돌해서는 안 된다.

**REQ-120**: 시스템은 **절대로** UI 업데이트 실패로 인해 트레이딩 기능이 중단되어서는 안 된다.

### 4.5 Optional Requirements (선택 사항)

**REQ-121**: **가능하면** 시스템은 대화형 키 입력을 지원해야 한다 (p: 일시정지, r: 재개).

**REQ-122**: **가능하면** 시스템은 히스토리 그래프를 ASCII 아트로 표시해야 한다.

**REQ-123**: **가능하면** 시스템은 다크 모드/라이트 모드 전환을 지원해야 한다.

---

## 5. Specifications (상세 설명)

### 5.1 Dashboard Layout (대시보드 레이아웃)

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  AutoCoin Trading Bot v0.1.0                     2026-02-07 14:32:15 KST   │
├──────────────────────────────────────────────────────────────────────────────┤
│  ┌─ Status ─────────────────────────────────┐  ┌─ Position ──────────────┐ │
│  │  Agent            State    Last Update   │  │  Market: KRW-BTC        │ │
│  │  ─────────────────────────────────────   │  │  Entry: 95,000,000      │ │
│  │  Market Monitor    Running    1s ago     │  │  Current: 96,500,000    │ │
│  │  Signal Detector   Running    1s ago     │  │  Amount: 0.001 BTC      │ │
│  │  Decision Maker    Running    1s ago     │  │  PnL: +1.58%  +1,500KRW │ │
│  │  Executor          Idle       5m ago     │  │                         │ │
│  │  Risk Manager      Running    1s ago     │  │  Target: +10% (Stop)    │ │
│  │  Notification      Running    1s ago     │  │  StopLoss: -5% (Safe)   │ │
│  └──────────────────────────────────────────┘  └─────────────────────────┘ │
│                                                                              │
│  ┌─ Balance ──────────────────────────────────┐  ┌─ Market ────────────────┐ │
│  │  KRW Balance: 5,234,500                    │  │  Top 20 Coins Monitor   │ │
│  │  Available:  5,234,500                     │  │  ─────────────────────  │ │
│  │  Locked:     0                             │  │  BTC:  96,500,000 +1.5% │ │
│  │  Total Asset Value: 5,234,500              │  │  ETH:  3,250,000  +0.8% │ │
│  │                                            │  │  XRP:  650       -0.3% │ │
│  │  Today's PnL:   +52,500 (+1.01%)           │  │  ... (scrollable)      │ │
│  └────────────────────────────────────────────┘  └─────────────────────────┘ │
│                                                                              │
│  ┌─ Notifications ──────────────────────────────────────────────────────────┐ │
│  │  [14:30:05] BUY signal detected: KRW-BTC (confidence: 85%)              │ │
│  │  [14:30:10] Order executed: BUY 0.001 BTC @ 95,000,000                  │ │
│  │  [14:31:00] Position opened: KRW-BTC @ 95,000,000                       │ │
│  │  [14:35:22] Price alert: KRW-BTC +1.58%                                 │ │
│  └──────────────────────────────────────────────────────────────────────────┘ │
│                                                                              │
│  [q] Quit  [p] Pause  [r] Resume  [?] Help                                  │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Four-Panel Design

| 패널 | 설명 | 업데이트 주기 |
|------|------|--------------|
| **Status** | 에이전트별 실행 상태 | 1초 |
| **Position** | 현재 포지션 및 PnL | 1초 |
| **Balance** | 계정 잔고 및 자산 가치 | 5초 |
| **Market** | 모니터링 중인 코인 목록 | 2초 |
| **Notifications** | 최근 알림/이벤트 로그 | 실시간 |

### 5.3 Data Sharing Architecture (데이터 공유 아키텍처)

```rust
// 에이전트 → 대시보드 데이터 채널
use tokio::sync::mpsc;

struct DashboardData {
    agent_states: HashMap<String, AgentState>,
    position: Option<PositionData>,
    balance: BalanceData,
    market_prices: Vec<CoinPrice>,
    notifications: Vec<Notification>,
}

// 채널을 통한 비동기 통신
let (data_tx, mut data_rx) = mpsc::channel::<DashboardData>(100);

// 에이전트가 상태 변경 시 전송
data_tx.send(DashboardData::new_state(...)).await;

// 대시보드는 수신한 데이터로만 UI 업데이트
```

### 5.4 Input Handling (입력 처리)

| 키 | 동작 | 설명 |
|----|------|------|
| `q` | 종료 | Graceful shutdown |
| `p` | 일시정지 | 트레이딩 일시 중지 |
| `r` | 재개 | 트레이딩 재개 |
| `?` | 도움말 | 키 바로가기 표시 |
| `Ctrl+C` | 강제 종료 | Emergency stop |

### 5.5 CLI Arguments (CLI 인자)

```bash
# TUI 대시보드 모드 (기본)
autocoin --dashboard

# 백그라운드 데몬 모드
autocoin --daemon

# 로그 레벨 지정
autocoin --dashboard --log-level debug

# 설정 파일 지정
autocoin --config ./config/custom.toml --dashboard

# 백그라운드 모드 + 로그 파일
autocoin --daemon --log-file ./logs/autocoin.log
```

### 5.6 Watchdog Design (워치독 설계)

#### Linux Systemd Service

```ini
[Unit]
Description=AutoCoin Trading Bot
After=network.target

[Service]
Type=simple
User=autocoin
WorkingDirectory=/opt/autocoin
ExecStart=/opt/autocoin/target/release/autocoin --daemon
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

#### Windows Task Scheduler

```xml
<Task>
  <Triggers>
    <BootTrigger />
  </Triggers>
  <Actions>
    <Exec>
      <Command>C:\autocoin\target\release\autocoin.exe</Command>
      <Arguments>--daemon --log-file C:\autocoin\logs\autocoin.log</Arguments>
    </Exec>
  </Actions>
  <Settings>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <RestartOnFailure>
      <Interval>PT1M</Interval>
      <Count>3</Count>
    </RestartOnFailure>
  </Settings>
</Task>
```

### 5.7 Color Scheme (색상 구성)

| 요소 | 색상 | 용도 |
|------|------|------|
| Running | Green | 정상 실행 중 |
| Idle | Yellow | 대기 상태 |
| Error | Red | 에러 상태 |
| Positive PnL | Green | 수익 |
| Negative PnL | Red | 손실 |
| Border | Blue | 패널 테두리 |
| Header | Cyan | 헤더 영역 |

---

## 6. Non-Functional Requirements (비기능 요구사항)

### 6.1 Performance

| 항목 | 목표 |
|------|------|
| UI 갱신 주기 | 1-2초 |
| 대시보드 메모리 오버헤드 | < 50MB |
| 렌더링 CPU 사용량 | < 5% |
| 키 입력 응답 지연 | < 100ms |

### 6.2 Reliability

| 항목 | 목표 |
|------|------|
| 워치독 재시작 시간 | < 10초 |
| UI 충돌로 인한 트레이딩 영향 | 0% |
| 터미널 크기 변경 복구 | 100% |

### 6.3 Usability

| 항목 | 목표 |
|------|------|
| 최소 터미널 크기 | 80x24 |
| 권장 터미널 크기 | 120x40 |
| 컬러 미지원 터미널 지원 | Yes (mono mode) |

### 6.4 Observability

| 항목 | 구현 |
|------|------|
| 상태 표시 | 모든 에이전트 |
| 로그 출력 | 데몬 모드에서만 |
| 통계 | 일일/전체 PnL |

---

## 7. Traceability (추적성)

| REQ | 관련 컴포넌트 | 테스트 시나리오 |
|-----|--------------|----------------|
| REQ-101 | Dashboard Controller | ui_blocking_test |
| REQ-102 | Data Channel | async_sync_test |
| REQ-103 | Agent→Channel | state_propagation_test |
| REQ-104 | CLI Parser | dashboard_flag_test |
| REQ-105 | CLI Parser | daemon_flag_test |
| REQ-106 | UI Renderer | realtime_update_test |
| REQ-107 | Position Panel | pnl_display_test |
| REQ-108 | Input Handler | quit_key_test |
| REQ-109 | Status Panel | ws_error_display_test |
| REQ-110 | Notification Panel | order_notification_test |
| REQ-111 | Systemd/Task Scheduler | auto_restart_test |
| REQ-112 | Watchdog | crash_recovery_test |
| REQ-113 | Layout Manager | compact_mode_test |
| REQ-114 | Color Scheme | truecolor_test |
| REQ-115 | Position Panel | no_position_test |
| REQ-116 | Main | daemon_no_ui_test |
| REQ-117 | Dashboard Controller | blocking_prevention_test |
| REQ-118 | UI Renderer | api_key_masking_test |
| REQ-119 | Terminal Handler | resize_handling_test |
| REQ-120 | Error Handler | ui_failure_isolation_test |

---

## 8. Dependencies (의존성)

### 8.1 External Dependencies

```
[dependencies]
# TUI Framework
ratatui = "0.28"
crossterm = "0.28"

# 기존 의존성 (SPEC-TRADING-001)
tokio = { version = "1.40", features = ["full"] }
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "chrono"] }
# ... (기존 의존성 유지)
```

### 8.2 Integration with SPEC-TRADING-001

| SPEC-TRADING-001 컴포넌트 | SPEC-TRADING-002 연동 |
|--------------------------|---------------------|
| Market Monitor Agent | 상태 → Data Channel |
| Signal Detector Agent | 상태 → Data Channel |
| Decision Maker Agent | 상태 → Data Channel |
| Executor Agent | 주문 결과 → Notification |
| Risk Manager Agent | 포지션 → Position Panel |
| Notification Agent | 알림 → Notification Panel |
| SQLite DB | 잔고 조회 |

---

## 9. Milestones (마일스톤)

### Phase 1: 기본 TUI 구조 (Primary Goal)

- 프로젝트에 ratatui, crossterm 추가
- 기본 4패널 레이아웃 구현
- 더미 데이터로 UI 렌더링

### Phase 2: 에이전트 연동 (Primary Goal)

- Data Channel 구현
- 에이전트 상태 전송
- 실시간 UI 업데이트

### Phase 3: 백그라운드 모드 (Secondary Goal)

- `--daemon` 플래그 구현
- TUI 비활성화 로직
- 로그 파일 출력

### Phase 4: 운영 안정성 (Final Goal)

- Systemd service 파일 작성
- Windows Task Scheduler 스크립트 작성
- 워치독 구현

### Phase 5: 고급 기능 (Optional Goal)

- 히스토리 그래프
- 다크/라이트 모드
- 대화형 키 기능 확장
