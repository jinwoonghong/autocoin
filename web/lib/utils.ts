import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/**
 * Utility function to merge Tailwind CSS classes
 * Combines clsx for conditional classes and tailwind-merge for deduplication
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

/**
 * Format currency value to KRW
 */
export function formatKRW(value: number): string {
  return new Intl.NumberFormat("ko-KR", {
    style: "currency",
    currency: "KRW",
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(value);
}

/**
 * Format percentage value
 */
export function formatPercent(value: number, decimals: number = 2): string {
  const sign = value >= 0 ? "+" : "";
  return `${sign}${value.toFixed(decimals)}%`;
}

/**
 * Format timestamp to locale string
 */
export function formatTimestamp(timestamp: number): string {
  return new Date(timestamp).toLocaleString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

/**
 * Get color class for PnL value
 */
export function getPnLColor(value: number): string {
  if (value > 0) return "text-green-500";
  if (value < 0) return "text-red-500";
  return "text-muted-foreground";
}

/**
 * Get background color class for PnL badge
 */
export function getPnLBgColor(value: number): string {
  if (value > 0) return "bg-green-500/10 text-green-500 border-green-500/20";
  if (value < 0) return "bg-red-500/10 text-red-500 border-red-500/20";
  return "bg-muted text-muted-foreground";
}

/**
 * Format uptime in seconds to human readable string
 */
export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const parts: string[] = [];
  if (days > 0) parts.push(`${days}d`);
  if (hours > 0 || days > 0) parts.push(`${hours}h`);
  if (minutes > 0 || hours > 0 || days > 0) parts.push(`${minutes}m`);
  parts.push(`${secs}s`);

  return parts.join(" ");
}

/**
 * Format duration in milliseconds to human readable string
 */
export function formatDuration(ms: number): string {
  const seconds = Math.floor(ms / 1000);
  return formatUptime(seconds);
}

/**
 * Debounce function for search inputs
 */
export function debounce<T extends (...args: unknown[]) => unknown>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: NodeJS.Timeout | null = null;
  return (...args: Parameters<T>) => {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}
