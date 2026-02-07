/**
 * API Client for AutoCoin Dashboard
 *
 * Handles communication with the Rust backend REST API
 */

import { DashboardData, ApiResponse } from "@/types/dashboard";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";

/**
 * Generic fetch wrapper with error handling
 */
async function fetchAPI<T>(
  endpoint: string,
  options?: RequestInit
): Promise<ApiResponse<T>> {
  try {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...options,
      headers: {
        "Content-Type": "application/json",
        ...options?.headers,
      },
    });

    if (!response.ok) {
      return {
        success: false,
        error: `HTTP ${response.status}: ${response.statusText}`,
      };
    }

    const data = await response.json();
    return {
      success: true,
      data,
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error.message : "Unknown error",
    };
  }
}

/**
 * Dashboard API
 */
export const dashboardAPI = {
  /**
   * Get all dashboard data
   */
  async getDashboardData(): Promise<ApiResponse<DashboardData>> {
    return fetchAPI<DashboardData>("/api/dashboard");
  },

  /**
   * Get balance information
   */
  async getBalance(): Promise<ApiResponse<{ krw: number; locked: number }>> {
    return fetchAPI("/api/balance");
  },

  /**
   * Get current position
   */
  async getPosition(): Promise<ApiResponse<any>> {
    return fetchAPI("/api/position");
  },

  /**
   * Get agent statuses
   */
  async getAgents(): Promise<ApiResponse<any[]>> {
    return fetchAPI("/api/agents");
  },

  /**
   * Get recent trades
   */
  async getTrades(limit: number = 10): Promise<ApiResponse<any[]>> {
    return fetchAPI(`/api/trades?limit=${limit}`);
  },

  /**
   * Get PnL history
   */
  async getPnLHistory(days: number = 7): Promise<ApiResponse<any[]>> {
    return fetchAPI(`/api/pnl?days=${days}`);
  },

  /**
   * Get statistics
   */
  async getStats(): Promise<ApiResponse<any>> {
    return fetchAPI("/api/stats");
  },
};

/**
 * WebSocket client for real-time updates
 */
export class WebSocketClient {
  private ws: WebSocket | null = null;
  private reconnectTimer: NodeJS.Timeout | null = null;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 5;

  constructor(private url: string) {}

  connect(onMessage: (data: unknown) => void, onError?: (error: Event) => void) {
    try {
      this.ws = new WebSocket(this.url);

      this.ws.onopen = () => {
        console.log("WebSocket connected");
        this.reconnectAttempts = 0;
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          onMessage(data);
        } catch (error) {
          console.error("Failed to parse WebSocket message:", error);
        }
      };

      this.ws.onclose = () => {
        console.log("WebSocket disconnected");
        this.scheduleReconnect(onMessage, onError);
      };

      this.ws.onerror = (error) => {
        console.error("WebSocket error:", error);
        onError?.(error);
      };
    } catch (error) {
      console.error("Failed to create WebSocket:", error);
      this.scheduleReconnect(onMessage, onError);
    }
  }

  private scheduleReconnect(
    onMessage: (data: unknown) => void,
    onError?: (error: Event) => void
  ) {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error("Max reconnect attempts reached");
      return;
    }

    const delay = Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);

    this.reconnectTimer = setTimeout(() => {
      this.reconnectAttempts++;
      console.log(`Reconnecting... (attempt ${this.reconnectAttempts})`);
      this.connect(onMessage, onError);
    }, delay);
  }

  disconnect() {
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }

    if (this.ws) {
      this.ws.close();
      this.ws = null;
    }
  }

  send(data: unknown) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
    }
  }
}

/**
 * Create a WebSocket client for the dashboard
 */
export function createWebSocketClient(): WebSocketClient {
  const wsUrl = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:3000/ws";
  return new WebSocketClient(wsUrl);
}
