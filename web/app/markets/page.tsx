/**
 * Markets Page
 *
 * Live price table for top KRW market coins with sorting, filtering,
 * and real-time updates via WebSocket.
 */

"use client";

import * as React from "react";
import { RefreshCw, Wifi, WifiOff, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { MarketTable, MarketCardView } from "@/components/markets/MarketTable";
import { MarketFilters } from "@/components/markets/MarketFilters";
import { MarketDetailModal } from "@/components/markets/MarketDetailModal";
import {
  CoinPriceData,
  MarketFilter,
  SortColumn,
  SortDirection,
} from "@/types/markets";
import { dashboardAPI } from "@/lib/api-client";
import { cn } from "@/lib/utils";

// WebSocket message types
type WSMessage =
  | { type: "price_update"; data: CoinPriceData }
  | { type: "connected" }
  | { type: "disconnected" };

export default function MarketsPage() {
  // State
  const [markets, setMarkets] = React.useState<CoinPriceData[]>([]);
  const [filter, setFilter] = React.useState<MarketFilter>("top20");
  const [search, setSearch] = React.useState("");
  const [sortColumn, setSortColumn] = React.useState<SortColumn>("volume");
  const [sortDirection, setSortDirection] = React.useState<SortDirection>("desc");
  const [loading, setLoading] = React.useState(true);
  const [wsConnected, setWsConnected] = React.useState(false);
  const [selectedMarket, setSelectedMarket] = React.useState<string | null>(null);
  const [marketDetail, setMarketDetail] = React.useState<any>(null);
  const [favorites, setFavorites] = React.useState<Set<string>>(new Set());

  // Load favorites from localStorage
  React.useEffect(() => {
    const stored = localStorage.getItem("favorite_markets");
    if (stored) {
      try {
        setFavorites(new Set(JSON.parse(stored)));
      } catch {
        console.error("Failed to parse favorites");
      }
    }
  }, []);

  // Save favorites to localStorage
  const toggleFavorite = (market: string) => {
    setFavorites((prev) => {
      const newFavorites = new Set(prev);
      if (newFavorites.has(market)) {
        newFavorites.delete(market);
      } else {
        newFavorites.add(market);
      }
      localStorage.setItem(
        "favorite_markets",
        JSON.stringify(Array.from(newFavorites))
      );
      return newFavorites;
    });
  };

  // Fetch initial market data
  const fetchMarkets = React.useCallback(async () => {
    setLoading(true);
    try {
      const response = await dashboardAPI.getDashboardData();
      if (response.success && response.data?.market_prices) {
        setMarkets(response.data.market_prices);
      }
    } catch (error) {
      console.error("Failed to fetch markets:", error);
    } finally {
      setLoading(false);
    }
  }, []);

  // Initial fetch
  React.useEffect(() => {
    fetchMarkets();

    // Set up polling for price updates (fallback for WebSocket)
    const interval = setInterval(() => {
      fetchMarkets();
    }, 3000); // Poll every 3 seconds

    return () => clearInterval(interval);
  }, [fetchMarkets]);

  // Sort markets
  const sortedMarkets = React.useMemo(() => {
    let result = [...markets];

    // Apply filter
    if (filter === "top10") {
      result = result.slice(0, 10);
    } else if (filter === "top20") {
      result = result.slice(0, 20);
    } else if (filter === "favorites") {
      result = result.filter((m) => favorites.has(m.market));
    }

    // Apply search
    if (search) {
      const searchLower = search.toLowerCase();
      result = result.filter(
        (m) =>
          m.market.toLowerCase().includes(searchLower) ||
          m.korean_name?.toLowerCase().includes(searchLower) ||
          m.english_name?.toLowerCase().includes(searchLower)
      );
    }

    // Apply sort
    result.sort((a, b) => {
      let comparison = 0;

      switch (sortColumn) {
        case "market":
          comparison = a.market.localeCompare(b.market);
          break;
        case "price":
          comparison = a.trade_price - b.trade_price;
          break;
        case "change":
          comparison = a.change_rate - b.change_rate;
          break;
        case "volume":
          comparison = a.volume - b.volume;
          break;
      }

      return sortDirection === "asc" ? comparison : -comparison;
    });

    return result;
  }, [markets, filter, search, sortColumn, sortDirection, favorites]);

  // Handle sort
  const handleSort = (column: SortColumn) => {
    if (sortColumn === column) {
      setSortDirection((prev) => (prev === "asc" ? "desc" : "asc"));
    } else {
      setSortColumn(column);
      setSortDirection("desc"); // Default to desc for new column
    }
  };

  // Handle row click
  const handleRowClick = (market: string) => {
    setSelectedMarket(market);
    const marketData = markets.find((m) => m.market === market);
    if (marketData) {
      setMarketDetail({
        market: marketData.market,
        korean_name: marketData.korean_name || "",
        english_name: marketData.english_name || "",
        current_price: marketData.trade_price,
        change_rate: marketData.change_rate,
        change_amount: marketData.change_amount,
        volume_24h: marketData.volume,
        high_24h: marketData.high_price,
        low_24h: marketData.low_price,
        prev_closing_price:
          marketData.trade_price /
          (1 + marketData.change_rate),
        candles_24h: [],
      });
    }
  };

  // Calculate stats
  const topGainer = React.useMemo(() => {
    if (markets.length === 0) return null;
    return markets.reduce((max, m) =>
      m.change_rate > max.change_rate ? m : max
    );
  }, [markets]);

  const topLoser = React.useMemo(() => {
    if (markets.length === 0) return null;
    return markets.reduce((min, m) =>
      m.change_rate < min.change_rate ? m : min
    );
  }, [markets]);

  const totalVolume = React.useMemo(() => {
    return markets.reduce((sum, m) => sum + m.volume, 0);
  }, [markets]);

  return (
    <div className="py-6 px-4 max-w-7xl mx-auto">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Markets</h1>
          <p className="text-muted-foreground">
            Real-time prices for top KRW market coins
          </p>
        </div>

        <div className="flex items-center gap-2">
          {/* Connection Status */}
          <Badge
            variant={wsConnected ? "success" : "secondary"}
            className="gap-1.5"
          >
            {wsConnected ? (
              <Wifi className="h-3.5 w-3.5" />
            ) : (
              <WifiOff className="h-3.5 w-3.5" />
            )}
            {wsConnected ? "Live" : "Polling"}
          </Badge>

          {/* Refresh Button */}
          <Button
            variant="outline"
            size="sm"
            onClick={fetchMarkets}
            disabled={loading}
          >
            {loading ? (
              <Loader2 className="h-4 w-4 mr-2 animate-spin" />
            ) : (
              <RefreshCw className="h-4 w-4 mr-2" />
            )}
            Refresh
          </Button>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Total Markets
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{markets.length}</div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Top Gainer (24h)
            </CardTitle>
          </CardHeader>
          <CardContent>
            {topGainer ? (
              <>
                <div className="text-xl font-bold text-green-500">
                  {topGainer.market}
                </div>
                <div className="text-sm text-muted-foreground">
                  +{(topGainer.change_rate * 100).toFixed(2)}%
                </div>
              </>
            ) : (
              <div className="text-xl font-bold text-muted-foreground">-</div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Top Loser (24h)
            </CardTitle>
          </CardHeader>
          <CardContent>
            {topLoser ? (
              <>
                <div className="text-xl font-bold text-red-500">
                  {topLoser.market}
                </div>
                <div className="text-sm text-muted-foreground">
                  {(topLoser.change_rate * 100).toFixed(2)}%
                </div>
              </>
            ) : (
              <div className="text-xl font-bold text-muted-foreground">-</div>
            )}
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              Total Volume (24h)
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-xl font-bold">
              {new Intl.NumberFormat("ko-KR", {
                style: "currency",
                currency: "KRW",
                maximumFractionDigits: 0,
              }).format(totalVolume)}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Filters */}
      <div className="mb-4">
        <MarketFilters
          filter={filter}
          search={search}
          onFilterChange={setFilter}
          onSearchChange={setSearch}
        />
      </div>

      {/* Market Table / Card View */}
      <div className="block lg:hidden">
        <MarketCardView
          markets={sortedMarkets}
          onRowClick={handleRowClick}
          favorites={favorites}
          onToggleFavorite={toggleFavorite}
        />
      </div>
      <div className="hidden lg:block">
        <MarketTable
          markets={sortedMarkets}
          sortColumn={sortColumn}
          sortDirection={sortDirection}
          onSort={handleSort}
          onRowClick={handleRowClick}
          favorites={favorites}
          onToggleFavorite={toggleFavorite}
          loading={loading}
        />
      </div>

      {/* Market Detail Modal */}
      <MarketDetailModal
        market={marketDetail}
        open={selectedMarket !== null}
        onClose={() => {
          setSelectedMarket(null);
          setMarketDetail(null);
        }}
      />
    </div>
  );
}
