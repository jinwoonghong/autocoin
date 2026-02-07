/**
 * WebSocket connection status indicator component
 */

"use client";

import { useWebSocket } from "@/app/providers/websocket-provider";
import { Badge } from "@/components/ui/badge";
import { Wifi, WifiOff, Loader2 } from "lucide-react";

export function ConnectionStatus() {
  const { connectionStatus } = useWebSocket();

  const statusConfig = {
    connected: {
      label: "Connected",
      variant: "success" as const,
      icon: Wifi,
    },
    connecting: {
      label: "Connecting...",
      variant: "warning" as const,
      icon: Loader2,
    },
    disconnected: {
      label: "Disconnected",
      variant: "destructive" as const,
      icon: WifiOff,
    },
    error: {
      label: "Connection Error",
      variant: "destructive" as const,
      icon: WifiOff,
    },
  };

  const config = statusConfig[connectionStatus];
  const Icon = config.icon;

  return (
    <Badge variant={config.variant} className="gap-1.5">
      <Icon className="h-3 w-3" />
      {config.label}
    </Badge>
  );
}
