# Acceptance Criteria: SPEC-TRADING-003

**TAG BLOCK**: `SPEC-ID: SPEC-TRADING-003`

---

## 1. Overview (개요)

본 문서는 Advanced Trading Strategies for AutoCoin의 인수 기준을 정의합니다. 모든 기능은 Gherkin 형식(Given-When-Then)으로 명시되어 있으며, 각 기능은 자동화된 테스트로 검증될 수 있습니다.

---

## 2. Functional Acceptance Criteria (기능적 인수 기준)

### AC-301: RSI Indicator

**Feature**: RSI 지표 계산

**Scenario**: RSI가 정확하게 계산되어야 한다

```
Given 14개의 종가 데이터가 있을 때
And RSI 지표가 생성되었을 때
When RSI를 계산하면
Then 표준 공식에 따른 값을 반환해야 한다
And 값은 0 ~ 100 사이여야 한다
```

**Scenario**: RSI 과매도 신호 생성

```
Given RSI가 30일 때
When Signal Detector가 분석을 수행하면
Then 과매도(oversold) 신호를 생성해야 한다
And 신호는 매수 기회를 나타내야 한다
```

**Scenario**: RSI 과매수 신호 생성

```
Given RSI가 70일 때
When Signal Detector가 분석을 수행하면
Then 과매수(overbought) 신호를 생성해야 한다
And 신호는 매도 기회를 나타내야 한다
```

**Scenario**: RSI 파라미터 검증

```
Given RSI 기간 파라미터가 0 또는 음수일 때
When 지표를 생성하면
Then 기본값 14를 사용해야 한다
And 경고 로그를 기록해야 한다
```

---

### AC-302: MACD Indicator

**Feature**: MACD 지표 계산

**Scenario**: MACD가 정확하게 계산되어야 한다

```
Given 충분한 가격 데이터가 있을 때 (최소 26개)
And MACD 지표가 생성되었을 때
When MACD를 계산하면
Then MACD 라인, 시그널 라인, 히스토그램을 반환해야 한다
```

**Scenario**: MACD 불리시 교차 신호

```
Given MACD 라인이 시그널 라인 아래에 있을 때
And MACD가 상향 추세일 때
When MACD가 시그널을 상향 돌파하면
Then 매수 신호(bullish cross)를 생성해야 한다
```

**Scenario**: MACD 베어리시 교차 신호

```
Given MACD 라인이 시그널 라인 위에 있을 때
And MACD가 하향 추세일 때
When MACD가 시그널을 하향 이탈하면
Then 매도 신호(bearish cross)를 생성해야 한다
```

---

### AC-303: Bollinger Bands

**Feature**: Bollinger Bands 지표 계산

**Scenario**: Bollinger Bands가 정확하게 계산되어야 한다

```
Given 20개의 종가 데이터가 있을 때
And Bollinger Bands가 생성되었을 때
When 밴드를 계산하면
Then 상단 밴드, 중간선(SMA), 하단 밴드를 반환해야 한다
And 상단/하단 밴드는 중간선에서 ±2표준편차여야 한다
```

**Scenario**: 하단 밴드 터치 신호

```
Given 가격이 하단 밴드에 근접할 때 (거리 < 1%)
When Signal Detector가 분석을 수행하면
Then 매수 기회 신호를 생성해야 한다
```

**Scenario**: 상단 밴드 터치 신호

```
Given 가격이 상단 밴드에 근접할 때 (거리 < 1%)
When Signal Detector가 분석을 수행하면
Then 매도 기회 신호를 생성해야 한다
```

**Scenario**: 밴드폭 축소(Squeeze) 감지

```
Given 밴드폭(bandwidth)이 0.1 미만일 때
When Bollinger Bands를 분석하면
Then 변동성 축소(squeeze) 상태를 감지해야 한다
And 추세 급등 가능성을 표시해야 한다
```

---

### AC-304: Moving Averages

**Feature**: 이동평균선 지표 계산

**Scenario**: SMA가 정확하게 계산되어야 한다

```
Given N개의 종가 데이터가 있을 때
When SMA를 계산하면
Then 단순 평균(Σ price / N)을 반환해야 한다
```

**Scenario**: EMA가 정확하게 계산되어야 한다

