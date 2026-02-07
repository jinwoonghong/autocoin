"use client";

/**
 * Backtest Page
 *
 * Backtesting configuration and results display
 */

import React, { useState, useCallback } from "react";
import {
  Play,
  RefreshCw,
  TrendingUp,
  TrendingDown,
  BarChart3,
  Settings,
  CheckCircle2,
  AlertCircle,
  Loader2,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@/components/ui/tabs";
import { Separator } from "@/components/ui/separator";
import { formatPrice, formatPercent } from "@/lib/formatters";
import { cn } from "@/lib/utils";

// API base URL
const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";

// Backtest configuration interface
interface BacktestConfig {
  market: string;
  strategy: string;
  startDate: string;
  endDate: string;
  initialBalance: number;
  commission: number;
  slippage: number;
}

// Backtest result interface
interface BacktestResult {
  success: boolean;
  config?: BacktestConfig;
  result?: {
    totalReturn: number;
    totalReturnRate: number;
    winRate: number;
    totalTrades: number;
    winningTrades: number;
    losingTrades: number;
    maxDrawdown: number;
    maxDrawdownRate: number;
    sharpeRatio: number;
    finalBalance: number;
    trades: Array<{
      market: string;
      side: "buy" | "sell";
      entryPrice: number;
      exitPrice: number;
      amount: number;
      profit: number;
      profitRate: number;
      entryTime: string;
      exitTime: string;
    }>;
    equityCurve: Array<{
      date: string;
      balance: number;
      return: number;
    }>;
  };
  error?: string;
}

const defaultConfig: BacktestConfig = {
  market: "KRW-BTC",
  strategy: "momentum",
  startDate: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)
    .toISOString()
    .split("T")[0],
  endDate: new Date().toISOString().split("T")[0],
  initialBalance: 1000000,
  commission: 0.0005,
  slippage: 0.001,
};

