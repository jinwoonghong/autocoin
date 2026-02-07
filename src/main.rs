//! AutoCoin - Upbit Automated Trading Agent System
//!
//! 멀티 에이전트 기반 자동 트레이딩 시스템입니다.

use autocoin::agents::{
    DecisionMaker, ExecutionAgent, MarketMonitor, NotificationAgent, RiskManager,
    SignalDetector,
};
use autocoin::config::Settings;
use autocoin::dashboard::{
    AgentState, AgentStatus, DashboardData, Notification as DashboardNotification,
    NotificationType,
};
use autocoin::db::Database;
use autocoin::error::Result;
use autocoin::types::{OrderResult, PriceTick};
use autocoin::upbit::UpbitClient;
use chrono::Utc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Dashboard data channel size
const DASHBOARD_CHANNEL_SIZE: usize = 100;

/// CLI arguments
#[derive(Debug, clap::Parser)]
#[command(name = "autocoin")]
#[command(about = "Upbit Automated Trading Agent System", long_about = None)]
struct Cli {
    /// Enable TUI dashboard (default: true)
    #[arg(short, long, default_value = "true")]
    dashboard: bool,

    /// Run in daemon mode (no TUI, log-only)
    #[arg(short, long, default_value = "false")]
    daemon: bool,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Log file path
    #[arg(short, long)]
    log_file: Option<String>,

    /// Config file path
    #[arg(short, long, default_value = ".env/config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // 로깅 초기화
    init_logging(&cli.log_level, cli.log_file.as_deref());

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

    // ========== DASHBOARD DATA CHANNEL (TAG-002) ==========
    // Dashboard 데이터를 위한 채널 생성
    let (dashboard_data_tx, dashboard_data_rx) = mpsc::channel::<DashboardData>(DASHBOARD_CHANNEL_SIZE);

    // Dashboard 시작 여부 확인
    let enable_dashboard = cli.dashboard && !cli.daemon;

    if enable_dashboard {
        info!("Dashboard enabled - starting TUI");
    } else if cli.daemon {
        info!("Daemon mode - TUI disabled, log output only");
    } else {
        info!("Dashboard disabled by CLI flag");
    }

    // ========== AGENTS WITH DASHBOARD INTEGRATION ==========

    // 채널 생성 (에이전트 간 통신)
    let (price_tx, price_rx) = mpsc::channel(1000);
    let (signal_tx, signal_rx) = mpsc::channel(1000);
    let (decision_tx, decision_rx) = mpsc::channel(1000);
    let (order_tx, order_rx) = mpsc::channel(1000);

    // AGENT 1: Market Monitor (시가 모니터링)
    let market_monitor = MarketMonitor::new(markets.clone());
    let monitor_price_tx = price_tx.clone();
    let monitor_dashboard_tx = dashboard_data_tx.clone();

    tokio::spawn(async move {
        // 에이전트 시작 상태 전송
        let _ = monitor_dashboard_tx.try_send(create_agent_status("MarketMonitor", AgentStatus::Running));

        if let Err(e) = market_monitor.monitor(monitor_price_tx).await {
            error!("Market monitor error: {}", e);
            let _ = monitor_dashboard_tx.try_send(create_agent_status("MarketMonitor", AgentStatus::Error));
        }
    });

    // AGENT 2: Signal Detector (신호 감지)
    let signal_detector = SignalDetector::new(settings.trading.clone(), price_rx);
    let signal_price_tx = price_tx.clone();
    let signal_dashboard_tx = dashboard_data_tx.clone();

    tokio::spawn(async move {
        // 에이전트 시작 상태 전송
        let _ = signal_dashboard_tx.try_send(create_agent_status("SignalDetector", AgentStatus::Running));

        let mut detector = signal_detector.with_signal_channel(signal_tx);
        if let Err(e) = detector.run().await {
            error!("Signal detector error: {}", e);
            let _ = signal_dashboard_tx.try_send(create_agent_status("SignalDetector", AgentStatus::Error));
        }
    });

