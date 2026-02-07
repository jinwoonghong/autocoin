"use client";

/**
 * QuickStats Component
 *
 * Displays quick statistics in small cards:
 * - Win rate
 * - Total trades
 * - Today's PnL
 * - Total PnL
 */

import React from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  Trophy,
  BarChart3,
  Calendar,
  TrendingUp,
  IconTrendingUp,
  IconTrendingDown,
} from "@/components/ui/icons";
import { QuickStats } from "@/types/dashboard";
import { formatKRW, formatPercent, getPnLColor, getPnLBgColor } from "@/lib/utils";

interface QuickStatsProps {
  stats: QuickStats;
}

interface StatCardProps {
  label: string;
  value: string | React.ReactNode;
  subtext?: string;
  icon: React.ElementType;
  variant?: "default" | "success" | "warning" | "destructive";
}

function StatCard({ label, value, subtext, icon: Icon, variant = "default" }: StatCardProps) {
  const variantStyles = {
    default: "bg-card",
    success: "bg-green-500/5 border-green-500/20",
    warning: "bg-yellow-500/5 border-yellow-500/20",
    destructive: "bg-red-500/5 border-red-500/20",
  };

  return (
    <Card className={variantStyles[variant]}>
      <CardContent className="p-4">
        <div className="flex items-center justify-between">
          <div className="space-y-1">
            <p className="text-xs text-muted-foreground">{label}</p>
            <p className="text-lg font-bold">{value}</p>
            {subtext && (
              <p className="text-xs text-muted-foreground">{subtext}</p>
            )}
          </div>
          <div className="p-2 rounded-lg bg-muted">
            <Icon className="h-4 w-4" />
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

export function QuickStats({ stats }: QuickStatsProps) {
  // Determine win rate badge color
  const getWinRateVariant = (rate: number): "success" | "warning" | "destructive" => {
    if (rate >= 60) return "success";
    if (rate >= 40) return "warning";
    return "destructive";
  };

  // Today's PnL trend
  const TodayPnLTrend = stats.today_pnl >= 0 ? IconTrendingUp : IconTrendingDown;

  return (
    <div className="col-span-full grid grid-cols-2 lg:grid-cols-4 gap-3">
      {/* Win Rate */}
      <StatCard
        label="Win Rate"
        value={
          <div className="flex items-center gap-2">
            <span>{formatPercent(stats.win_rate)}</span>
            <Badge variant={getWinRateVariant(stats.win_rate)} className="text-xs">
              {stats.winning_trades}W / {stats.losing_trades}L
            </Badge>
          </div>
        }
        subtext={`${stats.total_trades} total trades`}
        icon={Trophy}
      />

      {/* Total Trades */}
      <StatCard
        label="Total Trades"
        value={stats.total_trades.toString()}
        subtext={`${stats.winning_trades} wins, ${stats.losing_trades} losses`}
        icon={BarChart3}
      />

      {/* Today's PnL */}
      <StatCard
        label="Today's PnL"
        value={
          <div className="flex items-center gap-2">
            <span className={getPnLColor(stats.today_pnl)}>
              {stats.today_pnl >= 0 ? "+" : ""}{formatKRW(Math.abs(stats.today_pnl))}
            </span>
            <Badge className={getPnLBgColor(stats.today_pnl_rate)}>
              <TodayPnLTrend className="h-3 w-3 mr-1" />
              {formatPercent(stats.today_pnl_rate)}
            </Badge>
          </div>
        }
        subtext="Today"
        icon={Calendar}
        variant={stats.today_pnl >= 0 ? "success" : "destructive"}
      />

      {/* Total PnL */}
      <StatCard
        label="Total PnL"
        value={
          <span className={getPnLColor(stats.total_pnl)}>
            {stats.total_pnl >= 0 ? "+" : ""}{formatKRW(Math.abs(stats.total_pnl))}
          </span>
        }
        subtext={formatPercent(stats.total_pnl_rate)}
        icon={TrendingUp}
        variant={stats.total_pnl >= 0 ? "success" : "destructive"}
      />
    </div>
  );
}
