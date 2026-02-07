/**
 * WebSocket Client Hook
 *
 * Custom hook for managing WebSocket connections with auto-reconnect
 */

import { useEffect, useRef, useState, useCallback } from "react";

type ConnectionState = "connecting" | "connected" | "disconnected" | "error";

export interface UseWebSocketClientOptions {
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
  onMessage?: (data: string) => void;
  onConnect?: () => void;
  onDisconnect?: () => void;
  onError?: (error: Event) => void;
}

export function useWebSocketClient(
  url: string,
  options: UseWebSocketClientOptions = {}
) {
  const {
    reconnectInterval = 3000,
    maxReconnectAttempts = 5,
    onMessage,
    onConnect,
    onDisconnect,
    onError,
  } = options;

  const [connectionState, setConnectionState] =
    useState<ConnectionState>("disconnected");
  const [lastMessage, setLastMessage] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimerRef = useRef<NodeJS.Timeout | null>(null);
  const reconnectAttemptsRef = useRef(0);

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      return;
    }

    setConnectionState("connecting");

    try {
      const ws = new WebSocket(url);
      wsRef.current = ws;

      ws.onopen = () => {
        setConnectionState("connected");
        reconnectAttemptsRef.current = 0;
        onConnect?.();

        // Subscribe to price updates
        ws.send(
          JSON.stringify({
            type: "subscribe",
            channels: ["prices", "trades", "position", "agents"],
          })
        );
      };

      ws.onmessage = (event) => {
        setLastMessage(event.data);
        onMessage?.(event.data);
      };

      ws.onclose = () => {
        setConnectionState("disconnected");
        wsRef.current = null;
        onDisconnect?.();

        // Attempt reconnect
        if (reconnectAttemptsRef.current < maxReconnectAttempts) {
          reconnectTimerRef.current = setTimeout(() => {
            reconnectAttemptsRef.current++;
            connect();
          }, reconnectInterval);
        }
      };

      ws.onerror = (error) => {
        setConnectionState("error");
        onError?.(error);
      };
    } catch (error) {
      setConnectionState("error");
      console.error("WebSocket connection error:", error);
    }
  }, [
    url,
    reconnectInterval,
    maxReconnectAttempts,
    onConnect,
    onDisconnect,
    onError,
    onMessage,
  ]);

  const disconnect = useCallback(() => {
    if (reconnectTimerRef.current) {
      clearTimeout(reconnectTimerRef.current);
      reconnectTimerRef.current = null;
    }

    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }

    setConnectionState("disconnected");
  }, []);

  const send = useCallback((data: string) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(data);
      return true;
    }
    return false;
  }, []);

  // Connect on mount
  useEffect(() => {
    connect();

    return () => {
      disconnect();
    };
  }, [connect, disconnect]);

  return {
    connectionState,
    lastMessage,
    send,
    connect,
    disconnect,
    isConnected: connectionState === "connected",
  };
}