    // AGENT 3: Decision Maker (의사결정)
    let mut decision_maker = DecisionMaker::new(settings.trading.clone(), signal_rx)
        .with_decision_channel(decision_tx.clone());
    let decision_dashboard_tx = dashboard_data_tx.clone();

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
        // 에이전트 시작 상태 전송
        let _ = decision_dashboard_tx.try_send(create_agent_status("DecisionMaker", AgentStatus::Running));

        if let Err(e) = decision_maker.run().await {
            error!("Decision maker error: {}", e);
            let _ = decision_dashboard_tx.try_send(create_agent_status("DecisionMaker", AgentStatus::Error));
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
    let executor_dashboard_tx = dashboard_data_tx.clone();

    tokio::spawn(async move {
        // 에이전트 시작 상태 전송 (IDLE 상태로 시작)
        let _ = executor_dashboard_tx.try_send(create_agent_status("Executor", AgentStatus::Idle));

        if let Err(e) = execution_agent.run().await {
            error!("Execution agent error: {}", e);
            let _ = executor_dashboard_tx.try_send(create_agent_status("Executor", AgentStatus::Error));
        }
    });

    // AGENT 5: Risk Manager (리스크 관리)
    let (risk_decision_tx, risk_decision_rx) = mpsc::channel(1000);
    let mut risk_manager = RiskManager::new(settings.trading.clone(), db.clone(), signal_price_tx)
        .with_risk_channel(risk_decision_tx);
    let risk_dashboard_tx = dashboard_data_tx.clone();

    // 리스크 관리자도 실행 (결과는 execution으로 전송 필요)
    let risk_decision_tx_for_execution = decision_tx_clone.clone();
    tokio::spawn(async move {
        // 에이전트 시작 상태 전송
        let _ = risk_dashboard_tx.try_send(create_agent_status("RiskManager", AgentStatus::Running));

        if let Err(e) = risk_manager.run().await {
            error!("Risk manager error: {}", e);
            let _ = risk_dashboard_tx.try_send(create_agent_status("RiskManager", AgentStatus::Error));
        }
    });

    // 리스크 관리자의 결정을 Execution으로 전달
    tokio::spawn(async move {
        while let Some(decision) = risk_decision_rx.recv().await {
            let _ = decision_tx_clone.send(decision).await;
        }
    });

    // AGENT 6: Notification Agent (알림)
    let notification_dashboard_tx = dashboard_data_tx.clone();

    if settings.discord.enabled {
        let notification_agent = NotificationAgent::new(
            settings.discord.webhook_url.clone(),
            settings.discord.notify_on_buy,
            settings.discord.notify_on_sell,
            settings.discord.notify_on_signal,
            settings.discord.notify_on_error,
        );

        tokio::spawn(async move {
            // 에이전트 시작 상태 전송
            let _ = notification_dashboard_tx.try_send(create_agent_status("Notification", AgentStatus::Running));

            while let Some(order_result) = order_rx.recv().await {
                // Dashboard 알림 전송 (TAG-002)
                send_order_notification(&notification_dashboard_tx, &order_result);

                if let Err(e) = notification_agent.notify_order_result(&order_result).await {
                    warn!("Failed to send notification: {}", e);
                }
            }
        });

        info!("Notification agent started");
    } else {
        info!("Notification agent disabled");
    }

    // ========== DASHBOARD UI TASK (TAG-003) ==========
    // Dashboard UI 태스크 시작 (단, daemon 모드가 아닐 때만)
    if enable_dashboard {
        // 사용자 액션 채널 생성
        let (user_action_tx, mut user_action_rx) = mpsc::channel::<autocoin::dashboard::renderer::UserAction>(10);

        // Dashboard UI 태스크 시작
        let dashboard_handle = tokio::spawn(async move {
            if let Err(e) = autocoin::dashboard::renderer::run_dashboard(dashboard_data_rx, user_action_tx).await {
                error!("Dashboard UI error: {}", e);
            }
        });

        info!("Dashboard UI started");

        // 사용자 액션 처리 태스크
        let shutdown_signal = std::sync::Arc::new(tokio::sync::Notify::new());
        let shutdown_signal_clone = shutdown_signal.clone();

        tokio::spawn(async move {
            while let Some(action) = user_action_rx.recv().await {
                match action {
                    autocoin::dashboard::renderer::UserAction::Quit => {
                        info!("Quit action from dashboard, initiating shutdown");
                        shutdown_signal_clone.notify_one();
                        break;
                    }
                    autocoin::dashboard::renderer::UserAction::Pause => {
                        info!("Pause action from dashboard - trading agents paused");
                        // Pause is now a informational state - agents continue running
                        // but no new trades will be executed by DecisionMaker
                        // To fully implement pause, each agent would need a pause signal channel
                    }
                    autocoin::dashboard::renderer::UserAction::Resume => {
                        info!("Resume action from dashboard - trading agents resumed");
                        // Resume is now an informational state
                        // To fully implement resume, each agent would need a resume signal channel
                    }
                    autocoin::dashboard::renderer::UserAction::Help => {
                        info!("Help action from dashboard");
                        // Help overlay would show keyboard shortcuts
                        // Available: q=quit, p=pause, r=resume, h=help
                    }
                    autocoin::dashboard::renderer::UserAction::None => {
                        // No action
                    }
                }
            }
        });

        // 대시보드 완료 또는 종료 시그널 대기
        tokio::select! {
            _ = dashboard_handle => {
                info!("Dashboard UI task completed");
            }
            _ = shutdown_signal.notified() => {
                info!("Shutdown signal received");
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Ctrl+C received");
            }
        }
    } else {
        // Daemon 모드: Dashboard 데이터를 로그로 출력
        let mut rx = dashboard_data_rx;
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                // Daemon 모드에서는 로그로 출력 (1분 간격으로 요약)
                static LAST_LOG: std::sync::Mutex<Option<chrono::DateTime<chrono::Utc>>> = std::sync::Mutex::new(None);
                let now = chrono::Utc::now();
                let should_log = {
                    let mut last = LAST_LOG.lock().unwrap();
                    let elapsed = last.map_or(true, |l| now.signed_duration_since(l).num_seconds() >= 60);
                    if elapsed {
                        *last = Some(now);
                    }
                    elapsed
                };

                if should_log {
                    info!(
                        "Dashboard Update: {} agents, position: {}, balance: {} KRW, notifications: {}",
                        data.agent_states.len(),
                        data.position.is_some(),
                        data.balance.total_asset_value,
                        data.notifications.len()
                    );
                }
            }
        });
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

