# SPEC-TRADING-004: Implementation Plan

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-004
Related: spec.md, acceptance.md
Phase: Plan
Status: Planned
```

---

## 1. Milestones by Priority

### Primary Goal (MVP - Minimum Viable Product)

핵심 기능을 구현하여 웹 대시보드의 기본적인 모니터링 기능을 제공합니다.

- 기본 대시보드 페이지 (포지션, 잔고, 에이전트 상태)
- 실시간 가격 업데이트 (WebSocket)
- REST API 기본 엔드포인트
- 다크 모드 지원
- 반응형 레이아웃 (데스크톱/모바일)

### Secondary Goal (Core Features)

핵심 거래 관리 기능을 추가합니다.

- 시장 페이지 (실시간 가격 테이블)
- 거래 내역 페이지 (필터링, 정렬)
- 설정 페이지 (전략 파라미터 수정)
- 수동 매수/매도 기능
- PnL 차트 시각화

### Final Goal (Advanced Features)

고급 분석 및 백테스팅 기능을 추가합니다.

- 백테스팅 페이지 (설정, 실행, 결과)
- 성과 메트릭 시각화
- 파라미터 최적화 UI
- 내보내기 기능 (CSV)

### Optional Goal (Enhancements)

선택적인 향상 기능입니다.

- PWA 지원
- 푸시 알림
- 추가 테마 옵션
- 차트 고급 기능 (줌, 팬)

---

## 2. Technical Approach

### 2.1 Architecture Decisions

#### Frontend Architecture

```
web-dashboard/
├── app/                          # Next.js 16 App Router
│   ├── layout.tsx               # 루트 레이아웃
│   ├── page.tsx                 # 대시보드 페이지
│   ├── markets/
│   │   └── page.tsx             # 시장 페이지
│   ├── trades/
│   │   └── page.tsx             # 거래 내역 페이지
│   ├── backtest/
│   │   └── page.tsx             # 백테스팅 페이지
│   ├── settings/
│   │   └── page.tsx             # 설정 페이지
│   ├── api/                     # API Routes (선택)
│   └── providers/
│       ├── theme-provider.tsx   # 테마 제공자
│       └── websocket-provider.tsx # WebSocket 제공자
├── components/
│   ├── ui/                      # shadcn/ui 컴포넌트
│   │   ├── button.tsx
│   │   ├── card.tsx
│   │   ├── badge.tsx
│   │   ├── table.tsx
│   │   └── ...
│   ├── dashboard/               # 대시보드 컴포넌트
│   │   ├── portfolio-summary.tsx
│   │   ├── position-card.tsx
│   │   ├── agent-status-grid.tsx
│   │   └── pnl-chart.tsx
│   ├── markets/                 # 시장 컴포넌트
│   │   ├── price-table.tsx
│   │   └── price-badge.tsx
│   ├── trades/                  # 거래 컴포넌트
│   │   ├── trade-table.tsx
│   │   └── trade-filter.tsx
│   ├── backtest/                # 백테스팅 컴포넌트
│   │   ├── backtest-form.tsx
│   │   ├── backtest-results.tsx
│   │   └── equity-chart.tsx
│   └── settings/                # 설정 컴포넌트
│       ├── strategy-form.tsx
│       ├── risk-form.tsx
│       └── system-status.tsx
├── lib/
│   ├── api.ts                   # API 클라이언트
│   ├── websocket.ts             # WebSocket 클라이언트
│   ├── types.ts                 # 공유 타입
│   └── utils.ts                 # 유틸리티
└── styles/
    └── globals.css              # Tailwind CSS 설정
