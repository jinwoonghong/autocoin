//! Core trading data types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 가격 틱 데이터 (Price Tick)
///
/// Upbit WebSocket에서 수신한 실시간 가격 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTick {
    /// 마켓 코드 (예: "KRW-BTC")
    pub market: String,
    /// 타임스탬프 (millisecond)
    pub timestamp: i64,
    /// 체결 가격
    pub trade_price: f64,
    /// 전일 대비 변화율
    pub change_rate: f64,
    /// 체결량
    pub volume: f64,
    /// 체결 금액
    pub trade_amount: f64,
}

impl PriceTick {
    /// 새로운 가격 틱 생성
    pub fn new(
        market: String,
        timestamp: i64,
        trade_price: f64,
        change_rate: f64,
        volume: f64,
    ) -> Self {
        let trade_amount = trade_price * volume;
        Self {
            market,
            timestamp,
            trade_price,
            change_rate,
            volume,
            trade_amount,
        }
    }

    /// UTC 타임스탬프로 변환
    pub fn to_datetime(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(self.timestamp)
            .unwrap_or_else(|| DateTime::default())
    }
}

/// 거래 신호 (Trading Signal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// 마켓 코드
    pub market: String,
    /// 신호 타입
    pub signal_type: SignalType,
    /// 신뢰도 (0.0 ~ 1.0)
    pub confidence: f64,
    /// 신호 생성 이유
    pub reason: String,
    /// 생성 시간
    pub timestamp: DateTime<Utc>,
}

impl Signal {
    /// 새로운 매수 신호 생성
    pub fn buy(market: String, confidence: f64, reason: String) -> Self {
        Self {
            market,
            signal_type: SignalType::Buy,
            confidence,
            reason,
            timestamp: Utc::now(),
        }
    }

    /// 새로운 매도 신호 생성
    pub fn sell(market: String, confidence: f64, reason: String) -> Self {
        Self {
            market,
            signal_type: SignalType::Sell,
            confidence,
            reason,
            timestamp: Utc::now(),
        }
    }

    /// 강한 매수 신호 생성
    pub fn strong_buy(market: String, confidence: f64, reason: String) -> Self {
        Self {
            market,
            signal_type: SignalType::StrongBuy,
            confidence,
            reason,
            timestamp: Utc::now(),
        }
    }

    /// 새로운 보유 신호 생성
    pub fn hold(market: String) -> Self {
        Self {
            market,
            signal_type: SignalType::Hold,
            confidence: 0.0,
            reason: "No significant signal".to_string(),
            timestamp: Utc::now(),
        }
    }
}

/// 신호 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalType {
    /// 매수
    Buy,
    /// 강한 매수
    StrongBuy,
    /// 매도
    Sell,
    /// 강한 매도
    StrongSell,
    /// 보유
    Hold,
}

/// 거래 결정 (Trading Decision)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    /// 매수
    Buy {
        market: String,
        amount_krw: f64,
        reason: String,
    },
    /// 매도
    Sell {
        market: String,
        amount: f64,
        reason: String,
    },
    /// 보유 (아무것도 하지 않음)
    Hold {
        reason: String,
    },
}

impl Decision {
    /// 매수 결정 생성
    pub fn buy(market: String, amount_krw: f64, reason: String) -> Self {
        Decision::Buy {
            market,
            amount_krw,
            reason,
        }
    }

    /// 매도 결정 생성
    pub fn sell(market: String, amount: f64, reason: String) -> Self {
        Decision::Sell {
            market,
            amount,
            reason,
        }
    }

    /// 보유 결정 생성
    pub fn hold(reason: String) -> Self {
        Decision::Hold { reason }
    }

    /// 매수/매도 결정인지 확인
    pub fn is_trade(&self) -> bool {
        matches!(self, Decision::Buy { .. } | Decision::Sell { .. })
    }
}

/// 포지션 (Position)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// 마켓 코드
    pub market: String,
    /// 진입 가격
    pub entry_price: f64,
    /// 보유 수량
    pub amount: f64,
    /// 진입 시간
    pub entry_time: DateTime<Utc>,
    /// 손절가
    pub stop_loss: f64,
    /// 익절가
    pub take_profit: f64,
}

