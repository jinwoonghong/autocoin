"use client";

/**
 * SystemControls Component
 *
 * System control panel for trading bot including:
 * - Start/Stop/Restart buttons
 * - System status indicator
 * - Uptime display
 * - Last trade time
 * - Version information
 */

import React, { useState, useEffect } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { SystemStatus } from "@/types/settings";
import {
  IconPower,
  IconPowerOff,
  IconRestart,
  IconServer,
  IconClock,
  IconCheck,
  IconX,
  IconLoader,
} from "@/components/ui/icons";
import { formatUptime, formatTimestamp } from "@/lib/utils";
import { cn } from "@/lib/utils";

interface SystemControlsProps {
  status: SystemStatus;
  onStart: () => Promise<void>;
  onStop: () => Promise<void>;
  onRestart: () => Promise<void>;
  isLoading?: boolean;
}

type ControlAction = "start" | "stop" | "restart";

export function SystemControls({
  status,
  onStart,
  onStop,
  onRestart,
  isLoading,
}: SystemControlsProps) {
  const [isActionLoading, setIsActionLoading] = useState<ControlAction | null>(null);
  const [uptime, setUptime] = useState(status.uptime_seconds);

  // Update uptime every second
  useEffect(() => {
    if (!status.running) {
      setUptime(status.uptime_seconds);
      return;
    }

    const interval = setInterval(() => {
      setUptime((prev) => prev + 1);
    }, 1000);

    return () => clearInterval(interval);
  }, [status.running, status.uptime_seconds]);

  const handleAction = async (action: ControlAction) => {
    setIsActionLoading(action);
    try {
      switch (action) {
        case "start":
          await onStart();
          break;
        case "stop":
          await onStop();
          break;
        case "restart":
          await onRestart();
          break;
      }
    } finally {
      setIsActionLoading(null);
    }
  };

  const getStatusBadge = () => {
    if (isLoading || isActionLoading) {
      return (
        <Badge variant="secondary" className="gap-1.5">
          <IconLoader className="h-3 w-3 animate-spin" />
          Loading...
        </Badge>
      );
    }

    if (status.running) {
      return (
        <Badge variant="success" className="gap-1.5">
          <IconCheck className="h-3 w-3" />
          Running
        </Badge>
      );
    }

    return (
      <Badge variant="secondary" className="gap-1.5">
        <IconX className="h-3 w-3" />
        Stopped
      </Badge>
    );
  };

  const getStatusColor = () => {
    if (isLoading || isActionLoading) return "text-yellow-500";
    if (status.running) return "text-green-500";
    return "text-muted-foreground";
  };

  const getStatusText = () => {
    if (isLoading || isActionLoading) return "Loading system status...";
    if (status.running) return "System is running and actively trading";
    return "System is stopped. No trades will be executed.";
  };

  return (
    <div className="space-y-6">
      {/* System Status Card */}
      <Card className="bg-gradient-to-br from-background to-muted/20">
        <CardHeader>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div
                className={cn(
                  "p-3 rounded-full transition-colors",
                  status.running ? "bg-green-500/10" : "bg-muted"
                )}
              >
                <IconServer className={cn("h-6 w-6", getStatusColor())} />
              </div>
              <div>
                <CardTitle className="text-2xl">Trading System</CardTitle>
                <p className="text-sm text-muted-foreground mt-1">{getStatusText()}</p>
              </div>
            </div>
            {getStatusBadge()}
          </div>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Control Buttons */}
          <div className="flex flex-wrap gap-3">
            {!status.running ? (
              <Button
                size="lg"
                variant="success"
                onClick={() => handleAction("start")}
                disabled={isActionLoading !== null || isLoading}
                className="gap-2"
              >
                {isActionLoading === "start" ? (
                  <>
                    <IconLoader className="h-4 w-4 animate-spin" />
                    Starting...
                  </>
                ) : (
                  <>
                    <IconPower className="h-4 w-4" />
                    Start Trading
                  </>
                )}
              </Button>
            ) : (
              <>
                <Button
                  size="lg"
                  variant="destructive"
                  onClick={() => handleAction("stop")}
                  disabled={isActionLoading !== null || isLoading}
                  className="gap-2"
                >
                  {isActionLoading === "stop" ? (
                    <>
                      <IconLoader className="h-4 w-4 animate-spin" />
                    </>
                  ) : (
                    <>
                      <IconPowerOff className="h-4 w-4" />
                    </>
                  )}
                  {isActionLoading === "stop" ? "Stopping..." : "Stop Trading"}
                </Button>
                <Button
                  size="lg"
                  variant="outline"
                  onClick={() => handleAction("restart")}
                  disabled={isActionLoading !== null || isLoading}
                  className="gap-2"
                >
                  {isActionLoading === "restart" ? (
                    <>
                      <IconLoader className="h-4 w-4 animate-spin" />
                    </>
                  ) : (
                    <>
                      <IconRestart className="h-4 w-4" />
                    </>
                  )}
                  {isActionLoading === "restart" ? "Restarting..." : "Restart"}
                </Button>
              </>
            )}
          </div>

          <Separator />

          {/* System Information */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* Uptime */}
            <div className="space-y-1">
              <p className="text-sm text-muted-foreground flex items-center gap-1.5">
                <IconClock className="h-3.5 w-3.5" />
                Uptime
              </p>
              <p className="text-xl font-semibold">
                {status.running ? formatUptime(uptime) : "--"}
              </p>
            </div>

            {/* Last Trade */}
            <div className="space-y-1">
              <p className="text-sm text-muted-foreground">Last Trade</p>
              <p className="text-xl font-semibold">
                {status.last_trade_time
                  ? formatTimestamp(status.last_trade_time)
                  : "No trades yet"}
              </p>
            </div>
          </div>

          <Separator />

          {/* Version Information */}
          <div className="space-y-3">
            <h4 className="text-sm font-semibold">Version Information</h4>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
              <div>
                <p className="text-muted-foreground">Version</p>
                <p className="font-medium">{status.version}</p>
              </div>
              <div>
                <p className="text-muted-foreground">Commit</p>
                <p className="font-medium font-mono text-xs">
                  {status.commit_hash.slice(0, 8)}
                </p>
              </div>
              <div>
                <p className="text-muted-foreground">Built</p>
                <p className="font-medium">{status.build_time}</p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Warning Card */}
      {status.running && (
        <Card className="border-yellow-500/20 bg-yellow-500/5">
          <CardContent className="p-4">
            <div className="flex gap-3">
              <div className="text-yellow-500 mt-0.5">
                <svg
                  className="h-5 w-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                  />
                </svg>
              </div>
              <div>
                <p className="font-semibold text-sm">Trading is Active</p>
                <p className="text-sm text-muted-foreground mt-1">
                  The system is actively trading with real funds. Make sure your risk
                  settings are appropriate before making changes.
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