```

#### Backend Architecture (Rust)

```
autocoin/
├── src/
│   ├── web/                     # 웹 서버 모듈 (신규)
│   │   ├── mod.rs
│   │   ├── server.rs            # Axum 서버 설정
│   │   ├── routes/              # REST API 라우트
│   │   │   ├── mod.rs
│   │   │   ├── balance.rs
│   │   │   ├── position.rs
│   │   │   ├── trades.rs
│   │   │   ├── markets.rs
│   │   │   ├── orders.rs
│   │   │   ├── backtest.rs
│   │   │   └── settings.rs
│   │   ├── websocket/           # WebSocket 핸들러
│   │   │   ├── mod.rs
│   │   │   ├── handler.rs
│   │   │   └── broadcast.rs
│   │   ├── sse/                 # SSE 핸들러
│   │   │   ├── mod.rs
│   │   │   └── handler.rs
│   │   ├── middleware/          # 미들웨어
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   └── cors.rs
│   │   └── types.rs             # 웹 전용 타입
│   └── ...
└── web-static/                  # 정적 파일 (빌드 결과)
```

### 2.2 Technology Stack Justification

| 기술 | 선택 이유 | 대안 |
|------|----------|------|
| Next.js 16 | React Server Components, App Router, 최신 기능 | Remix, Vite+React |
| shadcn/ui | Radix UI 기반, 완전한 커스터마이징, Tailwind 통합 | MUI, Chakra UI |
| Tailwind CSS | 유틸리티 우선, 빠른 개발, 다크 모드 지원 | CSS Modules, Styled Components |
| Recharts | React 친화적, 선언적 API | Chart.js, Victory |
| Axum | Rust 생태계 표준, 타입 안전성, 비동기 지원 | Actix-web, Rocket |
| WebSocket | 양방향 실시간 통신, 낮은 지연 | SSE, Polling |

### 2.3 Data Flow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           Data Flow Diagram                                    │
└─────────────────────────────────────────────────────────────────────────────────┘

[User Action] → [Frontend] → [REST API] → [Core System] → [Database] → [Response]
                      ↓
                 [WebSocket] ← [Event Broadcast] ← [Core System]
                      ↓
                 [UI Update]

┌─────────────────────────────────────────────────────────────────────────────────┐
│                              Real-time Updates                                 │
└─────────────────────────────────────────────────────────────────────────────────┘

[Core System] → [Event Channel] → [WebSocket Handler] → [Broadcast] → [Client]
                                                      ↓
                                                 [UI Re-render]
```

### 2.4 State Management Strategy

#### Client-side State

```typescript
// React 19 Server Components + use hook
// 대부분의 상태는 서버 컴포넌트에서 처리
// 실시간 상태만 use 훅으로 관리

// Real-time state
interface DashboardState {
  position: Position | null;
  prices: Map<string, number>;
  agentStatus: AgentStatus[];
  notifications: Notification[];
}

// Zustand for global state (선택)
interface GlobalStore {
  theme: 'light' | 'dark';
  sidebarOpen: boolean;
  toggleTheme: () => void;
}
```

#### Server-side State

```rust
// Rust 기존 시스템 활용
// SQLite DB + 채널 기반 상태 공유

// 웹 서버는 기존 시스템과 채널로 통신
let (web_tx, web_rx) = mpsc::channel::<WebEvent>(100);

// 에이전트 상태 변경 → 웹 브로드캐스트
tokio::spawn(async move {
    while let Some(event) = web_rx.recv().await {
        broadcast_to_all_clients(event).await;
    }
});
```

### 2.5 API Versioning

- 초기 버전: `/api/v1/*`
- 버전은 헤더 또는 URL 경로로 관리
- 주요 변경 시에만 버전 업데이트

---

## 3. Implementation Phases

### Phase 1: Project Setup (Foundation)

**작업 항목:**
1. Next.js 16 프로젝트 생성 (app/src/web-dashboard)
2. TypeScript 설정
3. Tailwind CSS + shadcn/ui 초기화
4. Axum 웹 서버 기본 구조 생성
5. CORS 및 기본 미들웨어 설정

**완료 기준:**
- Next.js 개발 서버 실행 가능
- Axum 서버가 `/api/health` 응답
- 정적 파일 서빙 가능

### Phase 2: Core Components (Primary Goal)

**작업 항목:**
1. 기본 레이아웃 (헤더, 사이드바, 다크 모드)
2. Dashboard 페이지 구현
3. REST API 기본 엔드포인트 (balance, position, trades)
4. WebSocket 핸들러 구현
5. 실시간 가격 업데이트 연동

**완료 기준:**
- 대시보드에서 포지션과 잔고 표시
- 실시간 가격 업데이트 작동
- 다크 모드 토글 가능

### Phase 3: Pages & Features (Secondary Goal)

**작업 항목:**
1. Markets 페이지 구현
2. Trades 페이지 구현
3. Settings 페이지 구현
4. 수동 매수/매도 기능
5. PnL 차트 구현 (Recharts)

**완료 기준:**
- 모든 페이지 접근 가능
- 필터링 및 정렬 작동
- 설정 변경이 시스템에 반영

### Phase 4: Backtesting & Analytics (Final Goal)

**작업 항목:**
1. Backtest 페이지 구현
2. 백테스팅 API 연동
3. 성과 메트릭 시각화
4. 내보내기 기능 (CSV)

**완료 기준:**
- 백테스팅 실행 및 결과 표시
- 차트로 결과 시각화
- CSV 다운로드 가능

