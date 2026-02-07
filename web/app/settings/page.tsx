"use client";

/**
 * Settings Page
 *
 * Main settings page for configuring the AutoCoin trading system.
 * Sections: Strategy, Risk Management, System, Notifications, About
 */

import React, { useState, useEffect } from "react";
import {
  StrategySettings,
  RiskSettings,
  SystemControls,
  NotificationSettings,
  AboutSettings,
} from "@/components/settings";
import type { SettingsSection, SettingsConfig, SystemStatus } from "@/types/settings";
import {
  defaultStrategyParams,
  defaultRiskParams,
  defaultNotificationParams,
} from "@/types/settings";
import { Loader2, AlertTriangle } from "lucide-react";

// Icon components for compatibility
const IconLoader = Loader2;
const IconWarning = AlertTriangle;

// API base URL - adjust for your environment
const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:3000";

export default function SettingsPage() {
  const [activeSection, setActiveSection] = useState<SettingsSection>("strategy");
  const [config, setConfig] = useState<SettingsConfig>({
    strategy: defaultStrategyParams,
    risk: defaultRiskParams,
    notifications: defaultNotificationParams,
    system: {
      running: false,
      uptime_seconds: 0,
      version: "0.1.0",
      commit_hash: "unknown",
      build_time: new Date().toISOString(),
    },
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);
  const [saveSuccess, setSaveSuccess] = useState(false);
  const [toast, setToast] = useState<{ message: string; type: "success" | "error" } | null>(null);

  const showToast = (message: string, type: "success" | "error" = "success") => {
    setToast({ message, type });
    setTimeout(() => setToast(null), 3000);
  };

  // Fetch settings on mount
  useEffect(() => {
    fetchSettings();
    // Poll system status every 10 seconds
    const interval = setInterval(fetchSystemStatus, 10000);
    return () => clearInterval(interval);
  }, []);

  const fetchSettings = async () => {
    try {
      setIsLoading(true);
      setError(null);

      const response = await fetch(`${API_BASE}/api/settings`);
      if (!response.ok) {
        throw new Error(`Failed to fetch settings: ${response.statusText}`);
      }

      const data = await response.json();
      setConfig(data);
    } catch (err) {
      console.error("Error fetching settings:", err);
      setError(err instanceof Error ? err.message : "Failed to load settings");
      // Keep defaults on error
    } finally {
      setIsLoading(false);
    }
  };

  const fetchSystemStatus = async () => {
    try {
      const response = await fetch(`${API_BASE}/api/system/status`);
      if (!response.ok) return;

      const status: SystemStatus = await response.json();
      setConfig((prev) => ({
        ...prev,
        system: status,
      }));
    } catch (err) {
      console.error("Error fetching system status:", err);
    }
  };

  const saveStrategySettings = async (settings: typeof config.strategy) => {
    setIsSaving(true);
    setSaveSuccess(false);
    try {
      const response = await fetch(`${API_BASE}/api/settings/strategy`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(settings),
      });

      if (!response.ok) {
        throw new Error(`Failed to save: ${response.statusText}`);
      }

      setConfig((prev) => ({ ...prev, strategy: settings }));
      setSaveSuccess(true);
      showToast("Strategy settings saved successfully!", "success");
    } catch {
      showToast("Failed to save strategy settings", "error");
    } finally {
      setIsSaving(false);
    }
  };

  const saveRiskSettings = async (settings: typeof config.risk) => {
    setIsSaving(true);
    setSaveSuccess(false);
    try {
      const response = await fetch(`${API_BASE}/api/settings/risk`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(settings),
      });

      if (!response.ok) {
        throw new Error(`Failed to save: ${response.statusText}`);
      }

      setConfig((prev) => ({ ...prev, risk: settings }));
      setSaveSuccess(true);
      showToast("Risk settings saved successfully!", "success");
    } catch {
      showToast("Failed to save risk settings", "error");
    } finally {
      setIsSaving(false);
    }
  };

  const saveNotificationSettings = async (settings: typeof config.notifications) => {
    setIsSaving(true);
    setSaveSuccess(false);
    try {
      const response = await fetch(`${API_BASE}/api/settings/notifications`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(settings),
      });

      if (!response.ok) {
        throw new Error(`Failed to save: ${response.statusText}`);
      }

      setConfig((prev) => ({ ...prev, notifications: settings }));
      setSaveSuccess(true);
      showToast("Notification settings saved successfully!", "success");
    } catch {
      showToast("Failed to save notification settings", "error");
    } finally {
      setIsSaving(false);
    }
  };

  const handleStart = async () => {
    try {
      const response = await fetch(`${API_BASE}/api/system/start`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to start system");
      await fetchSystemStatus();
    } catch (err) {
      console.error("Error starting system:", err);
    }
  };

  const handleStop = async () => {
    try {
      const response = await fetch(`${API_BASE}/api/system/stop`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to stop system");
      await fetchSystemStatus();
    } catch (err) {
      console.error("Error stopping system:", err);
    }
  };

  const handleRestart = async () => {
    try {
      const response = await fetch(`${API_BASE}/api/system/restart`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to restart system");
      await fetchSystemStatus();
    } catch (err) {
      console.error("Error restarting system:", err);
    }
  };

  const handleTestNotification = async () => {
    try {
      const response = await fetch(`${API_BASE}/api/notifications/test`, {
        method: "POST",
      });
      if (!response.ok) throw new Error("Failed to send test notification");
    } catch (err) {
      throw err;
    }
  };

  const renderSection = () => {
    if (isLoading) {
      return (
        <div className="flex items-center justify-center py-20">
          <div className="text-center">
            <IconLoader className="h-8 w-8 animate-spin mx-auto mb-4 text-muted-foreground" />
            <p className="text-muted-foreground">Loading settings...</p>
          </div>
        </div>
      );
    }

    if (error) {
      return (
        <div className="flex items-center justify-center py-20">
          <div className="text-center max-w-md">
            <div className="mx-auto mb-4 text-destructive">
              <IconWarning className="h-12 w-12" />
            </div>
            <h3 className="text-lg font-semibold mb-2">Failed to Load Settings</h3>
            <p className="text-muted-foreground mb-4">{error}</p>
            <button
              onClick={fetchSettings}
              className="text-primary hover:underline"
            >
              Try again
            </button>
          </div>
        </div>
      );
    }

    switch (activeSection) {
      case "strategy":
        return (
          <div>
            <h2 className="text-2xl font-bold mb-6">Strategy Settings</h2>
            <StrategySettings
              settings={config.strategy}
              onSave={saveStrategySettings}
              isLoading={isSaving}
            />
          </div>
        );

      case "risk":
        return (
          <div>
            <h2 className="text-2xl font-bold mb-6">Risk Management</h2>
            <RiskSettings
              settings={config.risk}
              onSave={saveRiskSettings}
              isLoading={isSaving}
            />
          </div>
        );

      case "system":
        return (
          <div>
            <h2 className="text-2xl font-bold mb-6">System Controls</h2>
            <SystemControls
              status={config.system}
              onStart={handleStart}
              onStop={handleStop}
              onRestart={handleRestart}
            />
          </div>
        );

      case "notifications":
        return (
          <div>
            <h2 className="text-2xl font-bold mb-6">Notification Settings</h2>
            <NotificationSettings
              settings={config.notifications}
              onSave={saveNotificationSettings}
              onTestNotification={handleTestNotification}
              isLoading={isSaving}
            />
          </div>
        );

      case "about":
        return (
          <div>
            <h2 className="text-2xl font-bold mb-6">About AutoCoin</h2>
            <AboutSettings
              version={config.system.version}
              commitHash={config.system.commit_hash}
              buildTime={config.system.build_time}
            />
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-background">
      {/* Success Banner */}
      {saveSuccess && (
        <div className="fixed top-0 left-0 right-0 z-50 bg-green-500 text-white py-2 px-4 text-center text-sm font-medium">
          Settings saved successfully!
        </div>
      )}

      {/* Toast Notification */}
      {toast && (
        <div className={`fixed top-4 right-4 z-50 rounded-lg px-4 py-3 shadow-lg ${
          toast.type === "success" ? "bg-green-500 text-white" : "bg-red-500 text-white"
        }`}>
          {toast.message}
        </div>
      )}

      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground mt-2">
            Configure your AutoCoin trading system
          </p>
        </div>

        <div className="flex flex-col lg:flex-row gap-8">
          {/* Sidebar Navigation */}
          <nav className="lg:w-64 flex-shrink-0">
            <div className="sticky top-8 space-y-1">
              <SettingsNavItem
                id="strategy"
                label="Strategy"
                description="Trading strategy parameters"
                active={activeSection}
                onClick={setActiveSection}
              />
              <SettingsNavItem
                id="risk"
                label="Risk Management"
                description="Stop loss, take profit, limits"
                active={activeSection}
                onClick={setActiveSection}
              />
              <SettingsNavItem
                id="system"
                label="System"
                description="Control trading system"
                active={activeSection}
                onClick={setActiveSection}
              />
              <SettingsNavItem
                id="notifications"
                label="Notifications"
                description="Discord and alerts"
                active={activeSection}
                onClick={setActiveSection}
              />
              <SettingsNavItem
                id="about"
                label="About"
                description="System information"
                active={activeSection}
                onClick={setActiveSection}
              />
            </div>
          </nav>

          {/* Content Area */}
          <main className="flex-1 min-w-0">{renderSection()}</main>
        </div>
      </div>
    </div>
  );
}

interface SettingsNavItemProps {
  id: SettingsSection;
  label: string;
  description: string;
  active: SettingsSection;
  onClick: (section: SettingsSection) => void;
}

function SettingsNavItem({
  id,
  label,
  description,
  active,
  onClick,
}: SettingsNavItemProps) {
  return (
    <button
      onClick={() => onClick(id)}
      className={`w-full text-left px-4 py-3 rounded-lg transition-colors ${
        active === id
          ? "bg-primary text-primary-foreground font-medium"
          : "hover:bg-accent hover:text-accent-foreground text-muted-foreground"
      }`}
    >
      <div className="text-sm">{label}</div>
      <div className={`text-xs mt-0.5 ${active === id ? "opacity-80" : "opacity-60"}`}>
        {description}
      </div>
    </button>
  );
}
