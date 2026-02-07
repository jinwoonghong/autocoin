"use client";

/**
 * Dashboard Page
 *
 * Main dashboard page for AutoCoin trading system
 * Displays portfolio, positions, agent status, PnL chart, and recent trades
 */

import React, { useState, useEffect, useCallback } from "react";
import useSWR from "swr";
import { PortfolioSummary } from "@/components/dashboard/PortfolioSummary";
import { PositionCard } from "@/components/dashboard/PositionCard";
import { AgentStatusGrid } from "@/components/dashboard/AgentStatusGrid";
import { PnLChart } from "@/components/dashboard/PnLChart";
import { RecentTrades } from "@/components/dashboard/RecentTrades";
import { QuickStats } from "@/components/dashboard/QuickStats";
import { DashboardData, AgentState, Trade, PnLDataPoint } from "@/types/dashboard";
import { dashboardAPI, createWebSocketClient } from "@/lib/api-client";
import { IconRefresh } from "@/components/ui/icons";

// Fetcher for SWR
const fetcher = async (url: string) => {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}: ${response.statusText}`);
  }
  return response.json();
};

// Refresh interval in milliseconds (5 seconds)
const REFRESH_INTERVAL = 5000;

export default function DashboardPage() {
  const [isAutoRefresh, setIsAutoRefresh] = useState(true);
  const [wsConnected, setWsConnected] = useState(false);
  const [wsClient, setWsClient] = useState<ReturnType<typeof createWebSocketClient> | null>(null);

  // Fetch dashboard data with SWR
  const {
    data: dashboardData,
    error,
    isLoading,
    mutate,
  } = useSWR<DashboardData>(
    `${process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000"}/api/dashboard`,
    fetcher,
    {
      refreshInterval: isAutoRefresh ? REFRESH_INTERVAL : 0,
      revalidateOnFocus: true,
      revalidateOnReconnect: true,
      dedupingInterval: 1000,
    }
  );

  // Setup WebSocket connection
  useEffect(() => {
    if (!isAutoRefresh) {
      wsClient?.disconnect();
      setWsConnected(false);
      return;
    }

    const client = createWebSocketClient();
    setWsClient(client);

    client.connect(
      (data) => {
        // Handle WebSocket messages
        setWsConnected(true);

        // Mutate SWR data based on message type
        mutate((currentData) => {
          if (!currentData) return currentData;

          // Update data based on WebSocket message
          // This is a simplified version - you'd want to handle different message types
          return { ...currentData };
        }, false);
      },
      () => {
        setWsConnected(false);
      }
    );

    return () => {
      client.disconnect();
    };
  }, [isAutoRefresh, mutate]);

  // Manual refresh handler
  const handleRefresh = useCallback(() => {
    mutate();
  }, [mutate]);

  // Default data for display when loading or empty
  const defaultData: DashboardData = {
    balance: {
      krw: 0,
      krw_locked: 0,
      crypto_value: 0,
      total: 0,
    },
    agents: [],
    trades: [],
    pnl_history: [],
    stats: {
      win_rate: 0,
      total_trades: 0,
      winning_trades: 0,
      losing_trades: 0,
      today_pnl: 0,
      today_pnl_rate: 0,
      total_pnl: 0,
      total_pnl_rate: 0,
    },
    market_prices: [],
    ...dashboardData,
  };

  const data = dashboardData || defaultData;

  // Get current price for position (from market prices or current price)
  const currentPrice = data.position?.current_price || data.position?.entry_price || 0;

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="sticky top-0 z-10 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="container flex h-14 items-center justify-between px-4">
          <div className="flex items-center gap-2">
            <h1 className="text-lg font-semibold">AutoCoin Dashboard</h1>
            {wsConnected && (
              <span className="flex h-2 w-2 rounded-full bg-green-500" />
            )}
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={() => setIsAutoRefresh(!isAutoRefresh)}
              className={`inline-flex items-center rounded-md px-3 py-1.5 text-xs font-medium transition-colors ${
                isAutoRefresh
                  ? "bg-primary text-primary-foreground"
                  : "bg-secondary text-secondary-foreground hover:bg-secondary/80"
              }`}
            >
              Auto Refresh: {isAutoRefresh ? "On" : "Off"}
            </button>
            <button
              onClick={handleRefresh}
              disabled={isLoading}
              className="inline-flex items-center rounded-md bg-secondary px-3 py-1.5 text-xs font-medium text-secondary-foreground hover:bg-secondary/80 disabled:opacity-50"
            >
              <IconRefresh className={`h-3 w-3 mr-1 ${isLoading ? "animate-spin" : ""}`} />
              Refresh
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="container py-6 px-4">
        {/* Error State */}
        {error && (
          <div className="mb-6 rounded-lg border border-red-500/20 bg-red-500/10 p-4">
            <p className="text-sm text-red-500">
              Failed to load dashboard data. {error instanceof Error ? error.message : "Unknown error"}
            </p>
          </div>
        )}

        {/* Loading State */}
        {isLoading && !dashboardData && (
          <div className="flex items-center justify-center py-12">
            <div className="flex items-center gap-2">
              <IconRefresh className="h-5 w-5 animate-spin text-muted-foreground" />
              <p className="text-sm text-muted-foreground">Loading dashboard data...</p>
            </div>
          </div>
        )}

        {/* Dashboard Grid */}
        <div className="grid grid-cols-1 gap-4 lg:gap-6">
          {/* Quick Stats Row */}
          <QuickStats stats={data.stats} />

          {/* Portfolio Summary and Position Cards */}
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 lg:gap-6">
            <PortfolioSummary balance={data.balance} position={data.position} />
            {data.position && <PositionCard position={data.position} currentPrice={currentPrice} />}
          </div>

          {/* PnL Chart and Recent Trades */}
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 lg:gap-6">
            <PnLChart data={data.pnl_history} />
            <RecentTrades trades={data.trades} />
          </div>

          {/* Agent Status Grid */}
          <AgentStatusGrid agents={data.agents} />
        </div>

        {/* Footer Info */}
        <div className="mt-8 border-t pt-4 text-center text-xs text-muted-foreground">
          <p>
            Last updated: {new Date().toLocaleString("ko-KR")}
            {isAutoRefresh && " • Auto-refresh enabled (5s)"}
          </p>
        </div>
      </main>
    </div>
  );
}
