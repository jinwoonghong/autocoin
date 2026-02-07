/**
 * Settings type definitions for AutoCoin web interface
 * These types match the Rust backend configuration structures
 */

/**
 * Strategy type enumeration
 */
export type StrategyType = "momentum" | "multi-indicator" | "mean-reversion";

/**
 * Strategy parameters configuration
 */
export interface StrategyParams {
  // Active strategy selection
  strategy: StrategyType;

  // Target coins configuration
  target_coins_count: number;
  min_volume_24h: number;

  // Momentum strategy parameters
  surge_threshold: number;
  surge_window_minutes: number;
  volume_multiplier: number;

  // RSI indicator parameters
  rsi_period: number;
  rsi_oversold: number;
  rsi_overbought: number;

  // MACD indicator parameters
  macd_fast_period: number;
  macd_slow_period: number;
  macd_signal_period: number;

  // Position sizing
  position_size_percent: number;
  max_positions: number;
}

/**
 * Risk management parameters
 */
export interface RiskParams {
  // Stop loss / Take profit
  stop_loss_rate: number;
  take_profit_rate: number;

  // Position sizing limits
  max_position_size_krw: number;
  max_total_exposure_percent: number;

  // Trailing stop
  trailing_stop_enabled: boolean;
  trailing_stop_rate: number;
  trailing_stop_activation_percent: number;

  // Daily loss limit
  daily_loss_limit_enabled: boolean;
  daily_loss_limit_krw: number;
}

/**
 * Notification type preferences
 */
export interface NotificationPreferences {
  buy_signals: boolean;
  sell_signals: boolean;
  error_alerts: boolean;
  daily_summary: boolean;
  weekly_report: boolean;
}

/**
 * Notification parameters
 */
export interface NotificationParams {
  enabled: boolean;

  // Discord webhook
  discord_webhook_url: string;
  discord_enabled: boolean;

  // Telegram (future)
  telegram_enabled: boolean;
  telegram_chat_id: string;

  // Preferences
  preferences: NotificationPreferences;
}

/**
 * System status information
 */
export interface SystemStatus {
  running: boolean;
  uptime_seconds: number;
  last_trade_time?: number;
  version: string;
  commit_hash: string;
  build_time: string;
}

/**
 * Complete settings configuration
 */
export interface SettingsConfig {
  strategy: StrategyParams;
  risk: RiskParams;
  notifications: NotificationParams;
  system: SystemStatus;
}

/**
 * Default strategy parameters
 */
export const defaultStrategyParams: StrategyParams = {
  strategy: "multi-indicator",
  target_coins_count: 5,
  min_volume_24h: 1000000000, // 1B KRW
  surge_threshold: 5.0,
  surge_window_minutes: 30,
  volume_multiplier: 1.5,
  rsi_period: 14,
  rsi_oversold: 30,
  rsi_overbought: 70,
  macd_fast_period: 12,
  macd_slow_period: 26,
  macd_signal_period: 9,
  position_size_percent: 10,
  max_positions: 1,
};

/**
 * Default risk parameters
 */
export const defaultRiskParams: RiskParams = {
  stop_loss_rate: 3.0,
  take_profit_rate: 5.0,
  max_position_size_krw: 500000,
  max_total_exposure_percent: 90,
  trailing_stop_enabled: false,
  trailing_stop_rate: 1.5,
  trailing_stop_activation_percent: 2.0,
  daily_loss_limit_enabled: true,
  daily_loss_limit_krw: 100000,
};

/**
 * Default notification parameters
 */
export const defaultNotificationParams: NotificationParams = {
  enabled: true,
  discord_webhook_url: "",
  discord_enabled: false,
  telegram_enabled: false,
  telegram_chat_id: "",
  preferences: {
    buy_signals: true,
    sell_signals: true,
    error_alerts: true,
    daily_summary: true,
    weekly_report: false,
  },
};

/**
 * Settings section type for navigation
 */
export type SettingsSection = "strategy" | "risk" | "system" | "notifications" | "about";

/**
 * Settings navigation item
 */
export interface SettingsNavItem {
  id: SettingsSection;
  label: string;
  icon: string;
  description: string;
}