impl Position {
    /// 새로운 포지션 생성
    pub fn new(
        market: String,
        entry_price: f64,
        amount: f64,
        stop_loss_rate: f64,
        take_profit_rate: f64,
    ) -> Self {
        let stop_loss = entry_price * (1.0 - stop_loss_rate);
        let take_profit = entry_price * (1.0 + take_profit_rate);

        Self {
            market,
            entry_price,
            amount,
            entry_time: Utc::now(),
            stop_loss,
            take_profit,
        }
    }

    /// 현재 가격 기준 PnL 계산
    pub fn calculate_pnl(&self, current_price: f64) -> PnL {
        let value = self.amount * current_price;
        let cost = self.amount * self.entry_price;
        let profit = value - cost;
        let profit_rate = (current_price / self.entry_price) - 1.0;

        PnL {
            cost,
            value,
            profit,
            profit_rate,
        }
    }

    /// 손절가 도달 여부 확인
    pub fn should_stop_loss(&self, current_price: f64) -> bool {
        current_price <= self.stop_loss
    }

    /// 익절가 도달 여부 확인
    pub fn should_take_profit(&self, current_price: f64) -> bool {
        current_price >= self.take_profit
    }
}

/// 손익 (Profit and Loss)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnL {
    /// 원본 금액 (KRW)
    pub cost: f64,
    /// 현재 가치 (KRW)
    pub value: f64,
    /// 손익 금액 (KRW)
    pub profit: f64,
    /// 손익률
    pub profit_rate: f64,
}

/// 주문 (Order)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 주문 UUID
    pub id: String,
    /// 마켓 코드
    pub market: String,
    /// 주문 방향
    pub side: OrderSide,
    /// 주문 가격
    pub price: f64,
    /// 주문 수량
    pub volume: f64,
    /// 주문 상태
    pub status: OrderStatus,
    /// 생성 시간
    pub created_at: DateTime<Utc>,
    /// 체결된 수량
    pub executed_volume: f64,
    /// 체결된 금액
    pub executed_amount: f64,
}

impl Order {
    /// 새로운 주문 생성
    pub fn new(market: String, side: OrderSide, price: f64, volume: f64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            market,
            side,
            price,
            volume,
            status: OrderStatus::Waiting,
            created_at: Utc::now(),
            executed_volume: 0.0,
            executed_amount: 0.0,
        }
    }

    /// 주문 체결 여부 확인
    pub fn is_executed(&self) -> bool {
        matches!(self.status, OrderStatus::Executed)
    }

    /// 주문 대기 중인지 확인
    pub fn is_waiting(&self) -> bool {
        matches!(self.status, OrderStatus::Waiting)
    }
}

/// 주문 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 매수 (Bid)
    Bid,
    /// 매도 (Ask)
    Ask,
}

impl OrderSide {
    /// 매수인지 확인
    pub fn is_bid(&self) -> bool {
        matches!(self, OrderSide::Bid)
    }

    /// 매도인지 확인
    pub fn is_ask(&self) -> bool {
        matches!(self, OrderSide::Ask)
    }
}

/// 주문 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 대기 중
    Waiting,
    /// 체결됨
    Executed,
    /// 취소됨
    Canceled,
    /// 실패
    Failed,
}

/// 주문 결과 (Order Result)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResult {
    /// 주문 정보
    pub order: Order,
    /// 성공 여부
    pub success: bool,
    /// 에러 메시지 (실패 시)
    pub error: Option<String>,
    /// 체결 시간 (완료 시)
    pub executed_at: Option<DateTime<Utc>>,
}

impl OrderResult {
    /// 성공한 주문 결과 생성
    pub fn success(order: Order) -> Self {
        Self {
            order,
            success: true,
            error: None,
            executed_at: Some(Utc::now()),
        }
    }

    /// 실패한 주문 결과 생성
    pub fn failure(market: String, side: OrderSide, error: String) -> Self {
        let order = Order::new(market, side, 0.0, 0.0);
        Self {
            order,
            success: false,
            error: Some(error),
            executed_at: None,
        }
    }
}

