"use client";

/**
 * Trades Page
 *
 * Trade history page with filtering, sorting, and pagination
 */

import React, { useState, useEffect, useCallback, useMemo } from "react";
import {
  RefreshCw,
  Download,
  Filter,
  ChevronLeft,
  ChevronRight,
  Search,
  X,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { Trade } from "@/types/dashboard";
import { dashboardAPI } from "@/lib/api-client";
import { formatPrice, formatVolume } from "@/lib/formatters";
import { cn } from "@/lib/utils";

// API base URL
const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";

// Filter options
type TradeFilter = "all" | "buy" | "sell" | "filled" | "pending" | "failed";
type SortField = "timestamp" | "market" | "price" | "total" | "profit";
type SortDirection = "asc" | "desc";

interface TradesResponse {
  success: boolean;
  data?: Trade[];
  error?: string;
}

export default function TradesPage() {
  const [trades, setTrades] = useState<Trade[]>([]);
  const [filteredTrades, setFilteredTrades] = useState<Trade[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Filter states
  const [filter, setFilter] = useState<TradeFilter>("all");
  const [search, setSearch] = useState("");
  const [sortField, setSortField] = useState<SortField>("timestamp");
  const [sortDirection, setSortDirection] = useState<SortDirection>("desc");

  // Pagination states
  const [currentPage, setCurrentPage] = useState(1);
  const itemsPerPage = 20;

  // Fetch trades
  const fetchTrades = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await dashboardAPI.getDashboardData();
      if (response.success && response.data?.trades) {
        setTrades(response.data.trades);
      } else {
        // Fallback to direct API call
        const res = await fetch(`${API_BASE}/api/trades`);
        if (res.ok) {
          const data: Trade[] = await res.json();
          setTrades(data);
        } else {
          throw new Error("Failed to fetch trades");
        }
      }
    } catch (err) {
      console.error("Error fetching trades:", err);
      setError(err instanceof Error ? err.message : "Failed to load trades");
    } finally {
      setLoading(false);
    }
  }, []);

  // Initial fetch
  useEffect(() => {
    fetchTrades();

    // Auto-refresh every 10 seconds
    const interval = setInterval(fetchTrades, 10000);
    return () => clearInterval(interval);
  }, [fetchTrades]);

  // Apply filters, search, and sorting
  const processedTrades = useMemo(() => {
    let result = [...trades];

    // Apply filter
    if (filter === "buy") {
      result = result.filter((t) => t.side === "buy");
    } else if (filter === "sell") {
      result = result.filter((t) => t.side === "sell");
    } else if (filter === "filled") {
      result = result.filter((t) => t.status === "filled");
    } else if (filter === "pending") {
      result = result.filter((t) => t.status === "pending");
    } else if (filter === "failed") {
      result = result.filter((t) => t.status === "failed");
    }

    // Apply search
    if (search) {
      const searchLower = search.toLowerCase();
      result = result.filter(
        (t) =>
          t.market.toLowerCase().includes(searchLower) ||
          t.id.toLowerCase().includes(searchLower)
      );
    }

    // Apply sorting
    result.sort((a, b) => {
      let comparison = 0;

      switch (sortField) {
        case "timestamp":
          comparison = a.timestamp - b.timestamp;
          break;
        case "market":
          comparison = a.market.localeCompare(b.market);
          break;
        case "price":
          comparison = a.price - b.price;
          break;
        case "total":
          comparison = a.total - b.total;
          break;
        case "profit":
          const aProfit = a.profit ?? 0;
          const bProfit = b.profit ?? 0;
          comparison = aProfit - bProfit;
          break;
      }

      return sortDirection === "asc" ? comparison : -comparison;
    });

    return result;
  }, [trades, filter, search, sortField, sortDirection]);

  // Update filtered trades
  useEffect(() => {
    setFilteredTrades(processedTrades);
    setCurrentPage(1); // Reset to first page when filters change
  }, [processedTrades]);

  // Pagination
  const totalPages = Math.ceil(filteredTrades.length / itemsPerPage);
  const startIndex = (currentPage - 1) * itemsPerPage;
  const endIndex = startIndex + itemsPerPage;
  const paginatedTrades = filteredTrades.slice(startIndex, endIndex);

  // Handle sort
  const handleSort = (field: SortField) => {
    if (sortField === field) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortDirection("desc");
    }
  };

  // Clear filters
  const clearFilters = () => {
    setFilter("all");
    setSearch("");
  };

  // Export to CSV
  const exportToCSV = () => {
    const headers = ["Time", "Market", "Side", "Price", "Amount", "Total", "Status", "PnL"];
    const rows = filteredTrades.map((t) => [
      new Date(t.timestamp).toLocaleString("ko-KR"),
      t.market,
      t.side.toUpperCase(),
      t.price.toFixed(2),
      t.volume.toFixed(8),
      t.total.toFixed(2),
      t.status,
      t.profit ? t.profit.toFixed(2) : "",
    ]);

    const csvContent = [
      headers.join(","),
      ...rows.map((row) => row.join(",")),
    ].join("\n");

    const blob = new Blob([csvContent], { type: "text/csv;charset=utf-8;" });
    const link = document.createElement("a");
    const url = URL.createObjectURL(blob);
    link.setAttribute("href", url);
    link.setAttribute("download", `trades_${new Date().toISOString().split("T")[0]}.csv`);
    link.style.visibility = "hidden";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  // Calculate stats
  const stats = useMemo(() => {
    const totalVolume = filteredTrades.reduce((sum, t) => sum + t.total, 0);
    const buyTrades = filteredTrades.filter((t) => t.side === "buy").length;
    const sellTrades = filteredTrades.filter((t) => t.side === "sell").length;
    const totalPnL = filteredTrades.reduce((sum, t) => sum + (t.profit ?? 0), 0);

    return {
      totalTrades: filteredTrades.length,
      totalVolume,
      buyTrades,
      sellTrades,
      totalPnL,
    };
  }, [filteredTrades]);

  return (
    <div className="py-6 px-4 max-w-7xl mx-auto">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Trade History</h1>
          <p className="text-muted-foreground">
            View and filter your trading history
          </p>
        </div>

        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={exportToCSV}
            disabled={filteredTrades.length === 0}
          >
            <Download className="h-4 w-4 mr-2" />
            Export
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={fetchTrades}
            disabled={loading}
          >
            <RefreshCw className={cn("h-4 w-4 mr-2", loading && "animate-spin")} />
            Refresh
          </Button>
        </div>
      </div>

      {/* Error State */}
      {error && (
        <Card className="mb-6 border-red-500/20 bg-red-500/10">
          <CardContent className="pt-6">
            <p className="text-sm text-red-500">{error}</p>
          </CardContent>
        </Card>
      )}

      {/* Stats Cards */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 mb-6">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Total Trades
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{stats.totalTrades}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Total Volume
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-lg font-bold">
              {formatPrice(stats.totalVolume)}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Buy / Sell
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-lg font-bold">
              {stats.buyTrades} / {stats.sellTrades}
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Total PnL
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className={cn(
              "text-lg font-bold",
              stats.totalPnL >= 0 ? "text-green-500" : "text-red-500"
            )}>
              {stats.totalPnL >= 0 ? "+" : ""}{formatPrice(stats.totalPnL)}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Filters */}
      <Card className="mb-6">
        <CardContent className="pt-6">
          <div className="flex flex-col sm:flex-row gap-4">
            {/* Search */}
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder="Search by market or trade ID..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="pl-10"
              />
              {search && (
                <button
                  onClick={() => setSearch("")}
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground"
                >
                  <X className="h-4 w-4" />
                </button>
              )}
            </div>

            {/* Filter */}
            <Select value={filter} onValueChange={(v) => setFilter(v as TradeFilter)}>
              <SelectTrigger className="w-full sm:w-[180px]">
                <SelectValue placeholder="Filter trades" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Trades</SelectItem>
                <SelectItem value="buy">Buy Only</SelectItem>
                <SelectItem value="sell">Sell Only</SelectItem>
                <SelectItem value="filled">Filled</SelectItem>
                <SelectItem value="pending">Pending</SelectItem>
                <SelectItem value="failed">Failed</SelectItem>
              </SelectContent>
            </Select>

            {/* Sort */}
            <Select
              value={sortField}
              onValueChange={(v) => setSortField(v as SortField)}
            >
              <SelectTrigger className="w-full sm:w-[180px]">
                <SelectValue placeholder="Sort by" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="timestamp">Sort by Time</SelectItem>
                <SelectItem value="market">Sort by Market</SelectItem>
                <SelectItem value="price">Sort by Price</SelectItem>
                <SelectItem value="total">Sort by Total</SelectItem>
                <SelectItem value="profit">Sort by PnL</SelectItem>
              </SelectContent>
            </Select>

            {/* Sort Direction */}
            <Button
              variant="outline"
              size="icon"
              onClick={() => setSortDirection(sortDirection === "asc" ? "desc" : "asc")}
            >
              {sortDirection === "asc" ? (
                <ChevronLeft className="h-4 w-4" />
              ) : (
                <ChevronRight className="h-4 w-4" />
              )}
            </Button>

            {/* Clear Filters */}
            {(filter !== "all" || search) && (
              <Button variant="ghost" size="sm" onClick={clearFilters}>
                <X className="h-4 w-4 mr-2" />
                Clear
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Trades Table */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Trades</span>
            <span className="text-sm font-normal text-muted-foreground">
              Showing {paginatedTrades.length} of {filteredTrades.length} trades
            </span>
          </CardTitle>
        </CardHeader>
        <CardContent>
          {loading && trades.length === 0 ? (
            <div className="flex items-center justify-center py-12">
              <RefreshCw className="h-6 w-6 animate-spin text-muted-foreground" />
            </div>
          ) : filteredTrades.length === 0 ? (
            <div className="text-center py-12">
              <p className="text-muted-foreground">No trades found</p>
            </div>
          ) : (
            <>
              {/* Desktop Table */}
              <div className="hidden md:block overflow-x-auto">
                <table className="w-full">
                  <thead>
                    <tr className="border-b">
                      <th
                        className="text-left p-3 font-medium cursor-pointer hover:bg-muted/50"
                        onClick={() => handleSort("timestamp")}
                      >
                        Time {sortField === "timestamp" && (
                          <span className="ml-1">{sortDirection === "asc" ? "↑" : "↓"}</span>
                        )}
                      </th>
                      <th
                        className="text-left p-3 font-medium cursor-pointer hover:bg-muted/50"
                        onClick={() => handleSort("market")}
                      >
                        Market {sortField === "market" && (
                          <span className="ml-1">{sortDirection === "asc" ? "↑" : "↓"}</span>
                        )}
                      </th>
                      <th className="text-left p-3 font-medium">Side</th>
                      <th
                        className="text-right p-3 font-medium cursor-pointer hover:bg-muted/50"
                        onClick={() => handleSort("price")}
                      >
                        Price {sortField === "price" && (
                          <span className="ml-1">{sortDirection === "asc" ? "↑" : "↓"}</span>
                        )}
                      </th>
                      <th className="text-right p-3 font-medium">Amount</th>
                      <th
                        className="text-right p-3 font-medium cursor-pointer hover:bg-muted/50"
                        onClick={() => handleSort("total")}
                      >
                        Total {sortField === "total" && (
                          <span className="ml-1">{sortDirection === "asc" ? "↑" : "↓"}</span>
                        )}
                      </th>
                      <th className="text-center p-3 font-medium">Status</th>
                      <th
                        className="text-right p-3 font-medium cursor-pointer hover:bg-muted/50"
                        onClick={() => handleSort("profit")}
                      >
                        PnL {sortField === "profit" && (
                          <span className="ml-1">{sortDirection === "asc" ? "↑" : "↓"}</span>
                        )}
                      </th>
                    </tr>
                  </thead>
                  <tbody>
                    {paginatedTrades.map((trade) => (
                      <tr key={trade.id} className="border-b hover:bg-muted/50">
                        <td className="p-3 text-sm text-muted-foreground">
                          {new Date(trade.timestamp).toLocaleString("ko-KR", {
                            month: "short",
                            day: "numeric",
                            hour: "2-digit",
                            minute: "2-digit",
                          })}
                        </td>
                        <td className="p-3 font-medium">{trade.market}</td>
                        <td className="p-3">
                          <Badge
                            variant={trade.side === "buy" ? "success" : "destructive"}
                            className="font-medium"
                          >
                            {trade.side.toUpperCase()}
                          </Badge>
                        </td>
                        <td className="p-3 text-right font-mono">
                          {formatPrice(trade.price)}
                        </td>
                        <td className="p-3 text-right font-mono">
                          {formatVolume(trade.volume)}
                        </td>
                        <td className="p-3 text-right font-mono">
                          {formatPrice(trade.total)}
                        </td>
                        <td className="p-3 text-center">
                          <Badge
                            variant={
                              trade.status === "filled"
                                ? "default"
                                : trade.status === "pending"
                                  ? "secondary"
                                  : "destructive"
                            }
                          >
                            {trade.status}
                          </Badge>
                        </td>
                        <td className="p-3 text-right font-mono">
                          {trade.profit !== undefined && trade.profit !== null ? (
                            <span
                              className={cn(
                                trade.profit >= 0 ? "text-green-500" : "text-red-500"
                              )}
                            >
                              {trade.profit >= 0 ? "+" : ""}{formatPrice(trade.profit)}
                            </span>
                          ) : (
                            <span className="text-muted-foreground">-</span>
                          )}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>

              {/* Mobile Card View */}
              <div className="md:hidden space-y-3">
                {paginatedTrades.map((trade) => (
                  <Card key={trade.id} className="p-4">
                    <div className="flex items-start justify-between mb-3">
                      <div>
                        <div className="font-semibold">{trade.market}</div>
                        <div className="text-sm text-muted-foreground">
                          {new Date(trade.timestamp).toLocaleString("ko-KR", {
                            month: "short",
                            day: "numeric",
                            hour: "2-digit",
                            minute: "2-digit",
                          })}
                        </div>
                      </div>
                      <Badge
                        variant={trade.side === "buy" ? "success" : "destructive"}
                      >
                        {trade.side.toUpperCase()}
                      </Badge>
                    </div>
                    <div className="grid grid-cols-2 gap-3 text-sm">
                      <div>
                        <span className="text-muted-foreground">Price:</span>{" "}
                        <span className="font-mono">{formatPrice(trade.price)}</span>
                      </div>
                      <div>
                        <span className="text-muted-foreground">Amount:</span>{" "}
                        <span className="font-mono">{formatVolume(trade.volume)}</span>
                      </div>
                      <div>
                        <span className="text-muted-foreground">Total:</span>{" "}
                        <span className="font-mono">{formatPrice(trade.total)}</span>
                      </div>
                      <div>
                        <span className="text-muted-foreground">Status:</span>{" "}
                        <Badge variant="outline" className="ml-1">
                          {trade.status}
                        </Badge>
                      </div>
                      {trade.profit !== undefined && trade.profit !== null && (
                        <div className="col-span-2">
                          <span className="text-muted-foreground">PnL:</span>{" "}
                          <span
                            className={cn(
                              "font-mono",
                              trade.profit >= 0 ? "text-green-500" : "text-red-500"
                            )}
                          >
                            {trade.profit >= 0 ? "+" : ""}{formatPrice(trade.profit)}
                          </span>
                        </div>
                      )}
                    </div>
                  </Card>
                ))}
              </div>

              {/* Pagination */}
              {totalPages > 1 && (
                <div className="flex items-center justify-between mt-4 pt-4 border-t">
                  <div className="text-sm text-muted-foreground">
                    Page {currentPage} of {totalPages}
                  </div>
                  <div className="flex gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setCurrentPage((p) => Math.max(1, p - 1))}
                      disabled={currentPage === 1}
                    >
                      <ChevronLeft className="h-4 w-4" />
                      Previous
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => setCurrentPage((p) => Math.min(totalPages, p + 1))}
                      disabled={currentPage === totalPages}
                    >
                      Next
                      <ChevronRight className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              )}
            </>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
