"use client";

/**
 * PositionCard Component
 *
 * Displays active position information including:
 * - Market and current price
 * - Entry price
 * - PnL (value and percentage)
 * - Stop loss and take profit levels
 */

import React from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { IconTrendingUp, IconTrendingDown, IconBitcoin } from "@/components/ui/icons";
import { Position } from "@/types/dashboard";
import { formatKRW, formatPercent, getPnLColor, getPnLBgColor } from "@/lib/utils";

interface PositionCardProps {
  position: Position;
  currentPrice: number;
}

export function PositionCard({ position, currentPrice }: PositionCardProps) {
  // Calculate current PnL
  const pnlValue = (currentPrice - position.entry_price) * position.amount;
  const pnlRate = ((currentPrice - position.entry_price) / position.entry_price) * 100;
  const isProfit = pnlValue >= 0;

  const TrendIcon = isProfit ? IconTrendingUp : IconTrendingDown;

  // Distance to stop loss / take profit
  const distanceToSL = position.stop_loss > 0
    ? ((currentPrice - position.stop_loss) / currentPrice) * 100
    : 0;
  const distanceToTP = position.take_profit > 0
    ? ((position.take_profit - currentPrice) / currentPrice) * 100
    : 0;

  return (
    <Card className="col-span-full lg:col-span-1">
      <CardHeader className="flex flex-row items-center justify-between pb-2">
        <CardTitle className="text-sm font-medium flex items-center gap-2">
          <IconBitcoin className="h-4 w-4 text-orange-500" />
          Active Position
        </CardTitle>
        <Badge className={getPnLBgColor(pnlRate)}>
          <TrendIcon className="h-3 w-3 mr-1" />
          {formatPercent(pnlRate)}
        </Badge>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {/* Market */}
          <div className="flex items-baseline justify-between">
            <div>
              <p className="text-2xl font-bold">{position.market.replace("KRW-", "")}</p>
              <p className="text-xs text-muted-foreground">Market</p>
            </div>
            <div className="text-right">
              <p className={`text-xl font-bold ${getPnLColor(pnlRate)}`}>
                {formatKRW(currentPrice)}
              </p>
              <p className="text-xs text-muted-foreground">Current Price</p>
            </div>
          </div>

          <Separator />

          {/* Position Details */}
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <p className="text-muted-foreground text-xs">Entry Price</p>
              <p className="font-medium">{formatKRW(position.entry_price)}</p>
            </div>
            <div>
              <p className="text-muted-foreground text-xs">Amount</p>
              <p className="font-medium">{position.amount.toFixed(6)}</p>
            </div>
            <div>
              <p className="text-muted-foreground text-xs">Stop Loss</p>
              <p className="font-medium text-red-500">
                {position.stop_loss > 0 ? formatKRW(position.stop_loss) : "Not Set"}
              </p>
            </div>
            <div>
              <p className="text-muted-foreground text-xs">Take Profit</p>
              <p className="font-medium text-green-500">
                {position.take_profit > 0 ? formatKRW(position.take_profit) : "Not Set"}
              </p>
            </div>
          </div>

          {/* PnL Summary */}
          <div className={`rounded-lg border p-3 ${getPnLBgColor(pnlRate)}`}>
            <div className="flex items-center justify-between">
              <span className="text-sm">Unrealized PnL</span>
              <div className="text-right">
                <p className={`font-bold ${getPnLColor(pnlRate)}`}>
                  {pnlValue >= 0 ? "+" : ""}{formatKRW(Math.abs(pnlValue))}
                </p>
                <p className={`text-xs ${getPnLColor(pnlRate)}`}>
                  {formatPercent(pnlRate)}
                </p>
              </div>
            </div>
          </div>

          {/* Distance indicators */}
          {(distanceToSL > 0 || distanceToTP > 0) && (
            <div className="space-y-2 text-xs">
              {distanceToSL > 0 && (
                <div className="flex justify-between">
                  <span className="text-muted-foreground">To Stop Loss</span>
                  <span className="text-red-500">-{distanceToSL.toFixed(2)}%</span>
                </div>
              )}
              {distanceToTP > 0 && (
                <div className="flex justify-between">
                  <span className="text-muted-foreground">To Take Profit</span>
                  <span className="text-green-500">+{distanceToTP.toFixed(2)}%</span>
                </div>
              )}
            </div>
          )}

          {/* Entry Time */}
          <div className="flex items-center gap-2 text-xs text-muted-foreground">
            <p>Entered: {new Date(position.entry_time).toLocaleString("ko-KR")}</p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
