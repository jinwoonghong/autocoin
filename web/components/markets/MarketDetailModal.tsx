/**
 * MarketDetailModal Component
 *
 * Displays detailed information for a selected market
 */

"use client";

import * as React from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import {
  TrendingUp,
  TrendingDown,
  Activity,
  DollarSign,
  BarChart3,
} from "lucide-react";
import { MarketDetail } from "@/types/markets";
import { cn, formatKRW, formatPercent, getPnLColor } from "@/lib/utils";
import { Line, LineChart, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";

export interface MarketDetailModalProps {
  market: MarketDetail | null;
  open: boolean;
  onClose: () => void;
  onBuy?: (market: string) => void;
  onSell?: (market: string) => void;
}

export function MarketDetailModal({
  market,
  open,
  onClose,
  onBuy,
  onSell,
}: MarketDetailModalProps) {
  if (!market) return null;

  const changeColor = getPnLColor(market.change_rate);
  const isPositive = market.change_rate >= 0;

  // Prepare chart data
  const chartData = market.candles_24h.map((candle) => ({
    time: new Date(candle.timestamp).toLocaleTimeString("ko-KR", {
      hour: "2-digit",
      minute: "2-digit",
    }),
    price: candle.close,
  }));

  return (
    <Dialog open={open} onOpenChange={onClose}>
      <DialogContent className="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <div className="flex items-center justify-between">
            <div>
              <DialogTitle className="text-2xl">
                {market.korean_name} ({market.english_name})
              </DialogTitle>
              <DialogDescription className="text-base mt-1">
                {market.market}
              </DialogDescription>
            </div>
            <Badge
              variant={isPositive ? "success" : "destructive"}
              className="text-sm px-3 py-1"
            >
              {isPositive ? (
                <TrendingUp className="h-4 w-4 mr-1" />
              ) : (
                <TrendingDown className="h-4 w-4 mr-1" />
              )}
              {formatPercent(market.change_rate * 100)}
            </Badge>
          </div>
        </DialogHeader>

        <div className="space-y-6">
          {/* Price Information */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-sm font-medium text-muted-foreground">
                  Current Price
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="text-xl font-bold">{formatKRW(market.current_price)}</div>
                <div className={cn("text-sm", changeColor)}>
                  {isPositive ? "+" : ""}
                  {formatKRW(market.change_amount)}
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-sm font-medium text-muted-foreground">
                  24h Volume
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="text-xl font-bold">
                  {formatKRW(market.volume_24h)}
                </div>
                <div className="text-sm text-muted-foreground">
                  {market.market}
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-sm font-medium text-muted-foreground">
                  24h High
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="text-xl font-bold text-green-500">
                  {formatKRW(market.high_24h)}
                </div>
                <div className="text-sm text-muted-foreground">
                  {((market.high_24h - market.low_24h) / market.low_24h * 100).toFixed(1)}%
                                   range
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="pb-2">
                <CardTitle className="text-sm font-medium text-muted-foreground">
                  24h Low
                </CardTitle>
              </CardHeader>
              <CardContent>
                <div className="text-xl font-bold text-red-500">
                  {formatKRW(market.low_24h)}
                </div>
                <div className="text-sm text-muted-foreground">
                  Prev: {formatKRW(market.prev_closing_price)}
                </div>
              </CardContent>
            </Card>
          </div>

          {/* Price Chart */}
          {chartData.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center gap-2">
                  <BarChart3 className="h-4 w-4" />
                  24h Price Chart
                </CardTitle>
              </CardHeader>
              <CardContent>
                <ResponsiveContainer width="100%" height={200}>
                  <LineChart data={chartData}>
                    <XAxis
                      dataKey="time"
                      interval="preserveStartEnd"
                      tick={{ fontSize: 12 }}
                      stroke="hsl(var(--muted-foreground))"
                    />
                    <YAxis
                      domain={["dataMin - 1000", "dataMax + 1000"]}
                      tick={{ fontSize: 12 }}
                      stroke="hsl(var(--muted-foreground))"
                      tickFormatter={(value) => formatKRW(value)}
                    />
                    <Tooltip
                      contentStyle={{
                        backgroundColor: "hsl(var(--card))",
                        border: "1px solid hsl(var(--border))",
                        borderRadius: "8px",
                      }}
                      labelStyle={{ color: "hsl(var(--foreground)) }}
                      formatter={(value: number) => [formatKRW(value), "Price"]}
                    />
                    <Line
                      type="monotone"
                      dataKey="price"
                      stroke={isPositive ? "#22c55e" : "#ef4444"}
                      strokeWidth={2}
                      dot={false}
                    />
                  </LineChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          )}

          {/* Trading Signals */}
          {market.signals && market.signals.length > 0 && (
            <Card>
              <CardHeader>
                <CardTitle className="text-base flex items-center gap-2">
                  <Activity className="h-4 w-4" />
                  Trading Signals
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-3">
                {market.signals.map((signal, index) => (
                  <div
                    key={index}
                    className={cn(
                      "flex items-start gap-3 p-3 rounded-lg border",
                      signal.type === "buy" &&
                        "bg-green-500/10 border-green-500/20",
                      signal.type === "sell" &&
                        "bg-red-500/10 border-red-500/20",
                      signal.type === "neutral" &&
                        "bg-muted"
                    )}
                  >
                    <Badge
                      variant={
                        signal.type === "buy"
                          ? "success"
                          : signal.type === "sell"
                            ? "destructive"
                            : "secondary"
                      }
                      className="mt-0.5"
                    >
                      {signal.type.toUpperCase()}
                    </Badge>
                    <div className="flex-1">
                      <p className="text-sm">{signal.reason}</p>
                      <div className="flex items-center gap-2 mt-1">
                        <div className="h-1.5 flex-1 bg-muted rounded-full overflow-hidden">
                          <div
                            className={cn(
                              "h-full rounded-full",
                              signal.type === "buy" && "bg-green-500",
                              signal.type === "sell" && "bg-red-500",
                              signal.type === "neutral" && "bg-muted-foreground"
                            )}
                            style={{ width: `${signal.confidence * 100}%` }}
                          />
                        </div>
                        <span className="text-xs text-muted-foreground">
                          {Math.round(signal.confidence * 100)}%
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </CardContent>
            </Card>
          )}

          <Separator />

          {/* Action Buttons */}
          <div className="flex gap-3">
            {onBuy && (
              <Button
                className="flex-1 h-12 text-lg"
                variant="success"
                onClick={() => onBuy(market.market)}
              >
                <DollarSign className="h-5 w-5 mr-2" />
                Buy {market.market.split("-")[1]}
              </Button>
            )}
            {onSell && (
              <Button
                className="flex-1 h-12 text-lg"
                variant="destructive"
                onClick={() => onSell(market.market)}
              >
                <DollarSign className="h-5 w-5 mr-2" />
                Sell {market.market.split("-")[1]}
              </Button>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
