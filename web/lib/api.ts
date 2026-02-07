/**
 * API Client for AutoCoin Web Dashboard
 * Handles all REST API communication with the Rust backend
 */

import type {
  ApiResponse,
  BalanceData,
  Position,
  Trade,
  CoinPrice,
  AgentStatus,
  BacktestConfig,
  BacktestResult,
  Settings,
  PaginatedResponse,
} from "./types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080/api";

// ============================================================================
// Error Handling
// ============================================================================

class ApiError extends Error {
  constructor(
    message: string,
    public status?: number,
    public code?: string
  ) {
    super(message);
    this.name = "ApiError";
  }
}

async function handleResponse<T>(response: Response): Promise<T> {
  if (!response.ok) {
    let errorMessage = `API Error: ${response.status} ${response.statusText}`;
    try {
      const errorData = await response.json();
      errorMessage = errorData.message || errorData.error || errorMessage;
    } catch {
      // Use default error message
    }
    throw new ApiError(errorMessage, response.status);
  }

  const data = await response.json();
  return data as T;
}

// ============================================================================
// API Client
// ============================================================================

export const api = {
  // ========================================================================
  // Health & Status
  // ========================================================================

  async healthCheck(): Promise<ApiResponse<{ status: string }>> {
    const response = await fetch(`${API_BASE}/health`, {
      cache: "no-store",
    });
    return handleResponse<ApiResponse<{ status: string }>>(response);
  },

  // ========================================================================
  // Balance & Portfolio
  // ========================================================================

  async getBalance(): Promise<BalanceData> {
    const response = await fetch(`${API_BASE}/balance`, {
      cache: "no-store",
    });
    return handleResponse<BalanceData>(response);
  },

  async getPosition(): Promise<Position | null> {
    const response = await fetch(`${API_BASE}/position`, {
      cache: "no-store",
    });
    if (response.status === 404) {
      return null;
    }
    return handleResponse<Position>(response);
  },

  // ========================================================================
  // Trades
  // ========================================================================

  async getTrades(params?: {
    limit?: number;
    offset?: number;
    market?: string;
    side?: "buy" | "sell";
    status?: string;
  }): Promise<PaginatedResponse<Trade>> {
    const searchParams = new URLSearchParams();
    if (params?.limit) searchParams.set("limit", params.limit.toString());
    if (params?.offset) searchParams.set("offset", params.offset.toString());
    if (params?.market) searchParams.set("market", params.market);
    if (params?.side) searchParams.set("side", params.side);
    if (params?.status) searchParams.set("status", params.status);

    const response = await fetch(
      `${API_BASE}/trades?${searchParams.toString()}`,
      { cache: "no-store" }
    );
    return handleResponse<PaginatedResponse<Trade>>(response);
  },

  // ========================================================================
  // Markets
  // ========================================================================

  async getMarkets(): Promise<CoinPrice[]> {
    const response = await fetch(`${API_BASE}/markets`, {
      cache: "no-store",
    });
    return handleResponse<CoinPrice[]>(response);
  },

  async getMarketInfo(market?: string): Promise<CoinPrice[]> {
    const url = market
      ? `${API_BASE}/markets/${market}`
      : `${API_BASE}/markets`;
    const response = await fetch(url, { cache: "no-store" });
    return handleResponse<CoinPrice[]>(response);
  },

  // ========================================================================
  // Orders
  // ========================================================================

  async createOrder(params: {
    market: string;
    side: "buy" | "sell";
    amount_krw?: number;
    amount_coin?: number;
  }): Promise<ApiResponse<{ order_id: string; status: string }>> {
    const response = await fetch(`${API_BASE}/orders`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(params),
    });
    return handleResponse<ApiResponse<{ order_id: string; status: string }>>(
      response
    );
  },

  async closePosition(): Promise<ApiResponse<{ success: boolean }>> {
    const response = await fetch(`${API_BASE}/position`, {
      method: "DELETE",
    });
    return handleResponse<ApiResponse<{ success: boolean }>>(response);
  },

  // ========================================================================
  // Agents
  // ========================================================================

  async getAgentStatus(): Promise<AgentStatus[]> {
    const response = await fetch(`${API_BASE}/agents/status`, {
      cache: "no-store",
    });
    return handleResponse<AgentStatus[]>(response);
  },

  // ========================================================================
  // Backtesting
  // ========================================================================

  async runBacktest(config: BacktestConfig): Promise<ApiResponse<{ id: string }>> {
    const response = await fetch(`${API_BASE}/backtest`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(config),
    });
    return handleResponse<ApiResponse<{ id: string }>>(response);
  },

  async getBacktestResult(id: string): Promise<BacktestResult> {
    const response = await fetch(`${API_BASE}/backtest/${id}`, {
      cache: "no-store",
    });
    return handleResponse<BacktestResult>(response);
  },

  async listBacktests(): Promise<BacktestResult[]> {
    const response = await fetch(`${API_BASE}/backtest`, {
      cache: "no-store",
    });
    return handleResponse<BacktestResult[]>(response);
  },

  // ========================================================================
  // Settings
  // ========================================================================

  async getSettings(): Promise<Settings> {
    const response = await fetch(`${API_BASE}/settings`, {
      cache: "no-store",
    });
    return handleResponse<Settings>(response);
  },

  async updateSettings(settings: Partial<Settings>): Promise<ApiResponse> {
    const response = await fetch(`${API_BASE}/settings`, {
      method: "PUT",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(settings),
    });
    return handleResponse<ApiResponse>(response);
  },

  // ========================================================================
  // Trading Control
  // ========================================================================

  async pauseTrading(): Promise<ApiResponse> {
    const response = await fetch(`${API_BASE}/trading/pause`, {
      method: "POST",
    });
    return handleResponse<ApiResponse>(response);
  },

  async resumeTrading(): Promise<ApiResponse> {
    const response = await fetch(`${API_BASE}/trading/resume`, {
      method: "POST",
    });
    return handleResponse<ApiResponse>(response);
  },
};

// ============================================================================
// React Hooks for API
// ============================================================================

import { useState, useEffect, useCallback } from "react";

export function useBalance() {
  const [data, setData] = useState<BalanceData | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchBalance = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await api.getBalance();
      setData(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchBalance();
  }, [fetchBalance]);

  return { data, loading, error, refetch: fetchBalance };
}

export function usePosition() {
  const [data, setData] = useState<Position | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchPosition = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await api.getPosition();
      setData(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchPosition();
  }, [fetchPosition]);

  return { data, loading, error, refetch: fetchPosition };
}

export function useTrades(params?: {
  limit?: number;
  offset?: number;
  market?: string;
  side?: "buy" | "sell";
}) {
  const [data, setData] = useState<PaginatedResponse<Trade> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTrades = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await api.getTrades(params);
      setData(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }, [params]);

  useEffect(() => {
    fetchTrades();
  }, [fetchTrades]);

  return { data, loading, error, refetch: fetchTrades };
}

export function useMarkets() {
  const [data, setData] = useState<CoinPrice[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchMarkets = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await api.getMarkets();
      setData(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchMarkets();
  }, [fetchMarkets]);

  return { data, loading, error, refetch: fetchMarkets };
}

export function useAgentStatus() {
  const [data, setData] = useState<AgentStatus[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchAgentStatus = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const result = await api.getAgentStatus();
      setData(result);
    } catch (e) {
      setError(e instanceof Error ? e.message : "Unknown error");
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchAgentStatus();
  }, [fetchAgentStatus]);

  return { data, loading, error, refetch: fetchAgentStatus };
}

export { ApiError };
