/**
 * PriceCell Component
 *
 * Displays formatted price with change rate indicator
 */

import * as React from "react";
import { cn, formatKRW, formatPercent, getPnLColor } from "@/lib/utils";

export interface PriceCellProps {
  price: number;
  changeRate: number;
  format?: "short" | "full";
  showChange?: boolean;
  className?: string;
}

export function PriceCell({
  price,
  changeRate,
  format = "full",
  showChange = true,
  className,
}: PriceCellProps) {
  const priceColor = getPnLColor(changeRate);
  const priceClass = format === "short" ? "text-sm" : "text-base";

  return (
    <div className={cn("flex flex-col", className)}>
      <span className={cn("font-medium", priceClass)}>
        {format === "short" ? (
          // Short format for mobile: 96.5M
          price >= 1_000_000 ? (
            `${(price / 1_000_000).toFixed(1)}M`
          ) : price >= 1_000 ? (
            `${(price / 1_000).toFixed(1)}K`
          ) : (
            price.toLocaleString()
          )
        ) : (
          // Full format: ₩96,500,000
          formatKRW(price)
        )}
      </span>
      {showChange && (
        <span className={cn("text-xs", priceColor)}>
          {formatPercent(changeRate)}
        </span>
      )}
    </div>
  );
}

/**
 * Sparkline mini-chart component for price history
 */
export interface SparklineProps {
  data: number[];
  width?: number;
  height?: number;
  color?: string;
  className?: string;
}

export function Sparkline({
  data,
  width = 60,
  height = 20,
  color,
  className,
}: SparklineProps) {
  const canvasRef = React.useRef<HTMLCanvasElement>(null);

  React.useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || data.length < 2) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Clear canvas
    ctx.clearRect(0, 0, width, height);

    // Calculate min/max for scaling
    const min = Math.min(...data);
    const max = Math.max(...data);
    const range = max - min || 1;

    // Determine color based on trend
    const lineColor =
      color || (data[data.length - 1] > data[0] ? "#22c55e" : "#ef4444");

    // Draw line
    ctx.beginPath();
    ctx.strokeStyle = lineColor;
    ctx.lineWidth = 1.5;

    data.forEach((value, index) => {
      const x = (index / (data.length - 1)) * width;
      const y = height - ((value - min) / range) * height;

      if (index === 0) {
        ctx.moveTo(x, y);
      } else {
        ctx.lineTo(x, y);
      }
    });

    ctx.stroke();
  }, [data, width, height, color]);

  if (data.length < 2) {
    return null;
  }

  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      className={className}
      aria-hidden="true"
    />
  );
}
