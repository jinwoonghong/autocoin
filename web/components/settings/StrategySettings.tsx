"use client";

/**
 * StrategySettings Component
 *
 * Configuration form for trading strategy parameters including:
 * - Active strategy selection (momentum, multi-indicator, mean-reversion)
 * - Target coins configuration
 * - Momentum strategy parameters (surge threshold, volume multiplier)
 * - RSI indicator settings
 * - MACD indicator settings
 * - Position sizing
 */

import React, { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select, Option } from "@/components/ui/select";
import { Separator } from "@/components/ui/separator";
import { StrategyParams, StrategyType } from "@/types/settings";
import { IconSave, IconLoader } from "@/components/ui/icons";
import { cn } from "@/lib/utils";

interface StrategySettingsProps {
  settings: StrategyParams;
  onSave: (settings: StrategyParams) => Promise<void>;
  isLoading?: boolean;
}

const strategyOptions: { value: StrategyType; label: string; description: string }[] = [
  {
    value: "momentum",
    label: "Momentum",
    description: "Trade based on price surge and volume spikes",
  },
  {
    value: "multi-indicator",
    label: "Multi-Indicator",
    description: "Combine RSI, MACD, and momentum signals",
  },
  {
    value: "mean-reversion",
    label: "Mean Reversion",
    description: "Trade on RSI oversold/overbought reversals",
  },
];

interface FormErrors {
  [key: string]: string;
}

