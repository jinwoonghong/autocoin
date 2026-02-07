# SPEC-TRADING-004: Acceptance Criteria

## TAG BLOCK

```
SPEC-ID: SPEC-TRADING-004
Related: spec.md, plan.md
Phase: Acceptance
Status: Planned
```

---

## 1. Definition of Done

### 1.1 General Criteria

- [ ] 모든 필수 기능이 구현되어 있음
- [ ] 모든 테스트 케이스가 통과함
- [ ] Lighthouse 성능 점수가 90점 이상임
- [ ] 모든 페이지가 데스크톱과 모바일에서 정상 작동함
- [ ] 다크 모드가 모든 페이지에서 작동함
- [ ] WebSocket/SSE가 안정적으로 연결됨
- [ ] API 문서가 작성되어 있음

### 1.2 Quality Gates

- [ ] Zero TypeScript errors
- [ ] Zero ESLint warnings
- [ ] 85%+ test coverage
- [ ] WCAG 2.1 Level AA 준수
- [ ] OWASP Top 10 보안 체크리스트 통과

---

## 2. Test Scenarios (Given-When-Then Format)

### 2.1 Dashboard Page

#### Scenario 2.1.1: 대시보드 초기 로드

**Given** 사용자가 웹 대시보드에 접속할 때
**When** 페이지가 로드되면
**Then** 다음이 표시되어야 함:
- 포트폴리오 요약 (총 자산, 가용 금액, 24h PnL)
- 현재 포지션 (또는 "No Position" 메시지)
- 에이전트 상태 그리드
- 최근 활동 로그

#### Scenario 2.1.2: 실시간 가격 업데이트

**Given** 사용자가 대시보드를 보고 있을 때
**When** 새로운 가격 데이터가 수신되면
**Then** 해당 코인의 가격이 즉시 업데이트되어야 함
**And** 변경률에 따라 색상이 표시되어야 함 (양수: 초록, 음수: 빨강)

#### Scenario 2.1.3: 포지션 PnL 표시

**Given** 사용자가 포지션을 보유하고 있을 때
**When** 대시보드를 보면
**Then** 현재 PnL이 표시되어야 함
**And** PnL이 양수이면 초록색으로 표시되어야 함
**And** PnL이 음수이면 빨간색으로 표시되어야 함

#### Scenario 2.1.4: 에이전트 상태 모니터링

**Given** 사용자가 대시보드를 보고 있을 때
**When** 에이전트 상태가 변경되면
**Then** 상태 인디케이터가 즉시 업데이트되어야 함
**And** Running 상태는 초록색 점으로 표시되어야 함
**And** Idle 상태는 노란색 점으로 표시되어야 함
**And** Error 상태는 빨간색 점으로 표시되어야 함

### 2.2 Markets Page

#### Scenario 2.2.1: 시장 페이지 로드

**Given** 사용자가 Markets 페이지에 접속할 때
**When** 페이지가 로드되면
**Then** 모니터링 중인 코인 목록이 표시되어야 함
**And** 각 코인의 가격, 변동률, 거래량이 표시되어야 함

#### Scenario 2.2.2: 실시간 가격 업데이트 (Markets)

**Given** 사용자가 Markets 페이지를 보고 있을 때
**When** 새로운 가격 데이터가 수신되면
**Then** 해당 코인의 가격 행이 하이라이트되어야 함
**And** 가격과 변동률이 업데이트되어야 함

#### Scenario 2.2.3: 모바일 레이아웃

**Given** 사용자가 모바일 기기로 Markets 페이지에 접속할 때
**When** 페이지가 로드되면
**Then** 카드 형태로 코인 정보가 표시되어야 함
**And** 테이블 대신 세로 스크롤이 가능해야 함

### 2.3 Trades Page

#### Scenario 2.3.1: 거래 내역 조회

**Given** 사용자가 Trades 페이지에 접속할 때
**When** 페이지가 로드되면
**Then** 최근 거래 내역이 표시되어야 함
**And** 각 거래의 시간, 마켓, 방향, 가격, 수량, PnL이 표시되어야 함

#### Scenario 2.3.2: 필터링 기능

**Given** 사용자가 Trades 페이지를 보고 있을 때
**When** 특정 코인을 선택하여 필터링하면
**Then** 해당 코인의 거래 내역만 표시되어야 함

