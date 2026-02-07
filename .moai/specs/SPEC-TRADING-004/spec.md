# SPEC-TRADING-004: Web-based Monitoring Dashboard for AutoCoin

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-004
Title: Web-based Monitoring Dashboard for AutoCoin
Created: 2026-02-07
Status: Planned
Priority: High
Assigned: TBD
Lifecycle: spec-anchored
Language: TypeScript, Rust
Related: SPEC-TRADING-001, SPEC-TRADING-002, SPEC-TRADING-003
```

---

## 1. Overview (개요)

### 1.1 Purpose (목적)

AutoCoin 트레이딩 시스템에 웹 기반 모니터링 대시보드를 추가하여, 브라우저를 통해 실시간으로 트레이딩 활동을 모니터링하고 제어할 수 있게 합니다. CLI 대시보드(SPEC-TRADING-002)를 보완하며, 모바일 및 원격 접속을 지원합니다.

### 1.2 Scope (범위)

**포함**:
- React 19 + Next.js 16 기반 웹 대시보드
- 실시간 가격 업데이트 (WebSocket/SSE)
- 트레이딩 상태 모니터링 페이지
- 시장 현황 페이지
- 거래 내역 페이지
- 백테스팅 설정 및 결과 페이지
- 설정/전략 파라미터 관리 페이지
- 분석 대시보드 (PnL 차트, 승률, 성과 메트릭)
- 다크 모드 지원
- 반응형 디자인 (데스크톱/모바일)

**제외**:
- 사용자 인증 시스템 (로컬 사용만을 위한 API 키 인증)
- 소셜 트레이딩 기능
- 타 거래소 연동
- 머신러닝 기반 예측 시각화

### 1.3 Target Users (대상 사용자)

- 원격으로 트레이딩 봇 모니터링이 필요한 사용자
- 모바일 기기로 상태 확인이 필요한 사용자
- CLI 대시보드보다 시각적인 인터페이스를 선호하는 사용자
- 백테스팅 결과를 시각적으로 분석하고 싶은 사용자

---

## 2. Environment (환경)

### 2.1 System Environment

| 항목 | 프론트엔드 | 백엔드 |
|------|-----------|--------|
| 구현 언어 | TypeScript 5.9+ | Rust 1.85+ |
| 프레임워크 | Next.js 16 (App Router) | Axum 0.7+ / Actix-web 4.x |
| UI 라이브러리 | shadcn/ui | - |
| 스타일링 | Tailwind CSS 3.4+ | - |
| 상태 관리 | React 19 Server Components | - |
| 차트 라이브러리 | Recharts / Chart.js | - |
| 실시간 통신 | WebSocket / SSE | Axum WebSocket / tokio-tungstenite |
| 데이터베이스 | - | SQLite (공유) |

### 2.2 Frontend Dependencies

| 의존성 | 버전 | 용도 |
|--------|------|------|
| next | 16+ | React 프레임워크 |
| react | 19+ | UI 라이브러리 |
| typescript | 5.9+ | 타입 안전성 |
| @radix-ui/* | latest | shadcn/ui 기본 컴포넌트 |
| tailwindcss | 3.4+ | 유틸리티 스타일링 |
| recharts | latest | 차트 라이브러리 |
| lucide-react | latest | 아이콘 라이브러리 |
| class-variance-authority | latest | 동적 클래스 |
| clsx | latest | 조건부 클래스 |
| tailwind-merge | latest | Tailwind 클래스 병합 |

### 2.3 Backend Dependencies (Rust)

| 의존성 | 버전 | 용도 |
|--------|------|------|
| axum | 0.7+ | 웹 서버 프레임워크 |
| axum-extra | 0.9+ | 추가 기능 (쿠키, 상태 등) |
| tokio-tungstenite | 0.24+ | WebSocket |
| tower | 0.5+ | 미들웨어 |
| tower-http | 0.5+ | CORS, tracing 등 |
| serde | 1.0 | JSON 직렬화 |
| serde_json | 1.0 | JSON 처리 |
| sqlx | 0.8+ | 데이터베이스 (공유) |

### 2.4 Browser Support

| 브라우저 | 최소 버전 |
|----------|-----------|
| Chrome | 120+ |
| Firefox | 120+ |
| Safari | 17+ |
| Edge | 120+ |
| Mobile Safari | iOS 17+ |
| Chrome Mobile | Android 120+ |

---

## 3. Assumptions (가정)

### 3.1 Technical Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| Next.js 16 App Router가 안정적임 | 높음 | Vercel 문서 확인 |
| WebSocket 연결이 방화벽 없이 동작함 | 중간 | CORS/Proxy 구현 |
| SQLite가 동시 읽기/쓰기를 지원함 | 중간 | WAL 모드 사용 |
| shadcn/ui가 다크 모드를 안정적으로 지원함 | 높음 | 실제 테스트 |
| Recharts가 실시간 데이터 업데이트를 지원함 | 높음 | Recharts 문서 확인 |

### 3.2 Business Assumptions

| 가정 | 신뢰도 | 검증 방법 |
|------|--------|-----------|
| 사용자가 웹 대시보드를 CLI보다 선호함 | 중간 | 사용자 설문 |
| 모바일 접근성이 중요한 요구사항임 | 높음 | 사용자 피드백 |
| 백테스팅 결과 시각화가 필요함 | 높음 | SPEC-TRADING-003 기반 |

### 3.3 Risk if Wrong

- **WebSocket 안정성**: 연결 불안정 시 실시간 데이터 누락 → SSE로 폴백
- **브라우저 호환성**: 일부 브라우저에서 깨짐 → Polyfill 추가
- **성능 문제**: 실시간 업데이트로 렌더링 부하 → 가상화 및 쓰로틀링

---

## 4. Requirements (요구사항 - EARS Format)

### 4.1 Ubiquitous Requirements (항시 활성 요구사항)

**REQ-401**: 시스템은 **항상** API 키를 URL 파라미터로 노출해서는 안 된다.

**REQ-402**: 시스템은 **항상** CORS 헤더를 올바르게 설정해야 한다.

**REQ-403**: 시스템은 **항상** 모든 사용자 입력을 유효성 검사해야 한다.

**REQ-404**: 시스템은 **항상** 에러 메시지를 사용자 친화적으로 표시해야 한다.

**REQ-405**: 시스템은 **항상** 다크 모드 설정을 브라우저에 저장해야 한다.

### 4.2 Event-Driven Requirements (이벤트 기반 요구사항)

**REQ-406**: **WHEN** 웹 대시보드가 접속되면 **THEN** 시스템은 WebSocket 연결을 설정해야 한다.

**REQ-407**: **WHEN** 새로운 가격 데이터가 수신되면 **THEN** 시스템은 해당 코인의 가격을 실시간으로 업데이트해야 한다.

**REQ-408**: **WHEN** 주문이 체결되면 **THEN** 시스템은 알림을 표시하고 거래 내역을 업데이트해야 한다.

**REQ-409**: **WHEN** 포지션이 생성/변경되면 **THEN** 시스템은 포지션 패널을 즉시 업데이트해야 한다.

**REQ-410**: **WHEN** 에이전트 상태가 변경되면 **THEN** 시스템은 상태 인디케이터를 업데이트해야 한다.

**REQ-411**: **WHEN** 사용자가 백테스팅을 요청하면 **THEN** 시스템은 백테스팅을 실행하고 결과를 표시해야 한다.

**REQ-412**: **WHEN** 사용자가 전략 파라미터를 변경하면 **THEN** 시스템은 변경 사항을 저장하고 트레이딩 에이전트에 전파해야 한다.

**REQ-413**: **WHEN** WebSocket 연결이 끊어지면 **THEN** 시스템은 자동으로 재연결을 시도해야 한다.

**REQ-414**: **WHEN** 다크 모드 토글이 클릭되면 **THEN** 시스템은 테마를 전환해야 한다.

**REQ-415**: **WHEN** 사용자가 수동 매수/매도를 요청하면 **THEN** 시스템은 주문을 제출하고 결과를 표시해야 한다.

**REQ-416**: **WHEN** 페이지가 로드되면 **THEN** 시스템은 현재 포지션과 잔고를 표시해야 한다.

### 4.3 State-Driven Requirements (상태 기반 요구사항)

**REQ-417**: **IF** 사용자가 모바일 기기에서 접속하면 **THEN** 시스템은 모바일 최적화 레이아웃을 표시해야 한다.

**REQ-418**: **IF** 포지션이 존재하지 않으면 **THEN** 시스템은 "No Position" 메시지를 표시해야 한다.

**REQ-419**: **IF** WebSocket 연결이 실패하면 **THEN** 시스템은 SSE로 폴백해야 한다.

**REQ-420**: **IF** 백테스팅이 진행 중이면 **THEN** 시스템은 로딩 스피너를 표시해야 한다.

**REQ-421**: **IF** 시스템이 읽기 전용 모드이면 **THEN** 시스템은 거래 버튼을 비활성화해야 한다.

**REQ-422**: **IF** PnL이 양수이면 **THEN** 시스템은 초록색으로 표시해야 한다.

**REQ-423**: **IF** PnL이 음수이면 **THEN** 시스템은 빨간색으로 표시해야 한다.

### 4.4 Unwanted Requirements (금지 사항)

**REQ-424**: 시스템은 **절대로** API 키를 브라우저 로컬 스토리지에 평문으로 저장해서는 안 된다.

**REQ-425**: 시스템은 **절대로** 사용자 확인 없이 포지션을 청산해서는 안 된다.

**REQ-426**: 시스템은 **절대로** 민감한 정보(API 키, 웹훅 URL)를 로그에 출력해서는 안 된다.

**REQ-427**: 시스템은 **절대로** 서버 사이드 렌더링 중에 WebSocket 연결을 시도해서는 안 된다.

### 4.5 Optional Requirements (선택 사항)

**REQ-428**: **가능하면** 시스템은 PWA로 설치 가능해야 한다.

**REQ-429**: **가능하면** 시스템은 푸시 알림을 지원해야 한다.

**REQ-430**: **가능하면** 시스템은 여러 테마(색상)를 제공해야 한다.

**REQ-431**: **가능하면** 시스템은 내보내기(CSV, Excel) 기능을 제공해야 한다.

---

## 5. Specifications (상세 설명)

### 5.1 System Architecture (시스템 아키텍처)

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           Web Dashboard Architecture                            │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                            Frontend (Next.js 16)                          │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │  │
│  │  │ Dashboard │  │  Markets │  │  Trades  │  │ Backtest │  │ Settings │   │  │
│  │  │   Page   │  │   Page   │  │   Page   │  │   Page   │  │   Page   │   │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │  │
│  │                                   │                                     │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐ │  │
│  │  │                   Shared Components (shadcn/ui)                    │ │  │
│  │  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐       │ │  │
│  │  │  │  Card   │ │  Badge  │ │ Button  │ │  Table  │ │  Chart  │       │ │  │
│  │  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘       │ │  │
│  │  └─────────────────────────────────────────────────────────────────────┘ │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                    │                                           │
│                          WebSocket / SSE                                       │
│                                    │                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                         Backend (Axum - Rust)                             │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │  │
│  │  │    REST  │  │WebSocket │  │   SSE    │  │  Static  │  │   API    │   │  │
│  │  │   API    │  │ Handler  │  │ Handler  │  │  Files   │  │ Gateway  │   │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘  └──────────┘   │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                    │                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                        Core Trading System                                │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │  │
│  │  │ Market  │  │ Signal  │  │Decision │  │Executor │  │  Risk   │        │  │
│  │  │ Monitor │  │Detector │  │  Maker  │  │         │  │ Manager │        │  │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘        │  │
│  │                                                                                │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐                                       │  │
│  │  │  RSI    │  │  MACD   │  │Bollinger│  (Indicators)                         │  │
│  │  └─────────┘  └─────────┘  └─────────┘                                       │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                    │                                           │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                            SQLite Database                                 │  │
│  │  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐                       │  │
│  │  │ Trades  │  │Position │  │Balance  │  │Backtest │                       │  │
│  │  └─────────┘  └─────────┘  └─────────┘  └─────────┘                       │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Page Structure (페이지 구조)

#### 5.2.1 Dashboard Page (/)

메인 대시보드 페이지로 핵심 지표와 현재 상태를 표시합니다.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AutoCoin  [Dashboard] [Markets] [Trades] [Backtest] [Settings]    [Dark Mode] │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  ┌─────────────────────────────┐  ┌─────────────────────────────────────────┐  │
│  │      Portfolio Summary      │  │         Current Position                │  │
│  │  ┌─────────────────────────┐ │  │  ┌─────────────────────────────────────┐│  │
│  │  │  Total Asset: ₩5,234,500│ │  │  │  Market: KRW-BTC                   ││  │
│  │  │  Available:  ₩5,234,500  │ │  │  │  Entry:   ₩95,000,000              ││  │
│  │  │  24h PnL:     +₩52,500   │ │  │  │  Current: ₩96,500,000  [+1.58%]   ││  │
│  │  │  Win Rate:   65%         │ │  │  │  Amount:  0.001 BTC                ││  │
│  │  │  Total Trades: 20        │ │  │  │  PnL:     +₩1,500  [+1.58%]       ││  │
│  │  └─────────────────────────┘ │  │  │  Target:  +10% (Stop Loss)         ││  │
│  │                              │  │  └─────────────────────────────────────┘│  │
│  │  [Buy BTC] [Sell All]        │  │                                      │  │
│  └─────────────────────────────┘  └─────────────────────────────────────────┘  │
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                          PnL Chart (7 Days)                                 ││
│  │  ┌───────────────────────────────────────────────────────────────────────┐ ││
│  │  │     ▲                                                                   │ ││
│  │  │     │    ╱╲                                                              │ ││
│  │  │     │   ╱  ╲     ╱╲                                                       │ ││
│  │  │     │  ╱    ╲   ╱  ╲                                                      │ ││
│  │  │     │ ╱      ╲ ╱    ╲                                                     │ ││
│  │  │     │╱        ╲╱      ╲                                                    │ ││
│  │  │     └───────────────────────────────────────▶                            │ ││
│  │  │     Mon    Tue    Wed    Thu    Fri    Sat    Sun                        │ ││
│  │  └───────────────────────────────────────────────────────────────────────┘ ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                          Agent Status                                       ││
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐                ││
│  │  │ Market Monitor │  │ Signal Detector│  │ Decision Maker │                ││
│  │  │   Running ●    │  │   Running ●    │  │   Running ●    │                ││
│  │  └────────────────┘  └────────────────┘  └────────────────┘                ││
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐                ││
│  │  │    Executor    │  │  Risk Manager  │  │  Notification  │                ││
│  │  │     Idle ○     │  │   Running ●    │  │   Running ●    │                ││
│  │  └────────────────┘  └────────────────┘  └────────────────┘                ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
│                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────┐│
│  │                          Recent Activity                                    ││
│  │  • [14:30:05] BUY signal detected: KRW-BTC (confidence: 85%)              ││
│  │  • [14:30:10] Order executed: BUY 0.001 BTC @ ₩95,000,000                  ││
│  │  • [14:31:00] Position opened: KRW-BTC @ ₩95,000,000                       ││
│  │  • [14:35:22] Price alert: KRW-BTC +1.58%                                 ││
│  └─────────────────────────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### 5.2.2 Markets Page (/markets)

모니터링 중인 코인들의 실시간 가격을 표시합니다.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AutoCoin  [Dashboard] [Markets] [Trades] [Backtest] [Settings]    [Dark Mode] │
├─────────────────────────────────────────────────────────────────────────────────┤
│  Markets  Top 20 Coins by Volume                                                │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │  Market    │ Price        │ Change  │ Volume        │ High/Low   │Signal │  │
│  ├───────────────────────────────────────────────────────────────────────────┤  │
│  │  KRW-BTC   │ ₩96,500,000 │ +1.58%  │ ₩1.2T        │ ₩97M/₩94M │  ●BUY │  │
│  │  KRW-ETH   │  ₩3,250,000 │ +0.82%  │ ₩450B        │ ₩3.3M/₩3.1M│       │  │
│  │  KRW-XRP   │       ₩650  │ -0.31%  │ ₩120B        │ ₩660/₩640  │       │  │
│  │  KRW-SOL   │    ₩125,000 │ +2.45%  │ ₩89B         │ ₩127K/₩122K│  ●BUY │  │
│  │  ...       │     ...     │   ...   │     ...      │     ...    │       │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### 5.2.3 Trades Page (/trades)

거래 내역과 필터링 기능을 제공합니다.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AutoCoin  [Dashboard] [Markets] [Trades] [Backtest] [Settings]    [Dark Mode] │
├─────────────────────────────────────────────────────────────────────────────────┤
│  Trade History                                                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │  Filters: [All▼] [Date Range▼] [Coin▼] [Search...]                       │  │
│  │  ┌─────────────────────────────────────────────────────────────────────┐ │  │
│  │  │ Time       │ Market │ Side  │ Price      │ Amount   │ PnL       │Status│ │  │
│  │  ├─────────────────────────────────────────────────────────────────────┤ │  │
│  │  │ 14:30:10  │BTC    │ BUY   │ ₩95,000,000│ 0.001 BTC│ ₩0       │Executed│ │  │
│  │  │ 13:15:22  │ETH    │ SELL  │  ₩3,200,000│ 0.05 ETH │ +₩5,000  │Executed│ │  │
│  │  │ 12:00:00  │XRP    │ BUY   │       ₩660 │ 100 XRP  │ ₩0       │Executed│ │  │
│  │  │ 10:30:45  │SOL    │ SELL  │    ₩120,000│ 0.1 SOL  │ -₩2,500  │Executed│ │  │
│  │  └─────────────────────────────────────────────────────────────────────┘ │  │
│  │  Pagination: ◀ 1 2 3 4 5 ▶                                               │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### 5.2.4 Backtest Page (/backtest)

백테스팅 설정과 결과를 표시합니다.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AutoCoin  [Dashboard] [Markets] [Trades] [Backtest] [Settings]    [Dark Mode] │
├─────────────────────────────────────────────────────────────────────────────────┤
│  Backtesting                                                                   │
│  ┌─────────────────────────────────┐  ┌───────────────────────────────────────┐ │
│  │  Configuration                  │  │  Results                             │ │
│  │  ┌─────────────────────────────┐ │  │  ┌─────────────────────────────────┐ │ │
│  │  │ Market:     [BTC ▼]         │ │  │  │  Total Return:   +12.5%        │ │ │
│  │  │ Strategy:   [Multi ▼]       │ │  │  │  Win Rate:       65%           │ │ │
│  │  │ Date Range: [Last 30 Days▼] │ │  │  │  Total Trades:   20            │ │ │
│  │  │ Initial:    [₩1,000,000]    │ │  │  │  Max Drawdown:   -3.2%         │ │ │
│  │  │ Commission: [0.05%]         │ │  │  │  Sharpe Ratio:   1.8           │ │ │
│  │  │                             │ │  │  │                                 │ │ │
│  │  │  [Run Backtest]              │ │  │  │  ┌─────────────────────────────┐ │ │ │
│  │  └─────────────────────────────┘ │  │  │  │    Equity Curve             │ │ │ │
│  │                                   │  │  │  │    ┌───┐                   │ │ │ │
│  │  Parameter Optimization:          │  │  │  │    │   ╱╲                  │ │ │ │
│  │  [ ] Enable Grid Search           │  │  │  │    │  ╱  ╲    ╱╲           │ │ │ │
│  │  RSI Period: [10-20]              │  │  │  │    │ ╱    ╲  ╱  ╲          │ │ │ │
│  │  MACD Fast:  [8-15]               │  │  │  │    │╱      ╲╱    ╲         │ │ │ │
│  │  [Optimize]                       │  │  │  │    └─────────────────────── │ │ │ │
│  │                                   │  │  │  └─────────────────────────────┘ │ │ │
│  └─────────────────────────────────┘  └───────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────────┘
```