/// 리스크 관리 액션 (Risk Action)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskAction {
    /// 포지션 유지
    Maintain,
    /// 손절 실행
    StopLoss {
        market: String,
        current_price: f64,
        pnl_rate: f64,
    },
    /// 익절 실행
    TakeProfit {
        market: String,
        current_price: f64,
        pnl_rate: f64,
    },
}

/// 캔들 데이터 (Candlestick Data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// 마켓 코드
    pub market: String,
    /// 캔들 시간 (UTC)
    pub timestamp: DateTime<Utc>,
    /// 시가
    pub open_price: f64,
    /// 고가
    pub high_price: f64,
    /// 저가
    pub low_price: f64,
    /// 종가
    pub close_price: f64,
    /// 거래량
    pub volume: f64,
}

impl Candle {
    /// 새로운 캔들 생성
    pub fn new(
        market: String,
        timestamp: DateTime<Utc>,
        open_price: f64,
        high_price: f64,
        low_price: f64,
        close_price: f64,
        volume: f64,
    ) -> Self {
        Self {
            market,
            timestamp,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
        }
    }

    /// 양봉인지 확인
    pub fn is_bullish(&self) -> bool {
        self.close_price > self.open_price
    }

    /// 음봉인지 확인
    pub fn is_bearish(&self) -> bool {
        self.close_price < self.open_price
    }

    /// 상승률 계산
    pub fn change_rate(&self) -> f64 {
        (self.close_price / self.open_price) - 1.0
    }
}

/// 계정 잔고 (Account Balance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// 통화 코드 (예: "KRW")
    pub currency: String,
    /// 보유 수량
    pub balance: f64,
    /// 주문 중인 수량 (잠김)
    pub locked: f64,
    /// 사용 가능 수량
    pub available: f64,
    /// 평균 매입가 (KRW 마켓인 경우)
    pub avg_buy_price: f64,
}

impl Balance {
    /// 사용 가능 잔고 계산
    pub fn available_balance(&self) -> f64 {
        self.balance - self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_tick_creation() {
        let tick = PriceTick::new("KRW-BTC".to_string(), 1234567890, 50000000.0, 0.05, 0.001);
        assert_eq!(tick.market, "KRW-BTC");
        assert_eq!(tick.trade_price, 50000000.0);
        assert_eq!(tick.trade_amount, 50000.0);
    }

    #[test]
    fn test_signal_creation() {
        let buy_signal = Signal::buy("KRW-BTC".to_string(), 0.8, "Surge detected".to_string());
        assert!(matches!(buy_signal.signal_type, SignalType::Buy));
        assert_eq!(buy_signal.confidence, 0.8);
    }

    #[test]
    fn test_position_pnl_calculation() {
        let position = Position::new("KRW-BTC".to_string(), 100000.0, 0.001, 0.05, 0.1);
        let pnl = position.calculate_pnl(110000.0);
        assert!((pnl.profit_rate - 0.1).abs() < 0.001);
        assert!(pnl.profit > 0.0);
    }

    #[test]
    fn test_stop_loss_detection() {
        let position = Position::new("KRW-BTC".to_string(), 100000.0, 0.001, 0.05, 0.1);
        assert!(position.should_stop_loss(94000.0));
        assert!(!position.should_stop_loss(96000.0));
    }

    #[test]
    fn test_take_profit_detection() {
        let position = Position::new("KRW-BTC".to_string(), 100000.0, 0.001, 0.05, 0.1);
        assert!(position.should_take_profit(110000.0));
        assert!(!position.should_take_profit(109000.0));
    }

    #[test]
    fn test_order_creation() {
        let order = Order::new("KRW-BTC".to_string(), OrderSide::Bid, 50000000.0, 0.001);
        assert_eq!(order.market, "KRW-BTC");
        assert!(order.side.is_bid());
        assert!(order.is_waiting());
    }

    #[test]
    fn test_decision_types() {
        let buy = Decision::buy("KRW-BTC".to_string(), 50000.0, "Test".to_string());
        let sell = Decision::sell("KRW-BTC".to_string(), 0.001, "Test".to_string());
        let hold = Decision::hold("Test".to_string());

        assert!(buy.is_trade());
        assert!(sell.is_trade());
        assert!(!hold.is_trade());
    }
}
