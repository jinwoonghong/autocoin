/**
 * Dashboard type definitions for AutoCoin web interface
 * These types match the Rust backend data structures
 */

/**
 * Agent status enumeration
 */
export type AgentStatus = "running" | "idle" | "error" | "stopped";

/**
 * Individual agent state
 */
export interface AgentState {
  name: string;
  status: AgentStatus;
  last_update: string;
  message?: string;
}

/**
 * Balance information
 */
export interface BalanceData {
  krw: number;
  krw_locked: number;
  crypto_value: number;
  total: number;
}

/**
 * Active position information
 */
export interface Position {
  id: string;
  market: string;
  entry_price: number;
  amount: number;
  current_price: number;
  stop_loss: number;
  take_profit: number;
  entry_time: number;
  exit_price?: number;
  exit_time?: number;
  pnl?: number;
  pnl_rate?: number;
  status: "active" | "closed";
}

/**
 * Trade information
 */
export interface Trade {
  id: string;
  market: string;
  side: "buy" | "sell";
  price: number;
  volume: number;
  total: number;
  timestamp: number;
  status: "filled" | "pending" | "failed";
  profit?: number;
  profit_rate?: number;
}

/**
 * PnL data point for charts
 */
export interface PnLDataPoint {
  date: string;
  pnl: number;
  pnl_rate: number;
  balance: number;
}

/**
 * Quick statistics
 */
export interface QuickStats {
  win_rate: number;
  total_trades: number;
  winning_trades: number;
  losing_trades: number;
  today_pnl: number;
  today_pnl_rate: number;
  total_pnl: number;
  total_pnl_rate: number;
}

/**
 * Market price information
 */
export interface CoinPrice {
  market: string;
  trade_price: number;
  change_rate: number;
  change_amount: number;
  volume: number;
  high_price: number;
  low_price: number;
}

/**
 * Complete dashboard data
 */
export interface DashboardData {
  balance: BalanceData;
  position?: Position;
  agents: AgentState[];
  trades: Trade[];
  pnl_history: PnLDataPoint[];
  stats: QuickStats;
  market_prices: CoinPrice[];
}

/**
 * WebSocket message types
 */
export type WSMessageType =
  | "price_update"
  | "position_update"
  | "agent_status"
  | "trade_executed"
  | "balance_update";

export interface WSMessage {
  type: WSMessageType;
  data: unknown;
  timestamp: number;
}

/**
 * API response wrapper
 */
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}
