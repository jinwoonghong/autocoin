"use client";

import React, { createContext, useContext, useEffect, useState, useCallback, ReactNode } from "react";

interface WebSocketContextValue {
  isConnected: boolean;
  connectionStatus: "connecting" | "connected" | "disconnected" | "error";
  sendMessage: (message: any) => void;
  lastMessage: any;
  reconnect: () => void;
}

const WebSocketContext = createContext<WebSocketContextValue | undefined>(undefined);

const WS_URL = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8080/ws";

export function WebSocketProvider({ children }: { children: ReactNode }) {
  const [isConnected, setIsConnected] = useState(false);
  const [connectionStatus, setConnectionStatus] = useState<"connecting" | "connected" | "disconnected" | "error">("disconnected");
  const [lastMessage, setLastMessage] = useState<any>(null);
  const [ws, setWs] = useState<WebSocket | null>(null);

  const connect = useCallback(() => {
    setConnectionStatus("connecting");

    try {
      const websocket = new WebSocket(WS_URL);

      websocket.onopen = () => {
        setIsConnected(true);
        setConnectionStatus("connected");

        // Subscribe to channels
        websocket.send(JSON.stringify({
          type: "subscribe",
          channels: ["prices", "trades", "position", "agents"]
        }));
      };

      websocket.onclose = () => {
        setIsConnected(false);
        setConnectionStatus("disconnected");
      };

      websocket.onerror = () => {
        setConnectionStatus("error");
      };

      websocket.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          setLastMessage(data);
        } catch (e) {
          console.error("Failed to parse WebSocket message:", e);
        }
      };

      setWs(websocket);
    } catch (error) {
      console.error("WebSocket connection error:", error);
      setConnectionStatus("error");
    }
  }, []);

  const disconnect = useCallback(() => {
    if (ws) {
      ws.close();
      setWs(null);
    }
  }, [ws]);

  const sendMessage = useCallback((message: any) => {
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(message));
    }
  }, [ws]);

  const reconnect = useCallback(() => {
    disconnect();
    connect();
  }, [disconnect, connect]);

  useEffect(() => {
    connect();

    return () => {
      disconnect();
    };
  }, [connect, disconnect]);

  // Auto-reconnect logic
  useEffect(() => {
    if (connectionStatus === "disconnected" || connectionStatus === "error") {
      const timer = setTimeout(() => {
        connect();
      }, 5000);

      return () => clearTimeout(timer);
    }
  }, [connectionStatus, connect]);

  return (
    <WebSocketContext.Provider
      value={{
        isConnected,
        connectionStatus,
        sendMessage,
        lastMessage,
        reconnect,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
}

export function useWebSocket() {
  const context = useContext(WebSocketContext);
  if (!context) {
    throw new Error("useWebSocket must be used within WebSocketProvider");
  }
  return context;
}