#### 5.2.5 Settings Page (/settings)

전략 파라미터와 시스템 설정을 관리합니다.

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  AutoCoin  [Dashboard] [Markets] [Trades] [Backtest] [Settings]    [Dark Mode] │
├─────────────────────────────────────────────────────────────────────────────────┤
│  Settings                                                                      │
│  ┌─────────────────────────────────┐  ┌───────────────────────────────────────┐ │
│  │  Strategy Configuration         │  │  Risk Management                    │ │
│  │  ┌─────────────────────────────┐ │  │  ┌─────────────────────────────────┐ │ │
│  │  │ Active Strategy:            │ │  │  │ Stop Loss:        [5%]         │ │ │
│  │  │ [●] Momentum Following      │ │  │  │ Take Profit:      [10%]        │ │ │
│  │  │ [ ] Multi-Indicator         │ │  │  │ Max Position:     [1]          │ │ │
│  │  │ [ ] RSI Strategy            │ │  │  │ Max Position Ratio:[50%]       │ │ │
│  │  │                             │ │  │  │ Trailing Stop:    [Off ▼]      │ │ │
│  │  │ Target Coins: [20]          │ │  │  │                                 │ │ │
│  │  │ Surge Threshold: [5%]       │ │  │  │  [Save Risk Settings]            │ │ │
│  │  │ Volume Multiplier: [2.0]    │ │  │  └─────────────────────────────────┘ │ │
│  │  │                             │ │  │                                       │ │
│  │  │  [Save Strategy Settings]    │ │  │  System Status                      │ │
│  │  └─────────────────────────────┘ │  │  ┌─────────────────────────────────┐ │ │
│  │                                   │  │  │ Trading Bot:     [Running ●]   │ │ │
│  │  Indicators                      │  │  │  │ Uptime:          5d 12h 30m   │ │ │
│  │  ┌─────────────────────────────┐ │  │  │  │ API Calls Today:  1,234       │ │ │
│  │  │ [●] RSI    Period: [14]     │ │  │  │  │ Last Error:      None         │ │ │
│  │  │ [●] MACD   Fast: [12]       │ │  │  │  │                                 │ │ │
│  │  │         Slow: [26] Sig: [9] │ │  │  │  │  [Pause Trading] [Shutdown]    │ │ │
│  │  │ [ ] Bollinger Period: [20]  │ │  │  └─────────────────────────────────┘ │ │
│  │  │ [ ] SMA    Short: [5]       │ │  │                                       │ │
│  │  │         Long:  [20]         │ │  │                                       │ │
│  │  │  [Save Indicator Settings]   │ │  │                                       │ │
│  │  └─────────────────────────────┘ │  │                                       │ │
│  └─────────────────────────────────┘  └───────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 5.3 Component Library (shadcn/ui)

