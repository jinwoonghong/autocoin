/**
 * Markets page type definitions
 */

/**
 * Market filter type
 */
export type MarketFilter = "all" | "top10" | "top20" | "favorites";

/**
 * Sort column type
 */
export type SortColumn = "market" | "price" | "change" | "volume";

/**
 * Sort direction type
 */
export type SortDirection = "asc" | "desc";

/**
 * Coin price with optional signal data
 */
export interface CoinPriceData {
  market: string;
  korean_name?: string;
  english_name?: string;
  trade_price: number;
  change_rate: number;
  change_amount: number;
  volume: number;
  volume_power?: number;
  high_price: number;
  low_price: number;
  prev_closing_price: number;
  timestamp: number;
}

/**
 * 24-hour candle data for mini-chart
 */
export interface CandleData {
  timestamp: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

/**
 * Market detail data
 */
export interface MarketDetail {
  market: string;
  korean_name: string;
  english_name: string;
  current_price: number;
  change_rate: number;
  change_amount: number;
  volume_24h: number;
  high_24h: number;
  low_24h: number;
  prev_closing_price: number;
  candles_24h: CandleData[];
  signals?: {
    type: "buy" | "sell" | "neutral";
    confidence: number;
    reason: string;
  }[];
}

/**
 * Markets page state
 */
export interface MarketsPageState {
  markets: CoinPriceData[];
  filter: MarketFilter;
  search: string;
  sortColumn: SortColumn;
  sortDirection: SortDirection;
  loading: boolean;
  error?: string;
  selectedMarket?: string;
}

/**
 * Market filters configuration
 */
export const MARKET_FILTERS: Record<MarketFilter, string> = {
  all: "All Markets",
  top10: "Top 10",
  top20: "Top 20",
  favorites: "Favorites",
} as const;