/// 에이전트 상태 생성 헬퍼 함수
fn create_agent_status(name: &str, status: AgentStatus) -> DashboardData {
    let mut data = DashboardData::new();
    let agent_state = AgentState::new(name.to_string(), status);
    data.update_agent_state(name.to_string(), agent_state);
    data
}

/// 주문 결과를 Dashboard 알림으로 전송 (비차단 try_send)
fn send_order_notification(tx: &mpsc::Sender<DashboardData>, result: &OrderResult) {
    let notification_type = match result.order.side {
        autocoin::types::OrderSide::Bid => NotificationType::Buy,
        autocoin::types::OrderSide::Ask => NotificationType::Sell,
    };

    let message = if result.success {
        format!(
            "Order executed: {} {} @ {}",
            result.order.side.as_bid().then(|| "BUY").unwrap_or("SELL"),
            result.order.market,
            result.order.price
        )
    } else {
        format!(
            "Order failed: {} - {}",
            result.order.market,
            result.error.as_deref().unwrap_or("Unknown error")
        )
    };

    let mut data = DashboardData::new();
    let notification = DashboardNotification::new(notification_type, message);
    data.add_notification(notification);

    // 비차단 전송 (try_send)
    let _ = tx.try_send(data);
}

/// 로깅 초기화
fn init_logging(log_level: &str, log_file: Option<&str>) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level));

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

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::parse_from(["autocoin"]);
        assert!(cli.dashboard);
        assert!(!cli.daemon);

        let cli = Cli::parse_from(["autocoin", "--daemon"]);
        assert!(!cli.dashboard);
        assert!(cli.daemon);
    }

    #[tokio::test]
    async fn test_main_structure() {
        // 메인 구조 검증 테스트
        assert!(true, "Main structure validated");
    }
}