#### 5.3.1 Required Components

| 컴포넌트 | 용도 |
|----------|------|
| Card | 패널, 카드 레이아웃 |
| Badge | 상태 인디케이터, 신호 |
| Button | 액션 버튼 |
| Table | 거래 내역, 시장 데이터 |
| Chart | PnL 차트, 가격 차트 |
| Progress | 백테스팅 진행률 |
| Tabs | 페이지 내 섹션 전환 |
| Dialog | 확인 대화상자 |
| Form Input | 설정 값 입력 |
| Switch | 토글 스위치 |
| Select | 드롭다운 선택 |

#### 5.3.2 Custom Components

```typescript
// PriceBadge.tsx - 가장 변동 폭을 색상으로 표시
interface PriceBadgeProps {
  change: number;
  children: React.ReactNode;
}

// PositionCard.tsx - 포지션 요약 카드
interface PositionCardProps {
  position: Position | null;
  onClose?: () => void;
}

// TradeTable.tsx - 거래 내역 테이블
interface TradeTableProps {
  trades: Trade[];
  filter?: TradeFilter;
}

// AgentStatusGrid.tsx - 에이전트 상태 그리드
interface AgentStatusGridProps {
  agents: AgentStatus[];
}

// PnLChart.tsx - PnL 차트 컴포넌트
interface PnLChartProps {
  data: PnLDataPoint[];
  timeframe: '1D' | '7D' | '30D' | 'ALL';
}
```

