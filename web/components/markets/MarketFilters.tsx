/**
 * MarketFilters Component
 *
 * Filter controls for the markets table
 */

"use client";

import * as React from "react";
import { Search, TrendingUp, Filter } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { MarketFilter, MARKET_FILTERS } from "@/types/markets";
import { cn } from "@/lib/utils";

export interface MarketFiltersProps {
  filter: MarketFilter;
  search: string;
  onFilterChange: (filter: MarketFilter) => void;
  onSearchChange: (search: string) => void;
  className?: string;
}

export function MarketFilters({
  filter,
  search,
  onFilterChange,
  onSearchChange,
  className,
}: MarketFiltersProps) {
  const handleFilterClick = (newFilter: MarketFilter) => {
    onFilterChange(newFilter);
  };

  return (
    <div className={cn("flex items-center gap-2", className)}>
      {/* Search Input */}
      <div className="relative flex-1 max-w-sm">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
        <Input
          type="search"
          placeholder="Search markets..."
          value={search}
          onChange={(e) => onSearchChange(e.target.value)}
          className="pl-9"
        />
      </div>

      {/* Filter Dropdown */}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="sm">
            <Filter className="h-4 w-4 mr-2" />
            {MARKET_FILTERS[filter]}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-48">
          <DropdownMenuLabel>Filter Markets</DropdownMenuLabel>
          <DropdownMenuSeparator />
          {(Object.keys(MARKET_FILTERS) as MarketFilter[]).map(
            (filterOption) => (
              <DropdownMenuItem
                key={filterOption}
                onClick={() => handleFilterClick(filterOption)}
                className={cn(
                  "cursor-pointer",
                  filter === filterOption && "bg-accent"
                )}
              >
                <TrendingUp className="h-4 w-4 mr-2" />
                {MARKET_FILTERS[filterOption]}
              </DropdownMenuItem>
            )
          )}
        </DropdownMenuContent>
      </DropdownMenu>

      {/* Quick Filter Buttons */}
      <div className="hidden md:flex items-center gap-1">
        <Button
          variant={filter === "top10" ? "default" : "ghost"}
          size="sm"
          onClick={() => handleFilterClick("top10")}
        >
          Top 10
        </Button>
        <Button
          variant={filter === "top20" ? "default" : "ghost"}
          size="sm"
          onClick={() => handleFilterClick("top20")}
        >
          Top 20
        </Button>
        <Button
          variant={filter === "favorites" ? "default" : "ghost"}
          size="sm"
          onClick={() => handleFilterClick("favorites")}
        >
          Favorites
        </Button>
      </div>
    </div>
  );
}