#### Scenario 2.3.3: 날짜 범위 필터

**Given** 사용자가 Trades 페이지를 보고 있을 때
**When** 날짜 범위를 선택하면
**Then** 해당 기간의 거래 내역만 표시되어야 함

### 2.4 Backtest Page

#### Scenario 2.4.1: 백테스팅 설정

**Given** 사용자가 Backtest 페이지에 접속할 때
**When** 백테스팅 설정을 입력하고 실행하면
**Then** 백테스팅이 시작되어야 함
**And** 로딩 스피너가 표시되어야 함

#### Scenario 2.4.2: 백테스팅 결과 표시

**Given** 백테스팅이 완료되었을 때
**When** 결과가 도착하면
**Then** 총 수익률, 승률, 최대 낙폭, 샤프 비율이 표시되어야 함
**And** 자산 곡선(Equity Curve)이 차트로 표시되어야 함

#### Scenario 2.4.3: 백테스팅 진행률

**Given** 백테스팅이 진행 중일 때
**When** 사용자가 페이지를 보면
**Then** 진행률 바가 표시되어야 함
**And** 완료된 시뮬레이션 횟수가 표시되어야 함

### 2.5 Settings Page

#### Scenario 2.5.1: 전략 파라미터 변경

**Given** 사용자가 Settings 페이지에 접속할 때
**When** 전략 파라미터를 변경하고 저장하면
**Then** 변경 사항이 시스템에 적용되어야 함
**And** 성공 메시지가 표시되어야 함

#### Scenario 2.5.2: 리스크 관리 설정

**Given** 사용자가 Settings 페이지를 보고 있을 때
**When** 손절/익절 비율을 변경하면
**Then** Risk Manager 에이전트가 새로운 파라미터를 사용해야 함

#### Scenario 2.5.3: 트레이딩 일시정지

**Given** 사용자가 Settings 페이지를 보고 있을 때
**When** "Pause Trading" 버튼을 클릭하면
**Then** 트레이딩이 일시중지되어야 함
**And** 상태가 "Paused"로 표시되어야 함

### 2.6 Real-time Features

#### Scenario 2.6.1: WebSocket 연결

**Given** 사용자가 웹 대시보드에 접속할 때
**When** 페이지가 로드되면
**Then** WebSocket 연결이 자동으로 설정되어야 함
**And** 연결 상태가 표시되어야 함

#### Scenario 2.6.2: WebSocket 재연결

**Given** 사용자가 대시보드를 보고 있을 때
**When** WebSocket 연결이 끊어지면
**Then** 자동으로 재연결을 시도해야 함
**And** 연결 실패 시 SSE로 폴백해야 함

#### Scenario 2.6.3: 주문 체결 알림

**Given** 사용자가 대시보드를 보고 있을 때
**When** 주문이 체결되면
**Then** 알림 메시지가 표시되어야 함
**And** 거래 내역이 업데이트되어야 함

### 2.7 Dark Mode

#### Scenario 2.7.1: 다크 모드 토글

**Given** 사용자가 라이트 모드를 사용하고 있을 때
**When** 다크 모드 버튼을 클릭하면
**Then** 테마가 다크 모드로 변경되어야 함
**And** 설정이 브라우저에 저장되어야 함

#### Scenario 2.7.2: 다크 모드 지속성

**Given** 사용자가 다크 모드로 설정했을 때
**When** 페이지를 새로고침하면
**Then** 다크 모드가 유지되어야 함

### 2.8 Mobile Responsiveness

#### Scenario 2.8.1: 모바일 레이아웃

**Given** 사용자가 모바일 기기로 접속할 때
**When** 페이지가 로드되면
**Then** 모바일에 최적화된 레이아웃이 표시되어야 함
**And** 모든 기능이 접근 가능해야 함

#### Scenario 2.8.2: 모바일 터치

**Given** 사용자가 모바일 기기를 사용할 때
**When** 버튼을 탭하면
**Then** 적절한 터치 영역이 제공되어야 함 (최소 44x44px)

### 2.9 Manual Trading

#### Scenario 2.9.1: 수동 매수

**Given** 사용자가 대시보드를 보고 있을 때
**When** "Buy" 버튼을 클릭하고 금액을 입력하면
**Then** 주문이 제출되어야 함
**And** 주문 결과가 표시되어야 함

#### Scenario 2.9.2: 포지션 청산

