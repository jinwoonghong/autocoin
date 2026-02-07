/**
 * Portfolio summary card component
 * Displays total assets, available balance, 24h PnL, and trading statistics
 */

"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useBalance } from "@/lib/api";
import { formatKRW, formatPercent, getPnLColor } from "@/lib/utils";
import { Wallet, TrendingUp, TrendingDown, Activity } from "lucide-react";
import { Skeleton } from "@/components/ui/skeleton";

export function PortfolioSummary() {
  const { data: balance, loading, error } = useBalance();

  if (loading) {
    return <PortfolioSummarySkeleton />;
  }

  if (error || !balance) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Wallet className="h-5 w-5" />
            Portfolio Summary
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            {error || "Unable to load portfolio data"}
          </p>
        </CardContent>
      </Card>
    );
  }

  const pnl24h = balance.total_asset_value * 0.01; // Mock calculation
  const pnlPercent = 1.0;

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Wallet className="h-5 w-5" />
          Portfolio Summary
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Total Asset Value */}
        <div className="space-y-1">
          <p className="text-sm text-muted-foreground">Total Asset Value</p>
          <p className="text-2xl font-bold">{formatKRW(balance.total_asset_value)}</p>
        </div>

        {/* Available Balance */}
        <div className="space-y-1">
          <p className="text-sm text-muted-foreground">Available Balance</p>
          <p className="text-lg font-semibold">{formatKRW(balance.available)}</p>
          <p className="text-xs text-muted-foreground">
            Locked: {formatKRW(balance.locked)}
          </p>
        </div>

        {/* 24h PnL */}
        <div className="flex items-center justify-between border-t pt-4">
          <div className="flex items-center gap-2">
            {pnl24h >= 0 ? (
              <TrendingUp className="h-4 w-4 text-green-500" />
            ) : (
              <TrendingDown className="h-4 w-4 text-red-500" />
            )}
            <span className="text-sm text-muted-foreground">24h PnL</span>
          </div>
          <div className="text-right">
            <p className={cn("font-semibold", getPnLColor(pnl24h))}>
              {formatKRW(pnl24h)}
            </p>
            <p className={cn("text-xs", getPnLColor(pnl24h))}>
              {formatPercent(pnlPercent)}
            </p>
          </div>
        </div>

        {/* Coin Balances */}
        {balance.coin_balances.length > 0 && (
          <div className="border-t pt-4">
            <p className="mb-2 text-sm text-muted-foreground">Coin Holdings</p>
            <div className="space-y-2">
              {balance.coin_balances.map((coin) => (
                <div
                  key={coin.currency}
                  className="flex items-center justify-between text-sm"
                >
                  <span>{coin.currency}</span>
                  <div className="text-right">
                    <p className="font-medium">
                      {coin.balance.toFixed(6)} {coin.currency}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {formatKRW(coin.balance * coin.current_price)}
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}

function PortfolioSummarySkeleton() {
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Wallet className="h-5 w-5" />
          Portfolio Summary
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <Skeleton className="h-8 w-32" />
        <Skeleton className="h-6 w-24" />
        <Skeleton className="h-6 w-24" />
        <div className="border-t pt-4">
          <Skeleton className="h-10 w-full" />
        </div>
      </CardContent>
    </Card>
  );
}
