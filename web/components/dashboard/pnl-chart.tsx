/**
 * PnL Chart component using Recharts
 * Displays portfolio performance over time
 */

"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from "recharts";
import { TrendingUp } from "lucide-react";
import { formatKRW } from "@/lib/utils";
import { useEffect, useState } from "react";

interface ChartDataPoint {
  timestamp: number;
  value: number;
  label: string;
}

// Mock data generator
function generateMockData(days: number = 7): ChartDataPoint[] {
  const data: ChartDataPoint[] = [];
  let value = 5000000;

  for (let i = days; i >= 0; i--) {
    const date = new Date();
    date.setDate(date.getDate() - i);
    date.setHours(0, 0, 0, 0);

    // Random walk
    const change = (Math.random() - 0.45) * 100000;
    value = Math.max(100000, value + change);

    data.push({
      timestamp: date.getTime(),
      value,
      label: date.toLocaleDateString("ko-KR", { month: "short", day: "numeric" }),
    });
  }

  return data;
}

export function PnLChart() {
  const [timeRange, setTimeRange] = useState<"1D" | "7D" | "30D" | "ALL">("7D");
  const [data, setData] = useState<ChartDataPoint[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    setLoading(true);
    // Simulate API call
    setTimeout(() => {
      const days = timeRange === "1D" ? 1 : timeRange === "7D" ? 7 : timeRange === "30D" ? 30 : 90;
      setData(generateMockData(days));
      setLoading(false);
    }, 500);
  }, [timeRange]);

  const initialValue = data[0]?.value || 0;
  const currentValue = data[data.length - 1]?.value || 0;
  const totalPnL = currentValue - initialValue;
  const pnlPercent = ((totalPnL / initialValue) * 100);

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <TrendingUp className="h-5 w-5" />
            Portfolio Performance
          </CardTitle>
          <div className="flex gap-1">
            {(["1D", "7D", "30D", "ALL"] as const).map((range) => (
              <button
                key={range}
                onClick={() => setTimeRange(range)}
                className={cn(
                  "rounded px-2 py-1 text-xs font-medium transition-colors",
                  timeRange === range
                    ? "bg-primary text-primary-foreground"
                    : "text-muted-foreground hover:bg-muted"
                )}
              >
                {range}
              </button>
            ))}
          </div>
        </div>
      </CardHeader>
      <CardContent>
        {loading ? (
          <div className="flex h-[200px] items-center justify-center">
            <div className="text-sm text-muted-foreground">Loading chart...</div>
          </div>
        ) : (
          <>
            {/* Summary */}
            <div className="mb-4 flex items-center justify-between">
              <div>
                <p className="text-sm text-muted-foreground">Total PnL</p>
                <p className={cn("text-xl font-bold", totalPnL >= 0 ? "text-green-500" : "text-red-500")}>
                  {totalPnL >= 0 ? "+" : ""}{formatKRW(totalPnL)}
                </p>
              </div>
              <div className="text-right">
                <p className="text-sm text-muted-foreground">Return</p>
                <p className={cn("text-xl font-bold", pnlPercent >= 0 ? "text-green-500" : "text-red-500")}>
                  {pnlPercent >= 0 ? "+" : ""}{pnlPercent.toFixed(2)}%
                </p>
              </div>
            </div>

            {/* Chart */}
            <div className="h-[200px] w-full">
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={data} margin={{ top: 5, right: 5, bottom: 5, left: 5 }}>
                  <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                  <XAxis
                    dataKey="label"
                    className="text-xs text-muted-foreground"
                    tick={{ fontSize: 10 }}
                  />
                  <YAxis
                    tickFormatter={(value) => `${(value / 1000000).toFixed(1)}M`}
                    className="text-xs text-muted-foreground"
                    tick={{ fontSize: 10 }}
                  />
                  <Tooltip
                    contentStyle={{
                      backgroundColor: "hsl(var(--card))",
                      border: "1px solid hsl(var(--border))",
                      borderRadius: "0.5rem",
                    }}
                    labelFormatter={(label) => `Date: ${label}`}
                    formatter={(value: number) => [formatKRW(value), "Portfolio Value"]}
                  />
                  <Line
                    type="monotone"
                    dataKey="value"
                    stroke={totalPnL >= 0 ? "hsl(142 76% 36%)" : "hsl(0 84% 60%)"}
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 4 }}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
}