```
Given EMA 기간이 N일 때
And 이전 EMA 값이 존재할 때
When 새로운 가격으로 EMA를 계산하면
Then 가중 평균 공식을 적용해야 한다
And 최근 가격에 더 높은 가중치를 부여해야 한다
```

**Scenario**: 골든크로스 감지

```
Given 단기 SMA(50)가 장기 SMA(200) 아래에 있을 때
When 단기 SMA가 장기 SMA를 상향 돌파하면
Then 골든크로스 매수 신호를 생성해야 한다
```

**Scenario**: 데드크로스 감지

```
Given 단기 SMA(50)가 장기 SMA(200) 위에 있을 때
When 단기 SMA가 장기 SMA를 하향 이탈하면
Then 데드크로스 매도 신호를 생성해야 한다
```

---

### AC-305: Multi-Indicator Strategy

**Feature**: 다중 지표 결합 전략

**Scenario**: 신호 점수화

```
Given RSI 과매도 신호 (confidence: 0.8, weight: 1.5)
And MACD 불리시 교차 신호 (confidence: 0.9, weight: 1.0)
When 종합 점수를 계산하면
Then 가중평균 점수를 반환해야 한다
And 점수 = Σ(signal * confidence * weight) / Σ(weight)
```

**Scenario**: 매수 결정

```
Given 종합 점수가 0.6 이상일 때 (임계값 0.6)
When MultiIndicatorStrategy가 결정을 내리면
Then 매수 결정(Buy)을 내려야 한다
And confidence를 점수로 설정해야 한다
```

**Scenario**: 매도 결정

```
Given 종합 점수가 -0.6 이하일 때 (임계값 0.6)
When MultiIndicatorStrategy가 결정을 내리면
Then 매도 결정(Sell)을 내려야 한다
And confidence를 점수의 절댓값으로 설정해야 한다
```

**Scenario**: 유지 결정

```
Given 종합 점수가 -0.6 ~ 0.6 사이일 때
When MultiIndicatorStrategy가 결정을 내리면
Then 유지 결정(Hold)을 내려야 한다
```

---

### AC-306: Historical Data Fetching

**Feature**: 과거 데이터 수집

**Scenario**: 캔들 데이터 조회

```
Given Upbit API가 사용 가능할 때
When 특정 마켓의 1분봉 데이터를 조회하면
Then 최대 200개의 캔들을 반환해야 한다
And 각 캔들은 시가/고가/저가/종가/거래량을 포함해야 한다
```

**Scenario**: 날짜 범위 조회

```
Given 1년치 데이터를 요청할 때
When 날짜 범위로 데이터를 조회하면
Then 청크 기반으로 데이터를 로드해야 한다 (200개/요청)
And 모든 데이터를 합쳐서 반환해야 한다
```

**Scenario**: 데이터 캐싱

```
Given 동일한 범위의 데이터를 두 번 요청할 때
When 두 번째 요청을 수행하면
Then 캐시된 데이터를 반환해야 한다
And Upbit API를 호출하지 않아야 한다
```

---

### AC-307: Backtest Simulator

**Feature**: 백테스팅 시뮬레이션

**Scenario**: 기본 백테스팅 실행

```
Given 초기 잔고가 1,000,000 KRW일 때
And 수수료가 0.05%일 때
And 백테스팅 데이터가 1000개 캔들일 때
When 백테스팅을 실행하면
Then 모든 캔들을 순회해야 한다
And 발생한 모든 거래를 기록해야 한다
And 최종 잔고를 반환해야 한다
```

**Scenario**: 수수료 계산

```
Given 1,000,000 KRW 매수 주문을 실행할 때
When 주문이 체결되면
Then 수수료 500 KRW를 차감해야 한다
And 실제 투자 금액은 999,500 KRW여야 한다
```

**Scenario**: 자산 곡선 계산

```
Given 백테스팅이 완료되었을 때
When 자산 곡선(equity curve)을 계산하면
Then 각 캔들 시점의 포트폴리오 가치를 포함해야 한다
And 길이는 입력 캔들 수와 같아야 한다
```

**Scenario**: 불충분한 데이터 처리

```
Given 백테스팅 데이터가 50개 캔들 미만일 때
When 백테스팅을 실행하면
Then 시뮬레이션을 건너뛰어야 한다
And "데이터 불충분" 에러를 반환해야 한다
```

