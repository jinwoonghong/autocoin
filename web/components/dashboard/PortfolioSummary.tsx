"use client";

/**
 * PortfolioSummary Component
 *
 * Displays portfolio summary including:
 * - Total asset value (KRW balance + crypto value)
 * - Available KRW balance
 * - Crypto asset value
 * - Asset allocation
 */

import React from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { IconWallet, IconBitcoin } from "@/components/ui/icons";
import { BalanceData, Position } from "@/types/dashboard";
import { formatKRW } from "@/lib/utils";

interface PortfolioSummaryProps {
  balance: BalanceData;
  position?: Position;
}

export function PortfolioSummary({ balance, position }: PortfolioSummaryProps) {
  // Calculate crypto percentage of total portfolio
  const cryptoPercent = balance.total > 0
    ? (balance.crypto_value / balance.total) * 100
    : 0;
  const krwPercent = balance.total > 0
    ? (balance.krw / balance.total) * 100
    : 0;

  return (
    <Card className="col-span-full lg:col-span-2">
      <CardHeader className="flex flex-row items-center justify-between pb-2">
        <CardTitle className="text-sm font-medium flex items-center gap-2">
          <IconWallet className="h-4 w-4" />
          Portfolio Summary
        </CardTitle>
        <Badge variant={position ? "success" : "secondary"} className="text-xs">
          {position ? "Active Position" : "No Position"}
        </Badge>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {/* Total Asset Value */}
          <div>
            <p className="text-2xl font-bold">{formatKRW(balance.total)}</p>
            <p className="text-xs text-muted-foreground">Total Asset Value</p>
          </div>

          <Separator />

          {/* Asset Breakdown */}
          <div className="grid grid-cols-2 gap-4">
            {/* KRW Balance */}
            <div className="space-y-1">
              <div className="flex items-center justify-between">
                <p className="text-xs text-muted-foreground">KRW Available</p>
                <span className="text-xs font-medium">{krwPercent.toFixed(1)}%</span>
              </div>
              <p className="text-lg font-semibold">{formatKRW(balance.krw)}</p>
              {balance.krw_locked > 0 && (
                <p className="text-xs text-muted-foreground">
                  Locked: {formatKRW(balance.krw_locked)}
                </p>
              )}
            </div>

            {/* Crypto Value */}
            <div className="space-y-1">
              <div className="flex items-center justify-between">
                <p className="text-xs text-muted-foreground flex items-center gap-1">
                  <IconBitcoin className="h-3 w-3" />
                  Crypto Value
                </p>
                <span className="text-xs font-medium">{cryptoPercent.toFixed(1)}%</span>
              </div>
              <p className="text-lg font-semibold">{formatKRW(balance.crypto_value)}</p>
              {position && (
                <p className="text-xs text-muted-foreground">
                  {position.market.replace("KRW-", "")}
                </p>
              )}
            </div>
          </div>

          {/* Asset Allocation Bar */}
          <div className="space-y-2">
            <p className="text-xs text-muted-foreground">Asset Allocation</p>
            <div className="h-2 w-full overflow-hidden rounded-full bg-muted">
              <div className="h-full flex">
                <div
                  className="bg-blue-500 transition-all duration-500"
                  style={{ width: `${krwPercent}%` }}
                />
                <div
                  className="bg-orange-500 transition-all duration-500"
                  style={{ width: `${cryptoPercent}%` }}
                />
              </div>
            </div>
            <div className="flex justify-between text-xs text-muted-foreground">
              <span>KRW</span>
              <span>Crypto</span>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