export default function BacktestPage() {
  const [config, setConfig] = useState<BacktestConfig>(defaultConfig);
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<BacktestResult | null>(null);
  const [activeTab, setActiveTab] = useState<"config" | "results">("config");

  // Run backtest
  const runBacktest = useCallback(async () => {
    setRunning(true);
    setResult(null);

    try {
      const response = await fetch(`${API_BASE}/api/backtest`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(config),
      });

      if (!response.ok) {
        throw new Error(`Failed to run backtest: ${response.statusText}`);
      }

      const data: BacktestResult = await response.json();
      setResult(data);

      if (data.success) {
        setActiveTab("results");
      }
    } catch (err) {
      console.error("Error running backtest:", err);
      setResult({
        success: false,
        error: err instanceof Error ? err.message : "Failed to run backtest",
      });
    } finally {
      setRunning(false);
    }
  }, [config]);

  // Reset config
  const resetConfig = () => {
    setConfig(defaultConfig);
    setResult(null);
    setActiveTab("config");
  };

  return (
    <div className="py-6 px-4 max-w-7xl mx-auto">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Backtesting</h1>
          <p className="text-muted-foreground">
            Test trading strategies with historical data
          </p>
        </div>

        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={resetConfig}>
            <RefreshCw className="h-4 w-4 mr-2" />
            Reset
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Configuration Panel */}
        <div className="lg:col-span-1">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Settings className="h-5 w-5" />
                Configuration
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-5">
              {/* Market Selection */}
              <div className="space-y-2">
                <Label htmlFor="market">Market</Label>
                <Select
                  value={config.market}
                  onValueChange={(value) =>
                    setConfig({ ...config, market: value })
                  }
                >
                  <SelectTrigger id="market">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="KRW-BTC">KRW-BTC (Bitcoin)</SelectItem>
                    <SelectItem value="KRW-ETH">KRW-ETH (Ethereum)</SelectItem>
                    <SelectItem value="KRW-XRP">KRW-XRP (Ripple)</SelectItem>
                    <SelectItem value="KRW-SOL">KRW-SOL (Solana)</SelectItem>
                    <SelectItem value="KRW-ADA">KRW-ADA (Cardano)</SelectItem>
                    <SelectItem value="KRW-AVAX">KRW-AVAX (Avalanche)</SelectItem>
                    <SelectItem value="KRW-DOGE">KRW-DOGE (Dogecoin)</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              {/* Strategy Selection */}
              <div className="space-y-2">
                <Label htmlFor="strategy">Strategy</Label>
                <Select
                  value={config.strategy}
                  onValueChange={(value) =>
                    setConfig({ ...config, strategy: value })
                  }
                >
                  <SelectTrigger id="strategy">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="momentum">
                      Momentum Following
                    </SelectItem>
                    <SelectItem value="multi-indicator">
                      Multi-Indicator
                    </SelectItem>
                    <SelectItem value="rsi">RSI Strategy</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <Separator />

              {/* Date Range */}
              <div className="space-y-2">
                <Label htmlFor="startDate">Start Date</Label>
                <Input
                  id="startDate"
                  type="date"
                  value={config.startDate}
                  onChange={(e) =>
                    setConfig({ ...config, startDate: e.target.value })
                  }
                  max={config.endDate}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="endDate">End Date</Label>
                <Input
                  id="endDate"
                  type="date"
                  value={config.endDate}
                  onChange={(e) =>
                    setConfig({ ...config, endDate: e.target.value })
                  }
                  min={config.startDate}
                />
              </div>

              {/* Quick Date Range Buttons */}
              <div className="flex flex-wrap gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => {
                    const now = new Date();
                    const days = 7;
                    const start = new Date(
                      now.getTime() - days * 24 * 60 * 60 * 1000
                    );
                    setConfig({
                      ...config,
                      startDate: start.toISOString().split("T")[0],
                      endDate: now.toISOString().split("T")[0],
                    });
                  }}
                >
                  7D
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => {
                    const now = new Date();
                    const days = 30;
                    const start = new Date(
                      now.getTime() - days * 24 * 60 * 60 * 1000
                    );
                    setConfig({
                      ...config,
                      startDate: start.toISOString().split("T")[0],
                      endDate: now.toISOString().split("T")[0],
                    });
                  }}
                >
                  30D
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => {
                    const now = new Date();
                    const days = 90;
                    const start = new Date(
                      now.getTime() - days * 24 * 60 * 60 * 1000
                    );
                    setConfig({
                      ...config,
                      startDate: start.toISOString().split("T")[0],
                      endDate: now.toISOString().split("T")[0],
                    });
                  }}
                >
                  90D
                </Button>
              </div>

              <Separator />

              {/* Initial Balance */}
              <div className="space-y-2">
                <Label htmlFor="initialBalance">Initial Balance (KRW)</Label>
                <Input
                  id="initialBalance"
                  type="number"
                  value={config.initialBalance}
                  onChange={(e) =>
                    setConfig({
                      ...config,
                      initialBalance: parseFloat(e.target.value) || 0,
                    })
                  }
                  min="10000"
                  step="10000"
                />
              </div>

              {/* Commission Rate */}
              <div className="space-y-2">
                <Label htmlFor="commission">Commission Rate</Label>
                <div className="flex items-center gap-2">
                  <Input
                    id="commission"
                    type="number"
                    value={(config.commission * 100).toFixed(2)}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        commission: (parseFloat(e.target.value) || 0) / 100,
                      })
                    }
                    min="0"
                    max="1"
                    step="0.01"
                    className="flex-1"
                  />
                  <span className="text-sm text-muted-foreground">%</span>
                </div>
              </div>

              {/* Slippage */}
              <div className="space-y-2">
                <Label htmlFor="slippage">Slippage</Label>
                <div className="flex items-center gap-2">
                  <Input
                    id="slippage"
                    type="number"
                    value={(config.slippage * 100).toFixed(2)}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        slippage: (parseFloat(e.target.value) || 0) / 100,
                      })
                    }
                    min="0"
                    max="1"
                    step="0.01"
                    className="flex-1"
                  />
                  <span className="text-sm text-muted-foreground">%</span>
                </div>
              </div>

              {/* Advanced Settings Toggle */}
              <div className="flex items-center justify-between">
                <Label htmlFor="advanced">Advanced Settings</Label>
                <Switch id="advanced" />
              </div>

              {/* Run Button */}
              <Button
                className="w-full"
                size="lg"
                onClick={runBacktest}
                disabled={running}
              >
                {running ? (
                  <>
                    <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                    Running...
                  </>
                ) : (
                  <>
                    <Play className="h-4 w-4 mr-2" />
                    Run Backtest
                  </>
                )}
              </Button>
            </CardContent>
          </Card>
        </div>

        {/* Results Panel */}
        <div className="lg:col-span-2">
          {result && result.error && (
            <Card className="border-red-500/20 bg-red-500/10 mb-4">
              <CardContent className="pt-6">
                <div className="flex items-start gap-3">
                  <AlertCircle className="h-5 w-5 text-red-500 mt-0.5" />
                  <div>
                    <h3 className="font-semibold text-red-500 mb-1">
                      Backtest Failed
                    </h3>
                    <p className="text-sm text-red-500/80">{result.error}</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          )}

          {!result ? (
            <Card className="h-full min-h-[400px] flex items-center justify-center">
              <CardContent className="text-center py-12">
                <BarChart3 className="h-12 w-12 mx-auto mb-4 text-muted-foreground/50" />
                <h3 className="text-lg font-semibold mb-2">
                  No Backtest Results
                </h3>
                <p className="text-sm text-muted-foreground">
                  Configure your backtest parameters and click "Run Backtest" to
                  see results here.
                </p>
              </CardContent>
            </Card>
          ) : result.success && result.result ? (
            <div className="space-y-4">
              {/* Summary Stats */}
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-xs font-medium text-muted-foreground uppercase">
                      Total Return
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="flex items-center gap-2">
                      {result.result.totalReturn >= 0 ? (
                        <TrendingUp className="h-4 w-4 text-green-500" />
                      ) : (
                        <TrendingDown className="h-4 w-4 text-red-500" />
                      )}
                      <span
                        className={cn(
                          "text-xl font-bold",
                          result.result.totalReturn >= 0
                            ? "text-green-500"
                            : "text-red-500"
                        )}
                      >
                        {formatPercent(result.result.totalReturnRate)}
                      </span>
                    </div>
                    <div className="text-sm text-muted-foreground">
                      {formatPrice(result.result.totalReturn)}
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-xs font-medium text-muted-foreground uppercase">
                      Win Rate
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-xl font-bold">
                      {formatPercent(result.result.winRate)}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      {result.result.winningTrades}W /{" "}
                      {result.result.losingTrades}L
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-xs font-medium text-muted-foreground uppercase">
                      Max Drawdown
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-xl font-bold text-red-500">
                      {formatPercent(result.result.maxDrawdownRate)}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      {formatPrice(result.result.maxDrawdown)}
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader className="pb-2">
                    <CardTitle className="text-xs font-medium text-muted-foreground uppercase">
                      Sharpe Ratio
                    </CardTitle>
                  </CardHeader>
                  <CardContent>
                    <div className="text-xl font-bold">
                      {result.result.sharpeRatio.toFixed(2)}
                    </div>
                    <div className="text-sm text-muted-foreground">
                      {result.result.sharpeRatio > 1 ? "Good" : "Low"}
                    </div>
                  </CardContent>
                </Card>
              </div>

              {/* Detailed Results */}
              <Tabs defaultValue="overview" className="w-full">
                <TabsList className="grid w-full grid-cols-3">
                  <TabsTrigger value="overview">Overview</TabsTrigger>
                  <TabsTrigger value="trades">Trades</TabsTrigger>
                  <TabsTrigger value="equity">Equity Curve</TabsTrigger>
                </TabsList>

                <TabsContent value="overview" className="space-y-4">
                  <Card>
                    <CardHeader>
                      <CardTitle>Performance Summary</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="grid grid-cols-2 gap-4 text-sm">
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Initial Balance:
                          </span>
                          <span className="font-medium">
                            {formatPrice(config.initialBalance)}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Final Balance:
                          </span>
                          <span className="font-medium">
                            {formatPrice(result.result.finalBalance)}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Total Trades:
                          </span>
                          <span className="font-medium">
                            {result.result.totalTrades}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Winning Trades:
                          </span>
                          <span className="font-medium text-green-500">
                            {result.result.winningTrades}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Losing Trades:
                          </span>
                          <span className="font-medium text-red-500">
                            {result.result.losingTrades}
                          </span>
                        </div>
                        <div className="flex justify-between">
                          <span className="text-muted-foreground">
                            Commission:
                          </span>
                          <span className="font-medium">
                            {formatPercent(config.commission)}
                          </span>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </TabsContent>

                <TabsContent value="trades">
                  <Card>
                    <CardHeader>
                      <CardTitle>Trade History</CardTitle>
                    </CardHeader>
                    <CardContent>
                      {result.result.trades.length === 0 ? (
                        <div className="text-center py-8 text-muted-foreground">
                          No trades executed
                        </div>
                      ) : (
                        <div className="overflow-x-auto">
                          <table className="w-full text-sm">
                            <thead>
                              <tr className="border-b">
                                <th className="text-left p-2">Time</th>
                                <th className="text-left p-2">Market</th>
                                <th className="text-left p-2">Side</th>
                                <th className="text-right p-2">Entry</th>
                                <th className="text-right p-2">Exit</th>
                                <th className="text-right p-2">Profit</th>
                              </tr>
                            </thead>
                            <tbody>
                              {result.result.trades.map((trade, idx) => (
                                <tr key={idx} className="border-b">
                                  <td className="p-2 text-muted-foreground">
                                    {new Date(trade.entryTime).toLocaleDateString(
                                      "ko-KR"
                                    )}
                                  </td>
                                  <td className="p-2">{trade.market}</td>
                                  <td className="p-2">
                                    <Badge
                                      variant={
                                        trade.side === "buy"
                                          ? "success"
                                          : "destructive"
                                      }
                                    >
                                      {trade.side.toUpperCase()}
                                    </Badge>
                                  </td>
                                  <td className="p-2 text-right font-mono">
                                    {formatPrice(trade.entryPrice)}
                                  </td>
                                  <td className="p-2 text-right font-mono">
                                    {formatPrice(trade.exitPrice)}
                                  </td>
                                  <td
                                    className={cn(
                                      "p-2 text-right font-mono",
                                      trade.profit >= 0
                                        ? "text-green-500"
                                        : "text-red-500"
                                    )}
                                  >
                                    {formatPercent(trade.profitRate)}
                                  </td>
                                </tr>
                              ))}
                            </tbody>
                          </table>
                        </div>
                      )}
                    </CardContent>
                  </Card>
                </TabsContent>

                <TabsContent value="equity">
                  <Card>
                    <CardHeader>
                      <CardTitle>Equity Curve</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <div className="h-64 flex items-center justify-center bg-muted/10 rounded-lg">
                        <div className="text-center">
                          <BarChart3 className="h-8 w-8 mx-auto mb-2 text-muted-foreground" />
                          <p className="text-sm text-muted-foreground">
                            Equity curve chart visualization
                          </p>
                        </div>
                      </div>
                    </CardContent>
                  </Card>
                </TabsContent>
              </Tabs>
            </div>
          ) : null}
        </div>
      </div>
    </div>
  );
}
