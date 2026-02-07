"use client";

/**
 * RiskSettings Component
 *
 * Configuration form for risk management parameters including:
 * - Stop loss rate
 * - Take profit rate
 * - Maximum position size (KRW)
 * - Total exposure limits
 * - Trailing stop configuration
 * - Daily loss limits
 */

import React, { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Separator } from "@/components/ui/separator";
import { RiskParams } from "@/types/settings";
import { IconSave, IconLoader, IconShield } from "@/components/ui/icons";
import { cn } from "@/lib/utils";

interface RiskSettingsProps {
  settings: RiskParams;
  onSave: (settings: RiskParams) => Promise<void>;
  isLoading?: boolean;
}

interface FormErrors {
  [key: string]: string;
}

export function RiskSettings({ settings, onSave, isLoading }: RiskSettingsProps) {
  const [formData, setFormData] = useState<RiskParams>(settings);
  const [errors, setErrors] = useState<FormErrors>({});
  const [isSaving, setIsSaving] = useState(false);
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    setHasChanges(JSON.stringify(formData) !== JSON.stringify(settings));
  }, [formData, settings]);

  const handleInputChange = (
    field: keyof RiskParams,
    value: string | number | boolean
  ) => {
    setFormData((prev) => ({
      ...prev,
      [field]: value,
    }));
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

    // Validate stop loss
    if (formData.stop_loss_rate < 0.5 || formData.stop_loss_rate > 50) {
      newErrors.stop_loss_rate = "Must be between 0.5% and 50%";
    }

    // Validate take profit
    if (formData.take_profit_rate < 0.5 || formData.take_profit_rate > 100) {
      newErrors.take_profit_rate = "Must be between 0.5% and 100%";
    }

    // Validate max position size
    if (formData.max_position_size_krw < 10000) {
      newErrors.max_position_size_krw = "Minimum 10,000 KRW";
    }

    // Validate total exposure
    if (formData.max_total_exposure_percent < 10 || formData.max_total_exposure_percent > 100) {
      newErrors.max_total_exposure_percent = "Must be between 10% and 100%";
    }

    // Validate trailing stop if enabled
    if (formData.trailing_stop_enabled) {
      if (formData.trailing_stop_rate < 0.1 || formData.trailing_stop_rate > 10) {
        newErrors.trailing_stop_rate = "Must be between 0.1% and 10%";
      }
      if (formData.trailing_stop_activation_percent < 0.5 || formData.trailing_stop_activation_percent > 50) {
        newErrors.trailing_stop_activation_percent = "Must be between 0.5% and 50%";
      }
    }

    // Validate daily loss limit if enabled
    if (formData.daily_loss_limit_enabled && formData.daily_loss_limit_krw < 10000) {
      newErrors.daily_loss_limit_krw = "Minimum 10,000 KRW";
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

  // Calculate risk-reward ratio
  const riskRewardRatio = formData.take_profit_rate / formData.stop_loss_rate;

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Risk Overview */}
      <Card className="bg-primary/5 border-primary/20">
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <IconShield className="h-5 w-5" />
            Risk Overview
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <p className="text-sm text-muted-foreground">Stop Loss</p>
              <p className="text-2xl font-bold text-destructive">{formData.stop_loss_rate}%</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Take Profit</p>
              <p className="text-2xl font-bold text-green-500">{formData.take_profit_rate}%</p>
            </div>
            <div>
              <p className="text-sm text-muted-foreground">Risk/Reward Ratio</p>
              <p
                className={cn(
                  "text-2xl font-bold",
                  riskRewardRatio >= 2 ? "text-green-500" : riskRewardRatio >= 1.5 ? "text-yellow-500" : "text-destructive"
                )}
              >
                1:{riskRewardRatio.toFixed(1)}
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Stop Loss & Take Profit */}
      <Card>
        <CardHeader>
          <CardTitle>Exit Points</CardTitle>
          <CardDescription>
            Configure automatic exit levels for positions
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="stop_loss_rate">Stop Loss Rate (%)</Label>
              <Input
                id="stop_loss_rate"
                type="number"
                min="0.5"
                max="50"
                step="0.1"
                value={formData.stop_loss_rate}
                onChange={(e) =>
                  handleInputChange("stop_loss_rate", parseFloat(e.target.value) || 0)
                }
                className={cn(errors.stop_loss_rate && "border-destructive")}
              />
              {errors.stop_loss_rate && (
                <p className="text-sm text-destructive">{errors.stop_loss_rate}</p>
              )}
              <p className="text-xs text-muted-foreground">
                Maximum loss percentage before position is closed
              </p>
            </div>

            <div className="space-y-2">
              <Label htmlFor="take_profit_rate">Take Profit Rate (%)</Label>
              <Input
                id="take_profit_rate"
                type="number"
                min="0.5"
                max="100"
                step="0.1"
                value={formData.take_profit_rate}
                onChange={(e) =>
                  handleInputChange("take_profit_rate", parseFloat(e.target.value) || 0)
                }
                className={cn(errors.take_profit_rate && "border-destructive")}
              />
              {errors.take_profit_rate && (
                <p className="text-sm text-destructive">{errors.take_profit_rate}</p>
              )}
              <p className="text-xs text-muted-foreground">
                Target profit percentage to close position
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Position Limits */}
      <Card>
        <CardHeader>
          <CardTitle>Position Limits</CardTitle>
          <CardDescription>
            Set maximum exposure limits
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="max_position_size_krw">Max Position Size (KRW)</Label>
              <Input
                id="max_position_size_krw"
                type="number"
                min="10000"
                step="10000"
                value={formData.max_position_size_krw}
                onChange={(e) =>
                  handleInputChange("max_position_size_krw", parseInt(e.target.value) || 0)
                }
                className={cn(errors.max_position_size_krw && "border-destructive")}
              />
              {errors.max_position_size_krw && (
                <p className="text-sm text-destructive">{errors.max_position_size_krw}</p>
              )}
            </div>

            <div className="space-y-2">
              <Label htmlFor="max_total_exposure_percent">Max Total Exposure (%)</Label>
              <Input
                id="max_total_exposure_percent"
                type="number"
                min="10"
                max="100"
                value={formData.max_total_exposure_percent}
                onChange={(e) =>
                  handleInputChange("max_total_exposure_percent", parseInt(e.target.value) || 0)
                }
                className={cn(errors.max_total_exposure_percent && "border-destructive")}
              />
              {errors.max_total_exposure_percent && (
                <p className="text-sm text-destructive">{errors.max_total_exposure_percent}</p>
              )}
              <p className="text-xs text-muted-foreground">
                Maximum percentage of total balance to risk
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Trailing Stop */}
      <Card>
        <CardHeader>
          <CardTitle>Trailing Stop</CardTitle>
          <CardDescription>
            Automatically adjust stop loss as price moves favorably
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <Label htmlFor="trailing_stop_enabled" className="text-base">
                Enable Trailing Stop
              </Label>
              <p className="text-sm text-muted-foreground">
                Lock in profits by moving stop loss with price
              </p>
            </div>
            <Switch
              id="trailing_stop_enabled"
              checked={formData.trailing_stop_enabled}
              onChange={(e) =>
                handleInputChange("trailing_stop_enabled", e.target.checked)
              }
            />
          </div>

          {formData.trailing_stop_enabled && (
            <>
              <Separator />
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="trailing_stop_rate">Trailing Distance (%)</Label>
                  <Input
                    id="trailing_stop_rate"
                    type="number"
                    min="0.1"
                    max="10"
                    step="0.1"
                    value={formData.trailing_stop_rate}
                    onChange={(e) =>
                      handleInputChange("trailing_stop_rate", parseFloat(e.target.value) || 0)
                    }
                    className={cn(errors.trailing_stop_rate && "border-destructive")}
                  />
                  {errors.trailing_stop_rate && (
                    <p className="text-sm text-destructive">{errors.trailing_stop_rate}</p>
                  )}
                  <p className="text-xs text-muted-foreground">
                    Distance behind current price for stop loss
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="trailing_stop_activation_percent">
                    Activation (%)
                  </Label>
                  <Input
                    id="trailing_stop_activation_percent"
                    type="number"
                    min="0.5"
                    max="50"
                    step="0.1"
                    value={formData.trailing_stop_activation_percent}
                    onChange={(e) =>
                      handleInputChange(
                        "trailing_stop_activation_percent",
                        parseFloat(e.target.value) || 0
                      )
                    }
                    className={cn(errors.trailing_stop_activation_percent && "border-destructive")}
                  />
                  {errors.trailing_stop_activation_percent && (
                    <p className="text-sm text-destructive">
                      {errors.trailing_stop_activation_percent}
                    </p>
                  )}
                  <p className="text-xs text-muted-foreground">
                    Profit percentage before trailing starts
                  </p>
                </div>
              </div>
            </>
          )}
        </CardContent>
      </Card>

      {/* Daily Loss Limit */}
      <Card>
        <CardHeader>
          <CardTitle>Daily Loss Limit</CardTitle>
          <CardDescription>
            Stop trading when daily losses exceed threshold
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center justify-between">
            <div>
              <Label htmlFor="daily_loss_limit_enabled" className="text-base">
                Enable Daily Loss Limit
              </Label>
              <p className="text-sm text-muted-foreground">
                Prevent catastrophic losses in a single day
              </p>
            </div>
            <Switch
              id="daily_loss_limit_enabled"
              checked={formData.daily_loss_limit_enabled}
              onChange={(e) =>
                handleInputChange("daily_loss_limit_enabled", e.target.checked)
              }
            />
          </div>

          {formData.daily_loss_limit_enabled && (
            <>
              <Separator />
              <div className="space-y-2">
                <Label htmlFor="daily_loss_limit_krw">Daily Loss Limit (KRW)</Label>
                <Input
                  id="daily_loss_limit_krw"
                  type="number"
                  min="10000"
                  step="10000"
                  value={formData.daily_loss_limit_krw}
                  onChange={(e) =>
                    handleInputChange("daily_loss_limit_krw", parseInt(e.target.value) || 0)
                  }
                  className={cn(errors.daily_loss_limit_krw && "border-destructive")}
                />
                {errors.daily_loss_limit_krw && (
                  <p className="text-sm text-destructive">{errors.daily_loss_limit_krw}</p>
                )}
                <p className="text-xs text-muted-foreground">
                  Trading will stop when daily losses reach this amount
                </p>
              </div>
            </>
          )}
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
