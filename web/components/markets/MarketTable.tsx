/**
 * MarketTable Component
 *
 * Displays live price table for top KRW market coins
 */

"use client";

import * as React from "react";
import { ArrowUpDown, ArrowUp, ArrowDown, Star } from "lucide-react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { PriceCell, Sparkline } from "./PriceCell";
import { Button } from "@/components/ui/button";
import { CoinPriceData, SortColumn, SortDirection } from "@/types/markets";
import { cn, formatKRW, getPnLColor } from "@/lib/utils";

export interface MarketTableProps {
  markets: CoinPriceData[];
  sortColumn: SortColumn;
  sortDirection: SortDirection;
  onSort: (column: SortColumn) => void;
  onRowClick?: (market: string) => void;
  favorites?: Set<string>;
  onToggleFavorite?: (market: string) => void;
  loading?: boolean;
}

// Helper function to get sort icon
function SortIcon({
  column,
  sortColumn,
  sortDirection,
}: {
  column: SortColumn;
  sortColumn: SortColumn;
  sortDirection: SortDirection;
}) {
  if (column !== sortColumn) {
    return <ArrowUpDown className="h-4 w-4 opacity-20" />;
  }
  return sortDirection === "asc" ? (
    <ArrowUp className="h-4 w-4" />
  ) : (
    <ArrowDown className="h-4 w-4" />
  );
}

export function MarketTable({
  markets,
  sortColumn,
  sortDirection,
  onSort,
  onRowClick,
  favorites = new Set(),
  onToggleFavorite,
  loading = false,
}: MarketTableProps) {
  const handleSort = (column: SortColumn) => {
    if (sortColumn === column) {
      // Toggle direction if same column
      onSort(column);
    } else {
      // New column, default to desc for price/volume, asc for market name
      onSort(column);
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="flex flex-col items-center gap-2">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent" />
          <p className="text-sm text-muted-foreground">Loading markets...</p>
        </div>
      </div>
    );
  }

  if (markets.length === 0) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-muted-foreground">No markets found</p>
      </div>
    );
  }

  return (
    <div className="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-12 text-center">#</TableHead>
            <TableHead>
              <Button
                variant="ghost"
                size="sm"
                className="h-8 px-2 font-medium"
                onClick={() => handleSort("market")}
              >
                Market
                <SortIcon
                  column="market"
                  sortColumn={sortColumn}
                  sortDirection={sortDirection}
                />
              </Button>
            </TableHead>
            <TableHead className="text-right">
              <Button
                variant="ghost"
                size="sm"
                className="h-8 px-2 font-medium ml-auto"
                onClick={() => handleSort("price")}
              >
                Price
                <SortIcon
                  column="price"
                  sortColumn={sortColumn}
                  sortDirection={sortDirection}
                />
              </Button>
            </TableHead>
            <TableHead className="text-right">
              <Button
                variant="ghost"
                size="sm"
                className="h-8 px-2 font-medium ml-auto"
                onClick={() => handleSort("change")}
              >
                Change
                <SortIcon
                  column="change"
                  sortColumn={sortColumn}
                  sortDirection={sortDirection}
                />
              </Button>
            </TableHead>
            <TableHead className="text-right">
              <Button
                variant="ghost"
                size="sm"
                className="h-8 px-2 font-medium ml-auto"
                onClick={() => handleSort("volume")}
              >
                Volume
                <SortIcon
                  column="volume"
                  sortColumn={sortColumn}
                  sortDirection={sortDirection}
                />
              </Button>
            </TableHead>
            <TableHead className="text-right">24h High/Low</TableHead>
            <TableHead className="text-center w-20">Signal</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {markets.map((market, index) => {
            const changeColor = getPnLColor(market.change_rate);
            const isFavorite = favorites.has(market.market);

            return (
              <TableRow
                key={market.market}
                className={cn(
                  "cursor-pointer transition-colors",
                  onRowClick && "hover:bg-muted/50"
                )}
                onClick={() => onRowClick?.(market.market)}
              >
                <TableCell className="text-center font-medium text-muted-foreground">
                  {index + 1}
                </TableCell>
                <TableCell>
                  <div className="flex items-center gap-2">
                    {onToggleFavorite && (
                      <Button
                        variant="ghost"
                        size="icon"
                        className={cn(
                          "h-6 w-6",
                          isFavorite && "text-yellow-500"
                        )}
                        onClick={(e) => {
                          e.stopPropagation();
                          onToggleFavorite(market.market);
                        }}
                      >
                        <Star
                          className={cn(
                            "h-4 w-4",
                            isFavorite ? "fill-current" : "fill-none"
                          )}
                        />
                      </Button>
                    )}
                    <div>
                      <div className="font-medium">{market.market}</div>
                      {market.korean_name && (
                        <div className="text-xs text-muted-foreground hidden sm:block">
                          {market.korean_name}
                        </div>
                      )}
                    </div>
                  </div>
                </TableCell>
                <TableCell className="text-right">
                  <PriceCell
                    price={market.trade_price}
                    changeRate={market.change_rate}
                    format="full"
                    className="items-end"
                  />
                </TableCell>
                <TableCell className={cn("text-right font-medium", changeColor)}>
                  {market.change_rate >= 0 ? "+" : ""}
                  {(market.change_rate * 100).toFixed(2)}%
                </TableCell>
                <TableCell className="text-right text-muted-foreground">
                  {formatKRW(market.volume)}
                </TableCell>
                <TableCell className="text-right text-sm">
                  <div className="flex flex-col items-end gap-1">
                    <span className="text-green-500">
                      {formatKRW(market.high_price)}
                    </span>
                    <span className="text-red-500">
                      {formatKRW(market.low_price)}
                    </span>
                  </div>
                </TableCell>
                <TableCell className="text-center">
                  {Math.abs(market.change_rate) > 0.05 ? (
                    <Badge
                      variant={market.change_rate > 0 ? "success" : "destructive"}
                      className="text-xs"
                    >
                      {market.change_rate > 0 ? "BUY" : "SELL"}
                    </Badge>
                  ) : (
                    <span className="text-muted-foreground">-</span>
                  )}
                </TableCell>
              </TableRow>
            );
          })}
        </TableBody>
      </Table>
    </div>
  );
}