---

### AC-308: Performance Metrics

**Feature**: 성과 메트릭 계산

**Scenario**: ROI 계산

```
Given 초기 잔고가 1,000,000 KRW일 때
And 최종 잔고가 1,200,000 KRW일 때
When ROI를 계산하면
Then 0.2 (20%)를 반환해야 한다
```

**Scenario**: Win Rate 계산

```
Given 총 100건의 거래가 있을 때
And 60건이 수익 거래일 때
When Win Rate를 계산하면
Then 0.6 (60%)를 반환해야 한다
```

**Scenario**: Max Drawdown 계산

```
Given 자산 곡선이 [100, 120, 110, 90, 80, 100]일 때
When Max Drawdown을 계산하면
Then 최대 낙폭을 계산해야 한다 (고점 120 → 저점 80, -33.3%)
And 0.333를 반환해야 한다
```

**Scenario**: Sharpe Ratio 계산

```
Given 일별 수익률 데이터가 있을 때
When Sharpe Ratio를 계산하면
Then (평균 수익률 - 무위험 이자율) / 표준편차를 반환해야 한다
```

---

### AC-309: Parameter Optimizer

**Feature**: 파라미터 최적화

**Scenario**: 그리드 서치 실행

```
Given RSI 기간 파라미터 범위가 [10, 14, 18]일 때
And 과매수/과매도 임계값이 [65, 70, 75] / [25, 30, 35]일 때
When 그리드 서치를 실행하면
Then 3 × 3 × 2 = 18가지 조합을 테스트해야 한다
And 각 조합의 성과 메트릭을 반환해야 한다
```

**Scenario**: 최적 파라미터 선택

```
Given 18가지 파라미터 조합의 결과가 있을 때
And ROI를 최적화 목표로 설정할 때
When 최적 파라미터를 선택하면
Then 가장 높은 ROI를 가진 조합을 반환해야 한다
```

**Scenario**: 병렬 처리

```
Given 100가지 파라미터 조합이 있을 때
When 최적화를 실행하면
Then 병렬로 처리해야 한다 (rayon 또는 tokio 사용)
And 단일 스레드보다 빨라야 한다
```

---

### AC-310: Strategy Manager

**Feature**: 전략 관리

**Scenario**: 전략 등록

```
Given 새로운 전략을 생성했을 때
When Strategy Manager에 등록하면
Then 전략 목록에 추가되어야 한다
And 이름으로 조회할 수 있어야 한다
```

**Scenario**: 전략 전환

```
Given 현재 모멘텀 전략이 활성화되어 있을 때
And 다중 지표 전략이 등록되어 있을 때
When 전략을 "multi_indicator"로 전환하면
Then 활성 전략이 변경되어야 한다
And 새 전략의 파라미터를 로드해야 한다
```

**Scenario**: 전략 목록 조회

```
Given 3개의 전략이 등록되어 있을 때
When 전략 목록을 조회하면
Then 3개의 전략 이름을 반환해야 한다
```

---

### AC-311: Indicator Caching

**Feature**: 지표 캐싱

**Scenario**: 캐시 저장

```
Given 지표를 처음 계산할 때
When 지표 값을 계산하면
Then 결과를 SQLite 캐시에 저장해야 한다
And 키는 (market, indicator_type, timestamp)여야 한다
```

**Scenario**: 캐시 히트

```
Given 캐시에 RSI 값이 저장되어 있을 때
When 동일한 시점의 RSI를 조회하면
Then 캐시된 값을 반환해야 한다
And 재계산하지 않아야 한다
```

**Scenario**: 캐시 무효화

```
Given 캐시된 지표 값이 있을 때
When 지표를 리셋하면
Then 캐시를 비워야 한다
```

---

## 3. Non-Functional Acceptance Criteria (비기능적 인수 기준)

### AC-NFR-301: Performance

**Scenario**: 지표 계산 성능

```
Given 1000개의 캔들 데이터가 있을 때
When 모든 지표를 계산하면
Then 각 지표 계산은 100ms 이내여야 한다
```

**Scenario**: 백테스팅 성능

```
Given 1년치 1분봉 데이터 (약 525,600개)가 있을 때
When 백테스팅을 실행하면
Then 10초 이내에 완료해야 한다
```