### 5.4 Real-time Communication (실시간 통신)

#### 5.4.1 WebSocket Protocol

**Client → Server:**

```json
{
  "type": "subscribe",
  "channels": ["prices", "trades", "position", "agents"]
}
```

**Server → Client:**

```json
{
  "type": "price_update",
  "data": {
    "market": "KRW-BTC",
    "price": 96500000,
    "change": 0.0158,
    "timestamp": 1675790400000
  }
}
```

#### 5.4.2 Event Types

| 이벤트 타입 | 설명 | 데이터 |
|------------|------|--------|
| `price_update` | 가격 업데이트 | PriceTick |
| `trade_executed` | 주문 체결 | Trade |
| `position_update` | 포지션 변경 | Position |
| `agent_status` | 에이전트 상태 | AgentStatus |
| `notification` | 알림 | Notification |
| `backtest_progress` | 백테스팅 진행률 | Progress |
| `backtest_complete` | 백테스팅 완료 | BacktestResult |

#### 5.4.3 SSE Fallback

WebSocket 연결이 실패할 경우 SSE로 폴백합니다.

```typescript
// SSE 구현 예시
const eventSource = new EventSource('/api/events');

eventSource.addEventListener('price_update', (e) => {
  const data = JSON.parse(e.data);
  updatePrice(data);
});
```

