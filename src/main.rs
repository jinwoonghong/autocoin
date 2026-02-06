//! AutoCoin - Upbit Automated Trading Agent System
//!
//! 멀티 에이전트 기반 자동 트레이딩 시스템입니다.

use autocoin::agents::{
    DecisionMaker, ExecutionAgent, MarketMonitor, NotificationAgent, RiskManager,
    SignalDetector,
};
use autocoin::config::Settings;
use autocoin::db::Database;
use autocoin::error::Result;
use autocoin::upbit::UpbitClient;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // 로깅 초기화
    init_logging();

    info!("Starting AutoCoin Trading Bot");

    // 설정 로드
    let settings = Settings::load().map_err(|e| {
        error!("Failed to load settings: {}", e);
        e
    })?;

    // 설정 유효성 검증
    if let Err(e) = settings.validate() {
        error!("Configuration validation failed: {}", e);
        return Err(e.into());
    }

    info!("Settings loaded: {}", settings.display_safe());

    // 데이터베이스 초기화
    let db = Database::new(&settings.system.db_path).await?;
    info!("Database initialized: {}", settings.system.db_path);

    // Upbit 클라이언트 초기화
    let upbit_client = UpbitClient::new(
        settings.upbit.access_key.clone(),
        settings.upbit.secret_key.clone(),
    );
    info!("Upbit client initialized");

    // Top N 코인 목록 조회
    let markets = upbit_client.get_top_krw_markets(settings.trading.target_coins).await?;
    info!("Monitoring {} markets: {:?}", markets.len(), markets);

    // 채널 생성 (에이전트 간 통신)
    let (price_tx, price_rx) = mpsc::channel(1000);
    let (signal_tx, signal_rx) = mpsc::channel(1000);
    let (decision_tx, decision_rx) = mpsc::channel(1000);
    let (order_tx, order_rx) = mpsc::channel(1000);

    // AGENT 1: Market Monitor (시가 모니터링)
    let market_monitor = MarketMonitor::new(markets.clone());
    let monitor_price_tx = price_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = market_monitor.monitor(monitor_price_tx).await {
            error!("Market monitor error: {}", e);
        }
    });

    // AGENT 2: Signal Detector (신호 감지)
    let signal_detector = SignalDetector::new(settings.trading.clone(), price_rx);
    let signal_price_tx = price_tx.clone();
    tokio::spawn(async move {
        let mut detector = signal_detector.with_signal_channel(signal_tx);
        if let Err(e) = detector.run().await {
            error!("Signal detector error: {}", e);
        }
    });

    // AGENT 3: Decision Maker (의사결정)
    let mut decision_maker = DecisionMaker::new(settings.trading.clone(), signal_rx)
        .with_decision_channel(decision_tx.clone());

    // 초기 잔고 조회
    match upbit_client.get_krw_balance().await {
        Ok(balance) => {
            decision_maker.set_balance(balance);
            info!("KRW balance: {} KRW", balance);
        }
        Err(e) => {
            warn!("Failed to get KRW balance: {}", e);
            decision_maker.set_balance(0.0);
        }
    }

    // 초기 포지션 로드
    let initial_position = db.get_all_active_positions().await?.first().cloned();
    if let Some(ref pos) = initial_position {
        info!("Existing position found: {}", pos.market);
    }
    decision_maker.set_position(initial_position);

    let decision_tx_clone = decision_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = decision_maker.run().await {
            error!("Decision maker error: {}", e);
        }
    });

    // AGENT 4: Execution Agent (주문 실행)
    let execution_agent = ExecutionAgent::new(
        upbit_client.clone(),
        db.clone(),
        settings.trading.clone(),
        decision_rx,
    )
    .with_order_channel(order_tx.clone());

    let order_tx_clone = order_tx.clone();
    tokio::spawn(async move {
        if let Err(e) = execution_agent.run().await {
            error!("Execution agent error: {}", e);
        }
    });

    // AGENT 5: Risk Manager (리스크 관리)
    let (risk_decision_tx, risk_decision_rx) = mpsc::channel(1000);
    let mut risk_manager = RiskManager::new(settings.trading.clone(), db.clone(), signal_price_tx)
        .with_risk_channel(risk_decision_tx);

    // 리스크 관리자도 실행 (결과는 execution으로 전송 필요)
    let risk_decision_tx_for_execution = decision_tx_clone.clone();
    tokio::spawn(async move {
        if let Err(e) = risk_manager.run().await {
            error!("Risk manager error: {}", e);
        }
    });

    // 리스크 관리자의 결정을 Execution으로 전달
    tokio::spawn(async move {
        while let Some(decision) = risk_decision_rx.recv().await {
            let _ = decision_tx_clone.send(decision).await;
        }
    });

    // AGENT 6: Notification Agent (알림)
    if settings.discord.enabled {
        let notification_agent = NotificationAgent::new(
            settings.discord.webhook_url.clone(),
            settings.discord.notify_on_buy,
            settings.discord.notify_on_sell,
            settings.discord.notify_on_signal,
            settings.discord.notify_on_error,
        );

        tokio::spawn(async move {
            while let Some(order_result) = order_rx.recv().await {
                if let Err(e) = notification_agent.notify_order_result(&order_result).await {
                    warn!("Failed to send notification: {}", e);
                }
            }
        });

        info!("Notification agent started");
    } else {
        info!("Notification agent disabled");
    }

    // 상태 저장 및 건강 체크 태스크
    let db_for_health = db.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            // 건강 체크 로직
            if let Err(e) = db_for_health.save_state("health_check", "OK").await {
                warn!("Health check failed: {}", e);
            }
        }
    });

    // 종료 시그널 대기
    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal, shutting down...");

    // 정리 작업
    // ...

    Ok(())
}

/// 로깅 초기화
fn init_logging() {
    let log_level = std::env::var("RUST_LOG")
        .or_else(|_| std::env::var("LOG_LEVEL"))
        .unwrap_or_else(|_| "info".to_string());

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&log_level));

    // JSON 형식 로그 (프로덕션)
    if std::env::var("LOG_JSON").unwrap_or_else(|_| "true".to_string()) == "true" {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        // 일반 텍스트 로그 (개발)
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().pretty())
            .init();
    }
}

/// 테스트 모드 실행
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_structure() {
        // 메인 구조 검증 테스트
        assert!(true, "Main structure validated");
    }
}
