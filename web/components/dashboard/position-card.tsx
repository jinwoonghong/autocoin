/**
 * Position card component
 * Displays current trading position with PnL and action buttons
 */

"use client";

import { Card, CardContent, CardHeader, CardTitle, CardFooter } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { usePosition } from "@/lib/api";
import { formatKRW, formatPercent, getPnLBgColor } from "@/lib/utils";
import { TrendingUp, Target, ShieldAlert } from "lucide-react";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import { useState } from "react";

export function PositionCard() {
  const { data: position, loading, error, refetch } = usePosition();
  const [closing, setClosing] = useState(false);

  if (loading) {
    return <PositionCardSkeleton />;
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Current Position</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">{error}</p>
        </CardContent>
      </Card>
    );
  }

  if (!position) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Current Position</CardTitle>
        </CardHeader>
        <CardContent className="flex min-h-[200px] items-center justify-center">
          <div className="text-center">
            <ShieldAlert className="mx-auto h-12 w-12 text-muted-foreground" />
            <p className="mt-4 text-lg font-medium">No Active Position</p>
            <p className="text-sm text-muted-foreground">
              Waiting for trading signal...
            </p>
          </div>
        </CardContent>
      </Card>
    );
  }

  const pnlPercent = position.pnl_rate || 0;
  const isProfitable = pnlPercent > 0;

  async function handleClosePosition() {
    setClosing(true);
    try {
      await api.closePosition();
      refetch();
    } catch (error) {
      console.error("Failed to close position:", error);
    } finally {
      setClosing(false);
    }
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle className="flex items-center gap-2">
            <TrendingUp className="h-5 w-5" />
            Current Position
          </CardTitle>
          <Badge
            variant={position.status === "active" ? "success" : "secondary"}
          >
            {position.status}
          </Badge>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Market */}
        <div>
          <p className="text-sm text-muted-foreground">Market</p>
          <p className="text-xl font-bold">{position.market}</p>
        </div>

        {/* Entry Price */}
        <div className="grid grid-cols-2 gap-4">
          <div>
            <p className="text-sm text-muted-foreground">Entry Price</p>
            <p className="font-semibold">{formatKRW(position.entry_price)}</p>
          </div>
          <div>
            <p className="text-sm text-muted-foreground">Current Price</p>
            <p className="font-semibold">{formatKRW(position.current_price)}</p>
          </div>
        </div>

        {/* Amount */}
        <div>
          <p className="text-sm text-muted-foreground">Amount</p>
          <p className="font-semibold">
            {position.amount.toFixed(6)} {position.market.split("-")[1]}
          </p>
        </div>

        {/* PnL */}
        <div className={cn("rounded-lg border p-4", getPnLBgColor(pnlPercent))}>
          <div className="flex items-center justify-between">
            <span className="text-sm font-medium">PnL</span>
            <div className="text-right">
              <p className="text-lg font-bold">
                {position.pnl !== undefined ? formatKRW(position.pnl) : "-"}
              </p>
              <p className="text-sm font-medium">
                {formatPercent(pnlPercent)}
              </p>
            </div>
          </div>
        </div>

        {/* Targets */}
        <div className="grid grid-cols-2 gap-4 border-t pt-4">
          <div className="flex items-center gap-2">
            <Target className="h-4 w-4 text-green-500" />
            <div>
              <p className="text-xs text-muted-foreground">Take Profit</p>
              <p className="text-sm font-medium text-green-500">
                {formatKRW(position.take_profit)}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <ShieldAlert className="h-4 w-4 text-red-500" />
            <div>
              <p className="text-xs text-muted-foreground">Stop Loss</p>
              <p className="text-sm font-medium text-red-500">
                {formatKRW(position.stop_loss)}
              </p>
            </div>
          </div>
        </div>
      </CardContent>
      <CardFooter className="gap-2 border-t pt-4">
        <Button
          variant="destructive"
          className="flex-1"
          onClick={handleClosePosition}
          disabled={closing || position.status !== "active"}
        >
          {closing ? "Closing..." : "Close Position"}
        </Button>
      </CardFooter>
    </Card>
  );
}

function PositionCardSkeleton() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Current Position</CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        <Skeleton className="h-8 w-32" />
        <div className="grid grid-cols-2 gap-4">
          <Skeleton className="h-6 w-24" />
          <Skeleton className="h-6 w-24" />
        </div>
        <Skeleton className="h-20 w-full" />
        <Skeleton className="h-10 w-full" />
      </CardContent>
    </Card>
  );
}