**Given** 사용자가 포지션을 보유하고 있을 때
**When** "Sell All" 버튼을 클릭하면
**Then** 확인 대화상자가 표시되어야 함
**And** 확인 후 포지션이 청산되어야 함

### 2.10 Security

#### Scenario 2.10.1: API 키 보호

**Given** 사용자가 웹 대시보드를 사용할 때
**When** 네트워크 요청을 확인하면
**Then** API 키가 URL 파라미터에 포함되지 않아야 함
**And** API 키가 로컬 스토리지에 평문으로 저장되지 않아야 함

#### Scenario 2.10.2: 입력 유효성 검사

**Given** 사용자가 설정 페이지에서 값을 입력할 때
**When** 잘못된 값을 입력하면
**Then** 유효성 검사 오류가 표시되어야 함
**And** 잘못된 값이 저장되지 않아야 함

---

## 3. Quality Metrics

### 3.1 Performance Metrics

| 메트릭 | 목표 | 측정 방법 |
|--------|------|-----------|
| First Contentful Paint (FCP) | < 200ms | Lighthouse |
| Largest Contentful Paint (LCP) | < 500ms | Lighthouse |
| Time to Interactive (TTI) | < 500ms | Lighthouse |
| Cumulative Layout Shift (CLS) | < 0.1 | Lighthouse |
| First Input Delay (FID) | < 100ms | Lighthouse |
| WebSocket Message Latency | < 50ms | Custom |

### 3.2 Accessibility Metrics

| 메트릭 | 목표 | 측정 방법 |
|--------|------|-----------|
| WCAG 2.1 Level AA | 100% | Axe DevTools |
| Color Contrast | 4.5:1 | Lighthouse |
| Keyboard Navigation | 100% | Manual Test |
| Screen Reader | Compatible | NVDA/JAWS |

### 3.3 Code Quality Metrics

| 메트릭 | 목표 | 측정 방법 |
|--------|------|-----------|
| TypeScript Coverage | 100% | tsc --noEmit |
| Test Coverage | 85%+ | Vitest/Istanbul |
| ESLint Warnings | 0 | ESLint |
| Bundle Size (Initial) | < 200KB | webpack-bundle-analyzer |

---

## 4. Verification Methods

### 4.1 Automated Tests

```typescript
// Vitest 단위 테스트 예시
describe('PriceBadge', () => {
  it('renders green color for positive change', () => {
    render(<PriceBadge change={1.5}>₩96,500,000</PriceBadge>);
    expect(screen.getByText('₩96,500,000')).toHaveClass('text-green-500');
  });

  it('renders red color for negative change', () => {
    render(<PriceBadge change={-1.5}>₩94,000,000</PriceBadge>);
    expect(screen.getByText('₩94,000,000')).toHaveClass('text-red-500');
  });
});
```

```typescript
// Playwright E2E 테스트 예시
test('dashboard loads and displays portfolio', async ({ page }) => {
  await page.goto('http://localhost:3000');
  await expect(page.locator('text=Total Asset')).toBeVisible();
  await expect(page.locator('text=KRW-BTC')).toBeVisible();
});
```

### 4.2 Manual Tests

| 테스트 | 수행자 | 빈도 |
|--------|--------|------|
| 스모크 테스트 | QA | 매 배포 |
| 회귀 테스트 | QA | 주 1회 |
| 사용자 테스트 | 실제 사용자 | 스프린트 종료 시 |

### 4.3 Security Tests

```bash
# OWASP ZAP 자동 스캔
zap-baseline.py -t http://localhost:3000

# Nmap 포트 스캔
nmap -sV localhost
```

---

## 5. Rollout Criteria

### 5.1 Alpha Release (Internal)

- [ ] 모든 Primary Goal 기능 구현
- [ ] 단위 테스트 통과
- [ ] 로컬 환경에서 정상 작동

### 5.2 Beta Release (Limited Users)

- [ ] 모든 Secondary Goal 기능 구현
- [ ] E2E 테스트 통과
- [ ] 5명 이상의 내부 테스터 검증

### 5.3 Production Release

- [ ] 모든 Final Goal 기능 구현
- [ ] 성능 메트릭 목표 달성
- [ ] 보안 감사 통과
- [ ] 접근성 표준 준수

---

**Traceability**: `SPEC-ID: SPEC-TRADING-004` → Acceptance Phase Complete