export function StrategySettings({ settings, onSave, isLoading }: StrategySettingsProps) {
  const [formData, setFormData] = useState<StrategyParams>(settings);
  const [errors, setErrors] = useState<FormErrors>({});
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    setHasChanges(JSON.stringify(formData) !== JSON.stringify(settings));
  }, [formData, settings]);

  const handleInputChange = (
    field: keyof StrategyParams,
    value: string | number | boolean
  ) => {
    setFormData((prev) => ({
      ...prev,
      [field]: value,
    }));
    // Clear error for this field
    if (errors[field]) {
      setErrors((prev) => {
        const newErrors = { ...prev };
        delete newErrors[field];
        return newErrors;
      });
    }
  };

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    // Validate target coins count
    if (formData.target_coins_count < 1 || formData.target_coins_count > 20) {
      newErrors.target_coins_count = "Must be between 1 and 20";
    }

    // Validate surge threshold
    if (formData.surge_threshold < 0.1 || formData.surge_threshold > 50) {
      newErrors.surge_threshold = "Must be between 0.1% and 50%";
    }

    // Validate volume multiplier
    if (formData.volume_multiplier < 0.5 || formData.volume_multiplier > 10) {
      newErrors.volume_multiplier = "Must be between 0.5 and 10";
    }

    // Validate RSI
    if (formData.rsi_period < 2 || formData.rsi_period > 100) {
      newErrors.rsi_period = "Must be between 2 and 100";
    }
    if (formData.rsi_oversold >= formData.rsi_overbought) {
      newErrors.rsi_oversold = "Oversold must be less than overbought";
    }

    // Validate MACD
    if (formData.macd_fast_period >= formData.macd_slow_period) {
      newErrors.macd_fast_period = "Fast period must be less than slow period";
    }

    // Validate position size
    if (formData.position_size_percent < 1 || formData.position_size_percent > 100) {
      newErrors.position_size_percent = "Must be between 1% and 100%";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    setIsSaving(true);
    try {
      await onSave(formData);
    } finally {
      setIsSaving(false);
    }
  };

  const handleReset = () => {
    setFormData(settings);
    setErrors({});
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Strategy Selection */}
      <Card>
        <CardHeader>
          <CardTitle>Strategy Selection</CardTitle>
          <CardDescription>
            Choose the trading strategy to use for signal generation
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="strategy">Active Strategy</Label>
            <Select
              id="strategy"
              value={formData.strategy}
              onChange={(e) => handleInputChange("strategy", e.target.value as StrategyType)}
              className="w-full"
            >
              {strategyOptions.map((option) => (
                <Option key={option.value} value={option.value}>
                  {option.label}
                </Option>
              ))}
            </Select>
            <p className="text-sm text-muted-foreground">
              {strategyOptions.find((s) => s.value === formData.strategy)?.description}
            </p>
          </div>
        </CardContent>
      </Card>

      {/* Target Coins Configuration */}
      <Card>
        <CardHeader>
          <CardTitle>Target Coins</CardTitle>
          <CardDescription>
            Configure which coins to monitor and trade
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="target_coins_count">Target Coins Count</Label>
              <Input
                id="target_coins_count"
                type="number"
                min="1"
                max="20"
                value={formData.target_coins_count}
                onChange={(e) =>
                  handleInputChange("target_coins_count", parseInt(e.target.value) || 0)
                }
                className={cn(errors.target_coins_count && "border-destructive")}
              />
              {errors.target_coins_count && (
                <p className="text-sm text-destructive">{errors.target_coins_count}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="min_volume_24h">Min 24h Volume (KRW)</Label>
              <Input
                id="min_volume_24h"
                type="number"
                min="0"
                step="100000000"
                value={formData.min_volume_24h}
                onChange={(e) =>
                  handleInputChange("min_volume_24h", parseInt(e.target.value) || 0)
                }
              />
              <p className="text-xs text-muted-foreground">
                Minimum daily trading volume (default: 1B KRW)
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Momentum Parameters */}
      <Card>
        <CardHeader>
          <CardTitle>Momentum Parameters</CardTitle>
          <CardDescription>
            Settings for momentum-based signal detection
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="space-y-2">
              <Label htmlFor="surge_threshold">Surge Threshold (%)</Label>
              <Input
                id="surge_threshold"
                type="number"
                min="0.1"
                max="50"
                step="0.1"
                value={formData.surge_threshold}
                onChange={(e) =>
                  handleInputChange("surge_threshold", parseFloat(e.target.value) || 0)
                }
                className={cn(errors.surge_threshold && "border-destructive")}
              />
              {errors.surge_threshold && (
                <p className="text-sm text-destructive">{errors.surge_threshold}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="surge_window_minutes">Surge Window (minutes)</Label>
              <Input
                id="surge_window_minutes"
                type="number"
                min="1"
                max="120"
                value={formData.surge_window_minutes}
                onChange={(e) =>
                  handleInputChange("surge_window_minutes", parseInt(e.target.value) || 0)
                }
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="volume_multiplier">Volume Multiplier</Label>
              <Input
                id="volume_multiplier"
                type="number"
                min="0.5"
                max="10"
                step="0.1"
                value={formData.volume_multiplier}
                onChange={(e) =>
                  handleInputChange("volume_multiplier", parseFloat(e.target.value) || 0)
                }
                className={cn(errors.volume_multiplier && "border-destructive")}
              />
              {errors.volume_multiplier && (
                <p className="text-sm text-destructive">{errors.volume_multiplier}</p>
              )}
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Technical Indicators */}
      <Card>
        <CardHeader>
          <CardTitle>Technical Indicators</CardTitle>
          <CardDescription>
            RSI and MACD indicator settings for signal confirmation
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* RSI Settings */}
          <div className="space-y-4">
            <h4 className="text-sm font-semibold">RSI Settings</h4>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="rsi_period">RSI Period</Label>
                <Input
                  id="rsi_period"
                  type="number"
                  min="2"
                  max="100"
                  value={formData.rsi_period}
                  onChange={(e) =>
                    handleInputChange("rsi_period", parseInt(e.target.value) || 0)
                  }
                  className={cn(errors.rsi_period && "border-destructive")}
                />
                {errors.rsi_period && (
                  <p className="text-sm text-destructive">{errors.rsi_period}</p>
                )}
              </div>

              <div className="space-y-2">
                <Label htmlFor="rsi_oversold">Oversold Level</Label>
                <Input
                  id="rsi_oversold"
                  type="number"
                  min="0"
                  max="50"
                  value={formData.rsi_oversold}
                  onChange={(e) =>
                    handleInputChange("rsi_oversold", parseInt(e.target.value) || 0)
                  }
                  className={cn(errors.rsi_oversold && "border-destructive")}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="rsi_overbought">Overbought Level</Label>
                <Input
                  id="rsi_overbought"
                  type="number"
                  min="50"
                  max="100"
                  value={formData.rsi_overbought}
                  onChange={(e) =>
                    handleInputChange("rsi_overbought", parseInt(e.target.value) || 0)
                }
                className={cn(errors.rsi_oversold && "border-destructive")}
                />
                {errors.rsi_oversold && (
                  <p className="text-sm text-destructive">{errors.rsi_oversold}</p>
                )}
              </div>
            </div>
          </div>

          <Separator />

          {/* MACD Settings */}
          <div className="space-y-4">
            <h4 className="text-sm font-semibold">MACD Settings</h4>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="space-y-2">
                <Label htmlFor="macd_fast_period">Fast Period</Label>
                <Input
                  id="macd_fast_period"
                  type="number"
                  min="1"
                  max="50"
                  value={formData.macd_fast_period}
                  onChange={(e) =>
                    handleInputChange("macd_fast_period", parseInt(e.target.value) || 0)
                  }
                  className={cn(errors.macd_fast_period && "border-destructive")}
                />
                {errors.macd_fast_period && (
                  <p className="text-sm text-destructive">{errors.macd_fast_period}</p>
                )}
              </div>

              <div className="space-y-2">
                <Label htmlFor="macd_slow_period">Slow Period</Label>
                <Input
                  id="macd_slow_period"
                  type="number"
                  min="1"
                  max="100"
                  value={formData.macd_slow_period}
                  onChange={(e) =>
                    handleInputChange("macd_slow_period", parseInt(e.target.value) || 0)
                  }
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="macd_signal_period">Signal Period</Label>
                <Input
                  id="macd_signal_period"
                  type="number"
                  min="1"
                  max="50"
                  value={formData.macd_signal_period}
                  onChange={(e) =>
                    handleInputChange("macd_signal_period", parseInt(e.target.value) || 0)
                  }
                />
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Position Sizing */}
      <Card>
        <CardHeader>
          <CardTitle>Position Sizing</CardTitle>
          <CardDescription>
            Configure position size and limits
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="position_size_percent">Position Size (%)</Label>
              <Input
                id="position_size_percent"
                type="number"
                min="1"
                max="100"
                value={formData.position_size_percent}
                onChange={(e) =>
                  handleInputChange("position_size_percent", parseInt(e.target.value) || 0)
                }
                className={cn(errors.position_size_percent && "border-destructive")}
              />
              {errors.position_size_percent && (
                <p className="text-sm text-destructive">{errors.position_size_percent}</p>
              )}
              <p className="text-xs text-muted-foreground">
                Percentage of available balance to use per position
              </p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="max_positions">Max Positions</Label>
              <Input
                id="max_positions"
                type="number"
                min="1"
                max="10"
                value={formData.max_positions}
                onChange={(e) =>
                  handleInputChange("max_positions", parseInt(e.target.value) || 0)
                }
              />
              <p className="text-xs text-muted-foreground">
                Maximum number of concurrent positions
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Save Actions */}
      <div className="flex items-center justify-end gap-3 pt-4 border-t">
        <Button
          type="button"
          variant="outline"
          onClick={handleReset}
          disabled={!hasChanges || isSaving}
        >
          Reset
        </Button>
        <Button
          type="submit"
          disabled={!hasChanges || isSaving || isLoading}
        >
          {isSaving ? (
            <>
              <IconLoader className="h-4 w-4 animate-spin" />
              Saving...
            </>
          ) : (
            <>
              <IconSave className="h-4 w-4" />
              Save Changes
            </>
          )}
        </Button>
      </div>
    </form>
  );
}
