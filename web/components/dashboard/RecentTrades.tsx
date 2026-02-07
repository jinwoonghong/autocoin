"use client";

/**
 * RecentTrades Component
 *
 * Displays the last 10 trades in a table format:
 * - Timestamp
 * - Market
 * - Side (Buy/Sell with color coding)
 * - Price
 * - Profit (for sells)
 */

import React from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { IconUp, IconDown } from "@/components/ui/icons";
import { Trade } from "@/types/dashboard";
import { formatKRW, formatTimestamp, getPnLColor } from "@/lib/utils";

interface RecentTradesProps {
  trades: Trade[];
  maxTrades?: number;
}

export function RecentTrades({ trades, maxTrades = 10 }: RecentTradesProps) {
  // Sort by timestamp descending and limit
  const sortedTrades = [...trades]
    .sort((a, b) => b.timestamp - a.timestamp)
    .slice(0, maxTrades);

  const getSideBadge = (side: Trade["side"]) => {
    return side === "buy" ? (
      <Badge variant="success" className="text-xs">
        <IconUp className="h-3 w-3 mr-1" />
        Buy
      </Badge>
    ) : (
      <Badge variant="destructive" className="text-xs">
        <IconDown className="h-3 w-3 mr-1" />
        Sell
      </Badge>
    );
  };

  const getStatusBadge = (status: Trade["status"]) => {
    switch (status) {
      case "filled":
        return <Badge variant="success" className="text-xs">Filled</Badge>;
      case "pending":
        return <Badge variant="warning" className="text-xs">Pending</Badge>;
      case "failed":
        return <Badge variant="destructive" className="text-xs">Failed</Badge>;
      default:
        return <Badge variant="secondary" className="text-xs">{status}</Badge>;
    }
  };

  return (
    <Card className="col-span-full lg:col-span-1">
      <CardHeader className="pb-3">
        <CardTitle className="text-sm font-medium">
          Recent Trades
        </CardTitle>
      </CardHeader>
      <CardContent className="p-0">
        {sortedTrades.length > 0 ? (
          <div className="overflow-x-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="text-xs">Time</TableHead>
                  <TableHead className="text-xs">Market</TableHead>
                  <TableHead className="text-xs">Side</TableHead>
                  <TableHead className="text-xs text-right">Price</TableHead>
                  <TableHead className="text-xs text-right">PnL</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {sortedTrades.map((trade) => (
                  <TableRow key={trade.id}>
                    <TableCell className="text-xs py-2">
                      {new Date(trade.timestamp).toLocaleTimeString("ko-KR", {
                        hour: "2-digit",
                        minute: "2-digit",
                      })}
                    </TableCell>
                    <TableCell className="text-xs py-2 font-medium">
                      {trade.market.replace("KRW-", "")}
                    </TableCell>
                    <TableCell className="text-xs py-2">
                      {getSideBadge(trade.side)}
                    </TableCell>
                    <TableCell className="text-xs py-2 text-right">
                      {formatKRW(trade.price)}
                    </TableCell>
                    <TableCell className={`text-xs py-2 text-right ${getPnLColor(trade.profit || 0)}`}>
                      {trade.profit !== undefined ? (
                        <>
                          {(trade.profit >= 0 ? "+" : "")}{formatKRW(Math.abs(trade.profit))}
                          {trade.profit_rate !== undefined && (
                            <span className="ml-1">
                              ({trade.profit_rate >= 0 ? "+" : ""}{trade.profit_rate.toFixed(2)}%)
                            </span>
                          )}
                        </>
                      ) : (
                        "-"
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </div>
        ) : (
          <div className="flex items-center justify-center py-8 text-muted-foreground">
            <p className="text-sm">No trades yet</p>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
