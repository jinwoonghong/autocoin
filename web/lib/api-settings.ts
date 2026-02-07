/**
 * Settings API client
 *
 * Handles all API calls for settings management
 */

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";

export interface ApiResponse<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
}

/**
 * Fetch all settings
 */
export async function fetchSettings(): Promise<SettingsConfig> {
  const response = await fetch(`${API_BASE}/api/settings`);
  if (!response.ok) {
    throw new Error(`Failed to fetch settings: ${response.statusText}`);
  }
  return response.json();
}

/**
 * Fetch system status
 */
export async function fetchSystemStatus(): Promise<SystemStatus> {
  const response = await fetch(`${API_BASE}/api/system/status`);
  if (!response.ok) {
    throw new Error(`Failed to fetch system status: ${response.statusText}`);
  }
  return response.json();
}

/**
 * Save strategy settings
 */
export async function saveStrategySettings(
  settings: StrategyParams
): Promise<void> {
  const response = await fetch(`${API_BASE}/api/settings/strategy`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(settings),
  });
  if (!response.ok) {
    throw new Error(`Failed to save strategy settings: ${response.statusText}`);
  }
}

/**
 * Save risk settings
 */
export async function saveRiskSettings(settings: RiskParams): Promise<void> {
  const response = await fetch(`${API_BASE}/api/settings/risk`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(settings),
  });
  if (!response.ok) {
    throw new Error(`Failed to save risk settings: ${response.statusText}`);
  }
}

/**
 * Save notification settings
 */
export async function saveNotificationSettings(
  settings: NotificationParams
): Promise<void> {
  const response = await fetch(`${API_BASE}/api/settings/notifications`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(settings),
  });
  if (!response.ok) {
    throw new Error(`Failed to save notification settings: ${response.statusText}`);
  }
}

/**
 * Start trading system
 */
export async function startSystem(): Promise<void> {
  const response = await fetch(`${API_BASE}/api/system/start`, {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(`Failed to start system: ${response.statusText}`);
  }
}

/**
 * Stop trading system
 */
export async function stopSystem(): Promise<void> {
  const response = await fetch(`${API_BASE}/api/system/stop`, {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(`Failed to stop system: ${response.statusText}`);
  }
}

/**
 * Restart trading system
 */
export async function restartSystem(): Promise<void> {
  const response = await fetch(`${API_BASE}/api/system/restart`, {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(`Failed to restart system: ${response.statusText}`);
  }
}

/**
 * Send test notification
 */
export async function testNotification(): Promise<void> {
  const response = await fetch(`${API_BASE}/api/notifications/test`, {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(`Failed to send test notification: ${response.statusText}`);
  }
}

// Type imports
import type {
  SettingsConfig,
  StrategyParams,
  RiskParams,
  NotificationParams,
  SystemStatus,
} from "@/types/settings";