### 5.5 REST API Design (REST API 설계)

#### 5.5.1 Endpoints

| 메서드 | 경로 | 설명 |
|--------|------|------|
| GET | /api/health | 헬스 체크 |
| GET | /api/balance | 잔고 조회 |
| GET | /api/position | 현재 포지션 조회 |
| GET | /api/trades | 거래 내역 조회 |
| GET | /api/markets | 시장 데이터 조회 |
| POST | /api/orders | 수동 주문 |
| DELETE | /api/position | 포지션 청산 |
| GET | /api/agents/status | 에이전트 상태 조회 |
| POST | /api/backtest | 백테스팅 실행 |
| GET | /api/backtest/:id | 백테스팅 결과 조회 |
| GET | /api/settings | 설정 조회 |
| PUT | /api/settings | 설정 업데이트 |
| POST | /api/trading/pause | 트레이딩 일시정지 |
| POST | /api/trading/resume | 트레이딩 재개 |

#### 5.5.2 Request/Response Examples

**GET /api/balance**
```json
// Response
{
  "krw_balance": 5234500,
  "available": 5234500,
  "locked": 0,
  "total_asset_value": 5234500
}
```

**POST /api/orders**
```json
// Request
{
  "market": "KRW-BTC",
  "side": "buy",
  "amount_krw": 500000
}

// Response
{
  "order_id": "uuid",
  "status": "submitted",
  "message": "Order submitted successfully"
}
```