**Scenario**: 메모리 사용량

```
Given 백테스팅을 실행할 때
When 최대 메모리 사용량을 측정하면
Then 500MB 이하여야 한다
```

---

### AC-NFR-302: Accuracy

**Scenario**: 지표 계산 정확도

```
Given 표준 데이터 세트가 있을 때
When RSI를 계산하면
Then TradingView 또는 표준 라이브러리 값과 99.9% 일치해야 한다
And 오차는 0.01 미만이어야 한다
```

**Scenario**: 백테스팅 재현성

```
Given 동일한 데이터와 파라미터로 백테스팅을 실행할 때
When 두 번 실행하면
Then 결과가 100% 일치해야 한다
And 모든 거래 시점, 가격, 수량이 동일해야 한다
```

---

### AC-NFR-303: Security

**Scenario**: 파라미터 검증

```
Given 유효하지 않은 파라미터로 지표를 생성할 때 (기간 = -1)
When 지표를 생성하면
Then 에러를 반환하거나 기본값을 사용해야 한다
And 시스템이 크래시하면 안 된다
```

**Scenario**: 백테스팅 결과 해석

```
Given 백테스팅 결과를 표시할 때
When 사용자에게 결과를 보여주면
Then "백테스팅은 실제 수익을 보장하지 않음" 경고를 포함해야 한다
```

---

## 4. Quality Gates (품질 게이트)

### QG-301: Code Coverage

- [ ] 단위 테스트 커버리지: 85% 이상
- [ ] 지표별 테스트: 모든 지표 테스트 존재
- [ ] 백테스팅 테스트: 통합 테스트 포함

### QG-302: Linting

- [ ] `cargo clippy` 통과 (warning 0개)
- [ ] `cargo fmt --check` 통과

### QG-303: Accuracy

- [ ] 지표 공식 검증: 표준 라이브러리와 비교
- [ ] 백테스팅 재현성: 100% 일치

### QG-304: Performance

- [ ] 지표 계산 < 100ms
- [ ] 백테스팅 1년치 < 10초
- [ ] 메모리 사용량 < 500MB

### QG-305: Documentation

- [ ] 모든 공개 함수에 doc comment 존재
- [ ] 지표 공식이 문서화되어 있음
- [ ] 백테스팅 사용 가이드 제공

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

**M1 (Indicator Foundation)**:
- [ ] Trait 정의 완료
- [ ] 캐싱 시스템 동작

**M2-M5 (Individual Indicators)**:
- [ ] 지표 계산 정확도 검증 통과
- [ ] 신호 생성 기능 동작

**M6 (Multi-Indicator)**:
- [ ] 점수화 시스템 동작
- [ ] 통합 테스트 통과

**M8 (Backtest Simulator)**:
- [ ] 과거 데이터로 시뮬레이션 성공
- [ ] 성과 메트릭 정확함

**M10 (Optimizer)**:
- [ ] 그리드 서치로 최적 파라미터 탐색 가능
- [ ] 병렬 처리로 성능 개선

**M12 (Integration)**:
- [ ] 기존 시스템과 통합 완료
- [ ] 엔드투엔드 테스트 통과

---

## 6. Test Scenarios Summary

| ID | Feature | Priority | Test Type |
|----|---------|----------|-----------|
| AC-301 | RSI Indicator | High | Unit |
| AC-302 | MACD Indicator | High | Unit |
| AC-303 | Bollinger Bands | High | Unit |
| AC-304 | Moving Averages | Medium | Unit |
| AC-305 | Multi-Indicator Strategy | High | Unit + Integration |
| AC-306 | Historical Data Fetching | High | Integration |
| AC-307 | Backtest Simulator | High | Integration |
| AC-308 | Performance Metrics | Medium | Unit |
| AC-309 | Parameter Optimizer | Medium | Integration |
| AC-310 | Strategy Manager | Medium | Integration |
| AC-311 | Indicator Caching | Medium | Unit |
| AC-NFR-301 | Performance | Medium | Performance |
| AC-NFR-302 | Accuracy | High | Property-based |
| AC-NFR-303 | Security | High | Unit |

---

**Traceability**: `SPEC-ID: SPEC-TRADING-003` → Acceptance Phase Complete