### Phase 5: Polish & Deployment (Optional Goal)

**작업 항목:**
1. PWA 설정
2. 푸시 알림 (선택)
3. 성능 최적화
4. 접근성 개선
5. 프로덕션 빌드 및 배포

**완료 기준:**
- PWA 설치 가능
- Lighthouse 점수 90+
- 접근성 표준 준수

---

## 4. Dependencies and Integration

### 4.1 Integration with Existing SPECs

| SPEC | 통합 방법 |
|------|-----------|
| SPEC-TRADING-001 | 코어 트레이딩 시스템 상태 조회, WebSocket 브로드캐스트 |
| SPEC-TRADING-002 | CLI 대시보드와 동일한 데이터 소스 사용 |
| SPEC-TRADING-003 | 백테스팅 API 연동, 지표 데이터 시각화 |

### 4.2 Shared Components

```
[SQLite Database] ← 공유 데이터 소스
        ↓
[Core Agents] ← 상태 생산자
        ↓
[WebSocket Broadcast] ← 상태 소비자 및 브로드캐스트
        ↓
[Web Dashboard] ← 상태 표시
[CLI Dashboard] ← 상태 표시
```

### 4.3 Code Organization

**Rust Backend:**
- `src/web/` 디렉토리 생성
- `src/agents/`와 통신을 위한 채널 추가
- 기존 `src/db/` 재사용

**Frontend:**
- `web-dashboard/` 디렉토리 (프로젝트 루트)
- 또는 `src/web/` 내부 (단일 바이너리)

---

## 5. Risks and Response Plans

### 5.1 Technical Risks

| 리스크 | 확률 | 영향 | 완화 방안 |
|--------|------|------|-----------|
| WebSocket 연결 불안정 | 중간 | 높음 | SSE 폴백, 자동 재연결 |
| 실시간 데이터 렌더링 부하 | 낮음 | 중간 | 가상화, 쓰로틀링 |
| CORS 문제 | 중간 | 낮음 | 적절한 CORS 설정 |
| 모바일 성능 문제 | 낮음 | 중간 | 코드 분할, 지연 로딩 |

### 5.2 Development Risks

| 리스크 | 확률 | 영향 | 완화 방안 |
|--------|------|------|-----------|
| 타입 불일치 (Rust ↔ TypeScript) | 중간 | 중간 | 공유 타입 정의, 자동 생성 |
| UI/UX 변경 요청 | 높음 | 낮음 | shadcn/ui로 빠른 수정 |
| API 설계 변경 | 중간 | 중간 | 버전 관리, 유연한 설계 |

### 5.3 Response Plans

**WebSocket 연결 실패:**
1. SSE로 자동 폴백
2. 사용자에게 연결 상태 표시
3. 재연결 버튼 제공

**타입 불일치:**
1. OpenAPI/Swagger 명세 작성
2. 타입 자동 생성 (orval, openapi-typescript)
3. 통합 테스트로 검증

---

## 6. Testing Strategy

### 6.1 Frontend Testing

- **Unit Tests**: 컴포넌트 단위 테스트 (Vitest)
- **Integration Tests**: 페이지 간 네비게이션 (Playwright)
- **E2E Tests**: 사용자 시나리오 (Playwright)

### 6.2 Backend Testing

- **Unit Tests**: 핸들러 단위 테스트
- **Integration Tests**: API 엔드포인트 테스트
- **WebSocket Tests**: 실시간 통신 테스트

### 6.3 Performance Testing

- **Lighthouse**: CI에서 자동 실행
- **Load Testing**: WebSocket 동시 접속 테스트

---

## 7. Deployment Plan

### 7.1 Development Environment

```bash
# Frontend
cd web-dashboard
npm run dev  # http://localhost:3000

# Backend
cargo run --bin web-server  # http://localhost:8080
```

### 7.2 Production Build

**Frontend:**
```bash
npm run build
# Output: .next/ 디렉토리
```

**Backend:**
```bash
cargo build --release
# Output: target/release/web-dashboard
```

### 7.3 Deployment Options

| 옵션 | 장점 | 단점 |
|------|------|------|
| 단일 바이너리 (정적 파일 포함) | 배포 간단 | 파일 변경 시 재빌드 |
| 분리 배치 (Next.js + Rust) | 독립적 업데이트 | 인프라 복잡 |
| Docker 컨테이너 | 환경 일관성 | 이미지 크기 |

---

**Traceability**: `SPEC-ID: SPEC-TRADING-004` → Plan Phase Complete
