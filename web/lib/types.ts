/**
 * Core TypeScript types for AutoCoin Web Dashboard
 * These types mirror the Rust backend data structures
 */

// ============================================================================
// Position & Trading Types
// ============================================================================

export interface Position {
  id: string;
  market: string; // e.g., "KRW-BTC"
  entry_price: number;
  current_price: number;
  amount: number;
  entry_time: number;
  stop_loss: number;
  take_profit: number;
  exit_price?: number;
  exit_time?: number;
  pnl?: number;
  pnl_rate?: number;
  status: "active" | "closed";
}

export interface BalanceData {
  krw_balance: number;
  available: number;
  locked: number;
  total_asset_value: number;
  coin_balances: CoinBalance[];
}

export interface CoinBalance {
  currency: string; // e.g., "BTC", "ETH"
  balance: number;
  available: number;
  locked: number;
  avg_buy_price: number;
  current_price: number;
}

export interface Trade {
  id: string;
  market: string;
  side: "buy" | "sell";
  price: number;
  volume: number;
  amount: number;
  status: "pending" | "executed" | "cancelled" | "failed";
  created_at: number;
  executed_volume?: number;
  executed_amount?: number;
  pnl?: number;
  error?: string;
}

// ============================================================================
// Market Data Types
// ============================================================================

export interface CoinPrice {
  market: string;
  trade_price: number;
  change_rate: number;
  change_amount: number;
  volume: number;
  acc_trade_volume: number;
  acc_trade_price: number;
  high_price: number;
  low_price: number;
  prev_closing_price: number;
  timestamp: number;
}

export interface MarketInfo {
  market: string;
  korean_name: string;
  english_name: string;
  market_warning: string | null;
}

// ============================================================================
// Agent Status Types
// ============================================================================

export type AgentStatusValue = "running" | "idle" | "error" | "paused";

export interface AgentStatus {
  name: string;
  status: AgentStatusValue;
  last_update: number;
  message?: string;
}

export type AgentName =
  | "market_monitor"
  | "signal_detector"
  | "decision_maker"
  | "executor"
  | "risk_manager"
  | "notification";

// ============================================================================
// WebSocket Message Types
// ============================================================================

export type WebSocketMessageType =
  | "price_update"
  | "trade_executed"
  | "position_update"
  | "agent_status"
  | "notification"
  | "backtest_progress"
  | "backtest_complete"
  | "balance_update";

export interface WebSocketMessage<T = any> {
  type: WebSocketMessageType;
  data: T;
  timestamp: number;
}

export interface PriceUpdateData {
  market: string;
  price: number;
  change: number;
  timestamp: number;
}

export interface TradeExecutedData {
  trade: Trade;
}

export interface PositionUpdateData {
  position: Position;
}

export interface AgentStatusData {
  agent: string;
  status: AgentStatusValue;
  message?: string;
}

export interface NotificationData {
  type: "info" | "success" | "warning" | "error";
  title: string;
  message: string;
  timestamp: number;
}

// ============================================================================
// Backtest Types
// ============================================================================

export interface BacktestConfig {
  market: string;
  strategy: string;
  start_date: string;
  end_date: string;
  initial_balance: number;
  commission_rate: number;
  parameters?: Record<string, number>;
}

export interface BacktestResult {
  id: string;
  config: BacktestConfig;
  status: "pending" | "running" | "completed" | "failed";
  progress: number;
  result?: BacktestMetrics;
  error?: string;
  created_at: number;
  completed_at?: number;
}

export interface BacktestMetrics {
  total_trades: number;
  winning_trades: number;
  losing_trades: number;
  win_rate: number;
  roi: number;
  total_return: number;
  max_drawdown: number;
  sharpe_ratio: number;
  sortino_ratio: number;
  profit_factor: number;
  avg_win: number;
  avg_loss: number;
  equity_curve: EquityPoint[];
  trades: BacktestTrade[];
}

export interface EquityPoint {
  timestamp: number;
  balance: number;
  pnl: number;
}

export interface BacktestTrade {
  entry_time: number;
  exit_time: number;
  market: string;
  side: "buy" | "sell";
  entry_price: number;
  exit_price: number;
  volume: number;
  pnl: number;
  pnl_rate: number;
}

// ============================================================================
// Settings Types
// ============================================================================

export interface StrategySettings {
  name: string;
  target_coins: number;
  surge_threshold: number;
  surge_timeframe_minutes: number;
  volume_multiplier: number;
  target_profit_rate: number;
  stop_loss_rate: number;
  max_positions: number;
  max_position_ratio: number;
}

export interface IndicatorSettings {
  rsi: {
    enabled: boolean;
    period: number;
    oversold: number;
    overbought: number;
  };
  macd: {
    enabled: boolean;
    fast_period: number;
    slow_period: number;
    signal_period: number;
  };
  bollinger: {
    enabled: boolean;
    period: number;
    std_dev: number;
  };
  sma: {
    enabled: boolean;
    short_period: number;
    long_period: number;
  };
}

export interface SystemSettings {
  trading_enabled: boolean;
  readonly_mode: boolean;
  log_level: "debug" | "info" | "warn" | "error";
  notifications_enabled: boolean;
}

export interface Settings {
  strategy: StrategySettings;
  indicators: IndicatorSettings;
  system: SystemSettings;
}

// ============================================================================
// API Response Types
// ============================================================================

export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  message?: string;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

// ============================================================================
// UI Component Types
// ============================================================================

export interface TimeRangeOption {
  value: "1D" | "7D" | "30D" | "90D" | "ALL";
  label: string;
}

export interface ChartDataPoint {
  timestamp: number;
  value: number;
  label?: string;
}

export interface NotificationPreferences {
  types: ("buy" | "sell" | "signal" | "error")[];
  enabled: boolean;
}