### 5.6 Backend Integration (백엔드 연동)

#### 5.6.1 Axum Server Setup

```rust
use axum::{
    routing::{get, post, put, delete},
    Router, Json, extract::ws::{WebSocket, Message},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        // REST API
        .route("/api/health", get(health_check))
        .route("/api/balance", get(get_balance))
        .route("/api/position", get(get_position))
        .route("/api/trades", get(get_trades))
        .route("/api/markets", get(get_markets))
        .route("/api/orders", post(create_order))
        .route("/api/backtest", post(run_backtest))
        .route("/api/settings", get(get_settings).put(update_settings))
        // WebSocket
        .route("/ws", get(websocket_handler))
        // SSE
        .route("/api/events", get(sse_handler))
        // CORS
        .layer(tower_http::cors::CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

#### 5.6.2 WebSocket Handler

```rust
async fn websocket_handler(ws: WebSocket) {
    let (mut sender, mut receiver) = ws.split();

    // 클라이언트로부터 메시지 수신
    while let Some(Ok(message)) = receiver.next().await {
        match message {
            Message::Text(text) => {
                // 구독 요청 처리
                if let Ok(req) = serde_json::from_str::<SubscribeRequest>(&text) {
                    subscribe_channels(&req.channels);
                }
            }
            _ => {}
        }
    }

    // 실시간 데이터 전송
    // ... 데이터 브로드캐스트 로직
}
```

#### 5.6.3 Integration with Core System

| Core Component | Integration Method |
|----------------|-------------------|
| Market Monitor | 상태 → WebSocket broadcast |
| Signal Detector | 신호 → WebSocket broadcast |
| Decision Maker | 결정 → WebSocket broadcast |
| Executor | 주문 결과 → WebSocket broadcast |
| Risk Manager | 포지션 → WebSocket broadcast |
| Notification Agent | 알림 → WebSocket broadcast |
| SQLite DB | REST API로 조회 |

### 5.7 Security (보안)

#### 5.7.1 API Key Protection

```rust
// API 키 검증 미들웨어
pub async fn api_key_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = req.headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if verify_api_key(api_key).await {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
```

#### 5.7.2 CORS Configuration

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin(Any) // 개발 환경
    // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap()) // 프로덕션
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any);
```

### 5.8 Dark Mode Implementation (다크 모드 구현)

#### 5.8.1 Theme Provider

```typescript
// app/providers/theme-provider.tsx
'use client';

import { createContext, useContext, useEffect, useState } from 'react';

type Theme = 'dark' | 'light';

const ThemeContext = createContext<{
  theme: Theme;
  toggleTheme: () => void;
}>({ theme: 'dark', toggleTheme: () => {} });

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setTheme] = useState<Theme>('dark');

  useEffect(() => {
    const stored = localStorage.getItem('theme') as Theme;
    if (stored) setTheme(stored);
  }, []);

  useEffect(() => {
    document.documentElement.classList.remove('light', 'dark');
    document.documentElement.classList.add(theme);
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggleTheme = () => setTheme(t => t === 'dark' ? 'light' : 'dark');

  return (
    <ThemeContext.Provider value={{ theme, toggleTheme }}>
      {children}
    </ThemeContext.Provider>
  );
}

export const useTheme = () => useContext(ThemeContext);
```

### 5.9 Responsive Design (반응형 디자인)

#### 5.9.1 Breakpoints

| 브레이크포인트 | 너비 | 대상 |
|---------------|------|------|
| sm | 640px | 모바일 (가로) |
| md | 768px | 태블릿 |
| lg | 1024px | 노트북 |
| xl | 1280px | 데스크톱 |
| 2xl | 1536px | 대형 화면 |

#### 5.9.2 Mobile Adaptations

```typescript
// 모바일에서 카드를 세로로 배치
<div className="
  grid grid-cols-1
  md:grid-cols-2
  lg:grid-cols-3
  gap-4
">
  <Card>...</Card>
</div>

// 모바일에서 테이블을 카드로 변환
<div className="block lg:hidden">
  {trades.map(trade => (
    <TradeCard key={trade.id} trade={trade} />
  ))}
</div>
<table className="hidden lg:table">
  ...
</table>
```

---

## 6. Non-Functional Requirements (비기능 요구사항)

### 6.1 Performance

| 항목 | 목표 |
|------|------|
| 페이지 로드 시간 | < 500ms (FCP) |
| 첫 번째 콘텐츠 페인트 | < 200ms |
| 시간 대 대화형 지연 | < 100ms |
| WebSocket 메시지 지연 | < 50ms |
| 차트 렌더링 | < 200ms (데이터 1000개) |

### 6.2 Reliability

| 항목 | 목표 |
|------|------|
| WebSocket 재연결 | < 5초 |
| API 응답 시간 | < 200ms (P95) |
| 가동률 | 99.5% |
| 오류 복구 | 자동 재연결 |

### 6.3 Security

| 항목 | 조치 |
|------|------|
| API 키 전송 | 헤더 기반 인증 |
| HTTPS | 필수 (프로덕션) |
| XSS 방지 | React 자동 이스케이프 |
| CSRF 방지 | SameSite 쿠키 |
| 콘텐츠 보안 정책 | CSP 헤더 |

### 6.4 Accessibility (접근성)

| 항목 | 목표 |
|------|------|
| WCAG 레벨 | 2.1 Level AA |
| 색상 대비 | 4.5:1 (최소) |
| 키보드 내비게이션 | 전체 기능 지원 |
| 스크린 리더 | 주요 컴포넌트 지원 |

### 6.5 Browser Compatibility

| 항목 | 지원 |
|------|------|
| 최신 브라우저 | Chrome, Firefox, Safari, Edge |
| 모바일 브라우저 | iOS Safari, Chrome Mobile |
| 레거시 브라우저 | 지원 안 함 |

---

## 7. Traceability (추적성)

| REQ | 관련 컴포넌트 | 테스트 시나리오 |
|-----|--------------|----------------|
| REQ-401 | API Middleware | api_key_exposure_test |
| REQ-402 | CORS Layer | cors_configuration_test |
| REQ-403 | Form Components | input_validation_test |
| REQ-404 | Error Boundary | error_message_display_test |
| REQ-405 | ThemeProvider | dark_mode_persistence_test |
| REQ-406 | WebSocket Client | ws_connection_test |
| REQ-407 | Price Components | price_update_test |
| REQ-408 | Trade Components | order_notification_test |
| REQ-409 | Position Components | position_update_test |
| REQ-410 | Agent Components | agent_status_update_test |
| REQ-411 | Backtest Page | backtest_execution_test |
| REQ-412 | Settings Page | parameter_update_test |
| REQ-413 | WebSocket Client | ws_reconnect_test |
| REQ-414 | ThemeProvider | theme_toggle_test |
| REQ-415 | Manual Trade | manual_order_test |
| REQ-416 | Dashboard Page | initial_data_load_test |
| REQ-417 | Layout Components | mobile_layout_test |
| REQ-418 | Position Components | no_position_display_test |
| REQ-419 | WebSocket Client | sse_fallback_test |
| REQ-420 | Backtest Page | loading_spinner_test |
| REQ-421 | Trade Components | readonly_mode_test |
| REQ-422 | PnL Components | positive_pnl_color_test |
| REQ-423 | PnL Components | negative_pnl_color_test |
| REQ-424 | API Middleware | api_key_storage_protection_test |
| REQ-425 | Position Components | position_close_confirmation_test |
| REQ-426 | Logging | sensitive_info_logging_test |
| REQ-427 | App Router | ssr_ws_prevention_test |
| REQ-428 | PWA Config | pwa_install_test |
| REQ-429 | Notification API | push_notification_test |
| REQ-430 | ThemeProvider | theme_options_test |
| REQ-431 | Export Components | csv_export_test |

---

**Traceability**: `SPEC-ID: SPEC-TRADING-004` → Specification Phase Complete
