/**
 * Formatting utilities for AutoCoin Dashboard
 */

/**
 * Format price in KRW with commas
 */
export function formatPrice(value: number): string {
  return new Intl.NumberFormat("ko-KR", {
    style: "currency",
    currency: "KRW",
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(value);
}

/**
 * Format price with decimals for crypto amounts
 */
export function formatVolume(value: number): string {
  if (value < 0.0001) {
    return value.toFixed(8);
  }
  if (value < 1) {
    return value.toFixed(4);
  }
  return value.toFixed(2);
}

/**
 * Format percentage
 */
export function formatPercent(value: number): string {
  const sign = value >= 0 ? "+" : "";
  return `${sign}${(value * 100).toFixed(2)}%`;
}

/**
 * Format date/time
 */
export function formatDateTime(timestamp: number | Date): string {
  const date = typeof timestamp === "number" ? new Date(timestamp) : timestamp;
  return date.toLocaleString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

/**
 * Format date only
 */
export function formatDate(timestamp: number | Date): string {
  const date = typeof timestamp === "number" ? new Date(timestamp) : timestamp;
  return date.toLocaleDateString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
}

/**
 * Format time only
 */
export function formatTime(timestamp: number | Date): string {
  const date = typeof timestamp === "number" ? new Date(timestamp) : timestamp;
  return date.toLocaleTimeString("ko-KR", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
}

/**
 * Format large numbers with suffix
 */
export function formatLargeNumber(value: number): string {
  if (value >= 1_000_000_000) {
    return `${(value / 1_000_000_000).toFixed(1)}B`;
  }
  if (value >= 1_000_000) {
    return `${(value / 1_000_000).toFixed(1)}M`;
  }
  if (value >= 1_000) {
    return `${(value / 1_000).toFixed(1)}K`;
  }
  return value.toString();
}