/**
 * Mobile card view for markets table
 */
export interface MarketCardViewProps {
  markets: CoinPriceData[];
  onRowClick?: (market: string) => void;
  favorites?: Set<string>;
  onToggleFavorite?: (market: string) => void;
}

export function MarketCardView({
  markets,
  onRowClick,
  favorites = new Set(),
  onToggleFavorite,
}: MarketCardViewProps) {
  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
      {markets.map((market, index) => {
        const changeColor = getPnLColor(market.change_rate);
        const isFavorite = favorites.has(market.market);

        return (
          <div
            key={market.market}
            className="rounded-lg border p-4 cursor-pointer hover:bg-muted/50 transition-colors"
            onClick={() => onRowClick?.(market.market)}
          >
            <div className="flex items-start justify-between mb-2">
              <div>
                <div className="font-medium">{market.market}</div>
                {market.korean_name && (
                  <div className="text-xs text-muted-foreground">
                    {market.korean_name}
                  </div>
                )}
              </div>
              <div className="flex items-center gap-1">
                <span className="text-xs text-muted-foreground">
                  #{index + 1}
                </span>
                {onToggleFavorite && (
                  <Button
                    variant="ghost"
                    size="icon"
                    className={cn(
                      "h-6 w-6",
                      isFavorite && "text-yellow-500"
                    )}
                    onClick={(e) => {
                      e.stopPropagation();
                      onToggleFavorite(market.market);
                    }}
                  >
                    <Star
                      className={cn(
                        "h-4 w-4",
                        isFavorite ? "fill-current" : "fill-none"
                      )}
                    />
                  </Button>
                )}
              </div>
            </div>

            <div className="flex items-end justify-between">
              <PriceCell
                price={market.trade_price}
                changeRate={market.change_rate}
                format="full"
              />
              <div className={cn("text-lg font-medium", changeColor)}>
                {market.change_rate >= 0 ? "+" : ""}
                {(market.change_rate * 100).toFixed(2)}%
              </div>
            </div>

            <div className="flex items-center justify-between mt-2 text-xs text-muted-foreground">
              <span>Vol: {formatKRW(market.volume)}</span>
              <div className="flex gap-2">
                <span className="text-green-500">
                  H: {formatKRW(market.high_price)}
                </span>
                <span className="text-red-500">
                  L: {formatKRW(market.low_price)}
                </span>
              </div>
            </div>

            {Math.abs(market.change_rate) > 0.05 && (
              <div className="mt-2">
                <Badge
                  variant={market.change_rate > 0 ? "success" : "destructive"}
                  className="text-xs w-full justify-center"
                >
                  {market.change_rate > 0 ? "BUY Signal" : "SELL Signal"}
                </Badge>
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
