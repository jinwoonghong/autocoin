"use client";

/**
 * PnLChart Component
 *
 * Displays PnL history over the last 7 days using Recharts
 * - Line chart for PnL percentage
 * - Area fill for visual impact
 * - Responsive design
 */

import React from "react";
import {
  LineChart,
  Line,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  ReferenceLine,
} from "recharts";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { IconLineChart } from "@/components/ui/icons";
import { PnLDataPoint } from "@/types/dashboard";
import { formatPercent } from "@/lib/utils";

interface PnLChartProps {
  data: PnLDataPoint[];
}

// Custom tooltip component
const CustomTooltip = ({ active, payload }: any) => {
  if (!active || !payload || !payload.length) {
    return null;
  }

  const data = payload[0].payload;
  const isPositive = data.pnl >= 0;

  return (
    <div className="rounded-lg border bg-background p-3 shadow-md">
      <p className="text-xs text-muted-foreground mb-1">{data.date}</p>
      <p className={`text-sm font-bold ${isPositive ? "text-green-500" : "text-red-500"}`}>
        {formatPercent(data.pnl_rate)}
      </p>
      <p className="text-xs text-muted-foreground">
        PnL: {isPositive ? "+" : ""}{data.pnl.toLocaleString()} KRW
      </p>
    </div>
  );
};

export function PnLChart({ data }: PnLChartProps) {
  // Transform data for chart display
  const chartData = data.map((point) => ({
    date: new Date(point.date).toLocaleDateString("ko-KR", { month: "short", day: "numeric" }),
    pnl_rate: point.pnl_rate,
    pnl: point.pnl,
    balance: point.balance,
  }));

  // Calculate min/max for Y-axis scaling
  const rates = chartData.map((d) => d.pnl_rate);
  const minRate = Math.min(...rates, -5);
  const maxRate = Math.max(...rates, 5);

  return (
    <Card className="col-span-full lg:col-span-2">
      <CardHeader className="pb-2">
        <CardTitle className="text-sm font-medium flex items-center gap-2">
          <IconLineChart className="h-4 w-4" />
          PnL History (7 Days)
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="h-[200px] w-full">
          {chartData.length > 0 ? (
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={chartData} margin={{ top: 10, right: 10, left: 0, bottom: 0 }}>
                <defs>
                  <linearGradient id="pnlGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="rgb(34 197 94)" stopOpacity={0.3} />
                    <stop offset="95%" stopColor="rgb(34 197 94)" stopOpacity={0} />
                  </linearGradient>
                  <linearGradient id="pnlGradientNegative" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="rgb(239 68 68)" stopOpacity={0.3} />
                    <stop offset="95%" stopColor="rgb(239 68 68)" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="date"
                  className="text-xs"
                  stroke="hsl(var(--muted-foreground))"
                />
                <YAxis
                  domain={[minRate - 1, maxRate + 1]}
                  tickFormatter={(value) => `${value.toFixed(1)}%`}
                  className="text-xs"
                  stroke="hsl(var(--muted-foreground))"
                />
                <Tooltip content={<CustomTooltip />} />
                <ReferenceLine y={0} stroke="hsl(var(--border))" strokeDasharray="3 3" />
                <Area
                  type="monotone"
                  dataKey="pnl_rate"
                  fill="url(#pnlGradient)"
                  stroke="none"
                />
                <Line
                  type="monotone"
                  dataKey="pnl_rate"
                  stroke="rgb(34 197 94)"
                  strokeWidth={2}
                  dot={false}
                  activeDot={{ r: 4 }}
                />
              </LineChart>
            </ResponsiveContainer>
          ) : (
            <div className="flex items-center justify-center h-full text-muted-foreground">
              <p className="text-sm">No PnL data available</p>
            </div>
          )}
        </div>

        {/* Summary stats below chart */}
        {chartData.length > 0 && (
          <div className="grid grid-cols-3 gap-4 mt-4 pt-4 border-t">
            <div className="text-center">
              <p className="text-xs text-muted-foreground">7-Day PnL</p>
              <p className={`text-sm font-bold ${
                chartData[chartData.length - 1].pnl_rate >= 0
                  ? "text-green-500"
                  : "text-red-500"
              }`}>
                {formatPercent(chartData[chartData.length - 1].pnl_rate)}
              </p>
            </div>
            <div className="text-center">
              <p className="text-xs text-muted-foreground">Best Day</p>
              <p className="text-sm font-bold text-green-500">
                {formatPercent(Math.max(...rates))}
              </p>
            </div>
            <div className="text-center">
              <p className="text-xs text-muted-foreground">Worst Day</p>
              <p className="text-sm font-bold text-red-500">
                {formatPercent(Math.min(...rates))}
              </p>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
