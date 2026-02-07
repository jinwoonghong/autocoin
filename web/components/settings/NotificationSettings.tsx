"use client";

/**
 * NotificationSettings Component
 *
 * Configuration form for notification settings including:
 * - Discord webhook URL
 * - Enable/disable notifications
 * - Notification preferences (buy signals, sell signals, errors, daily summary)
 * - Test notification button
 */

import React, { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Separator } from "@/components/ui/separator";
import { NotificationParams } from "@/types/settings";
import {
  IconSave,
  IconLoader,
  IconSend,
  IconBell,
  IconCheck,
  IconX,
} from "@/components/ui/icons";
import { cn } from "@/lib/utils";

interface NotificationSettingsProps {
  settings: NotificationParams;
  onSave: (settings: NotificationParams) => Promise<void>;
  onTestNotification?: () => Promise<void>;
  isLoading?: boolean;
}

interface FormErrors {
  [key: string]: string;
}

export function NotificationSettings({
  settings,
  onSave,
  onTestNotification,
  isLoading,
}: NotificationSettingsProps) {
  const [formData, setFormData] = useState<NotificationParams>(settings);
  const [errors, setErrors] = useState<FormErrors>({});
  const [isSaving, setIsSaving] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(null);
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    setHasChanges(JSON.stringify(formData) !== JSON.stringify(settings));
  }, [formData, settings]);

  const handleInputChange = (
    field: keyof NotificationParams,
    value: string | boolean
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

  const handlePreferenceChange = (
    field: keyof NotificationParams["preferences"],
    value: boolean
  ) => {
    setFormData((prev) => ({
      ...prev,
      preferences: {
        ...prev.preferences,
        [field]: value,
      },
    }));
  };

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    // Validate Discord webhook URL if Discord is enabled
    if (formData.discord_enabled && !formData.discord_webhook_url.trim()) {
      newErrors.discord_webhook_url = "Discord webhook URL is required when enabled";
    }

    // Validate webhook URL format
    if (
      formData.discord_webhook_url &&
      !formData.discord_webhook_url.startsWith("https://discord.com/api/webhooks/")
    ) {
      newErrors.discord_webhook_url = "Invalid Discord webhook URL";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    setIsSaving(true);
    setTestResult(null);
    try {
      await onSave(formData);
    } finally {
      setIsSaving(false);
    }
  };

  const handleTestNotification = async () => {
    setIsTesting(true);
    setTestResult(null);
    try {
      await onTestNotification?.();
      setTestResult({ success: true, message: "Test notification sent successfully!" });
    } catch (error) {
      setTestResult({
        success: false,
        message: error instanceof Error ? error.message : "Failed to send test notification",
      });
    } finally {
      setIsTesting(false);
      // Clear result after 5 seconds
      setTimeout(() => setTestResult(null), 5000);
    }
  };

  const handleReset = () => {
    setFormData(settings);
    setErrors({});
    setTestResult(null);
  };

  // Check if any notification type is enabled
  const hasEnabledNotifications =
    formData.discord_enabled || formData.telegram_enabled;

  // Check if any preference is enabled
  const hasEnabledPreferences = Object.values(formData.preferences).some(Boolean);

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* Master Toggle */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle className="flex items-center gap-2">
                <IconBell className="h-5 w-5" />
                Notifications
              </CardTitle>
              <CardDescription>
                Receive alerts for trading activities and system events
              </CardDescription>
            </div>
            <Switch
              id="notifications_enabled"
              checked={formData.enabled}
              onChange={(e) => handleInputChange("enabled", e.target.checked)}
            />
          </div>
        </CardHeader>
        {formData.enabled && (
          <CardContent className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Enable notifications to receive real-time updates about your trading
              activities.
            </p>
          </CardContent>
        )}
      </Card>

      {formData.enabled && (
        <>
          {/* Discord Integration */}
          <Card>
            <CardHeader>
              <CardTitle>Discord Integration</CardTitle>
              <CardDescription>
                Configure Discord webhook for notifications
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <div>
                  <Label htmlFor="discord_enabled" className="text-base">
                    Enable Discord Notifications
                  </Label>
                  <p className="text-sm text-muted-foreground">
                    Send notifications to a Discord channel via webhook
                  </p>
                </div>
                <Switch
                  id="discord_enabled"
                  checked={formData.discord_enabled}
                  onChange={(e) => handleInputChange("discord_enabled", e.target.checked)}
                />
              </div>

              {formData.discord_enabled && (
                <>
                  <Separator />
                  <div className="space-y-2">
                    <Label htmlFor="discord_webhook_url">Webhook URL</Label>
                    <Input
                      id="discord_webhook_url"
                      type="url"
                      placeholder="https://discord.com/api/webhooks/..."
                      value={formData.discord_webhook_url}
                      onChange={(e) =>
                        handleInputChange("discord_webhook_url", e.target.value)
                      }
                      className={cn(errors.discord_webhook_url && "border-destructive")}
                    />
                    {errors.discord_webhook_url && (
                      <p className="text-sm text-destructive">
                        {errors.discord_webhook_url}
                      </p>
                    )}
                    <p className="text-xs text-muted-foreground">
                      Create a webhook in your Discord server settings
                    </p>
                  </div>

                  {/* Test Button */}
                  <Button
                    type="button"
                    variant="outline"
                    onClick={handleTestNotification}
                    disabled={isTesting || !formData.discord_webhook_url}
                    className="gap-2"
                  >
                    {isTesting ? (
                      <>
                        <IconLoader className="h-4 w-4 animate-spin" />
                        Sending...
                      </>
                    ) : (
                      <>
                        <IconSend className="h-4 w-4" />
                        Send Test Notification
                      </>
                    )}
                  </Button>

                  {/* Test Result */}
                  {testResult && (
                    <div
                      className={cn(
                        "flex items-center gap-2 p-3 rounded-md text-sm",
                        testResult.success
                          ? "bg-green-500/10 text-green-500"
                          : "bg-destructive/10 text-destructive"
                      )}
                    >
                      {testResult.success ? (
                        <IconCheck className="h-4 w-4 flex-shrink-0" />
                      ) : (
                        <IconX className="h-4 w-4 flex-shrink-0" />
                      )}
                      <span>{testResult.message}</span>
                    </div>
                  )}
                </>
              )}
            </CardContent>
          </Card>

          {/* Notification Preferences */}
          <Card>
            <CardHeader>
              <CardTitle>Notification Preferences</CardTitle>
              <CardDescription>
                Choose which events trigger notifications
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              {!hasEnabledNotifications ? (
                <div className="p-4 rounded-md bg-muted text-center">
                  <p className="text-sm text-muted-foreground">
                    Enable a notification platform (Discord or Telegram) to configure
                    preferences
                  </p>
                </div>
              ) : (
                <div className="space-y-4">
                  {/* Buy Signals */}
                  <div className="flex items-center justify-between py-2">
                    <div>
                      <Label htmlFor="pref_buy_signals">Buy Signals</Label>
                      <p className="text-sm text-muted-foreground">
                        Notify when new buy signals are generated
                      </p>
                    </div>
                    <Switch
                      id="pref_buy_signals"
                      checked={formData.preferences.buy_signals}
                      onChange={(e) => handlePreferenceChange("buy_signals", e.target.checked)}
                    />
                  </div>

                  <Separator />

                  {/* Sell Signals */}
                  <div className="flex items-center justify-between py-2">
                    <div>
                      <Label htmlFor="pref_sell_signals">Sell Signals</Label>
                      <p className="text-sm text-muted-foreground">
                        Notify when positions are sold
                      </p>
                    </div>
                    <Switch
                      id="pref_sell_signals"
                      checked={formData.preferences.sell_signals}
                      onChange={(e) => handlePreferenceChange("sell_signals", e.target.checked)}
                    />
                  </div>

                  <Separator />

                  {/* Error Alerts */}
                  <div className="flex items-center justify-between py-2">
                    <div>
                      <Label htmlFor="pref_error_alerts">Error Alerts</Label>
                      <p className="text-sm text-muted-foreground">
                        Notify when errors occur in the trading system
                      </p>
                    </div>
                    <Switch
                      id="pref_error_alerts"
                      checked={formData.preferences.error_alerts}
                      onChange={(e) => handlePreferenceChange("error_alerts", e.target.checked)}
                    />
                  </div>

                  <Separator />

                  {/* Daily Summary */}
                  <div className="flex items-center justify-between py-2">
                    <div>
                      <Label htmlFor="pref_daily_summary">Daily Summary</Label>
                      <p className="text-sm text-muted-foreground">
                        Receive a daily summary of trading activities
                      </p>
                    </div>
                    <Switch
                      id="pref_daily_summary"
                      checked={formData.preferences.daily_summary}
                      onChange={(e) => handlePreferenceChange("daily_summary", e.target.checked)}
                    />
                  </div>

                  <Separator />

                  {/* Weekly Report */}
                  <div className="flex items-center justify-between py-2">
                    <div>
                      <Label htmlFor="pref_weekly_report">Weekly Report</Label>
                      <p className="text-sm text-muted-foreground">
                        Receive a detailed weekly performance report
                      </p>
                    </div>
                    <Switch
                      id="pref_weekly_report"
                      checked={formData.preferences.weekly_report}
                      onChange={(e) => handlePreferenceChange("weekly_report", e.target.checked)}
                    />
                  </div>

                  {/* Warning if no preferences enabled */}
                  {!hasEnabledPreferences && (
                    <div className="p-3 rounded-md bg-yellow-500/10 text-yellow-500 text-sm">
                      No notification preferences are enabled. Enable at least one
                      preference to receive notifications.
                    </div>
                  )}
                </div>
              )}
            </CardContent>
          </Card>
        </>
      )}

      {/* Save Actions */}
      {formData.enabled && (
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
      )}
    </form>
  );
}
