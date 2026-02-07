"use client";

/**
 * AgentStatusGrid Component
 *
 * Displays status of all 6 trading agents in a 2x3 grid:
 * - Market Monitor
 * - Signal Detector
 * - Decision Maker
 * - Execution Agent
 * - Risk Manager
 * - Notification Agent
 */

import React from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  IconRunning,
  IconIdle,
  IconError,
  IconSuccess,
  Activity,
  Zap,
  Settings,
  AlertCircle,
  Bell,
  BarChart3,
} from "@/components/ui/icons";
import { AgentState, AgentStatus } from "@/types/dashboard";

interface AgentStatusGridProps {
  agents: AgentState[];
}

// Agent icon mapping
const AGENT_ICONS: Record<string, React.ElementType> = {
  "Market Monitor": Activity,
  "Signal Detector": BarChart3,
  "Decision Maker": Settings,
  "Execution Agent": Zap,
  "Risk Manager": AlertCircle,
  "Notification Agent": Bell,
};

// Status badge variant mapping
const getStatusVariant = (status: AgentStatus): "success" | "secondary" | "destructive" => {
  switch (status) {
    case "running":
      return "success";
    case "idle":
      return "secondary";
    case "error":
      return "destructive";
    case "stopped":
      return "secondary";
    default:
      return "secondary";
  }
};

// Status icon mapping
const getStatusIcon = (status: AgentStatus) => {
  switch (status) {
    case "running":
      return IconRunning;
    case "idle":
      return IconIdle;
    case "error":
      return IconError;
    case "stopped":
      return IconSuccess;
    default:
      return Activity;
  }
};

function formatLastUpdate(timestamp: string): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffSecs = Math.floor(diffMs / 1000);
  const diffMins = Math.floor(diffSecs / 60);
  const diffHours = Math.floor(diffMins / 60);

  if (diffSecs < 60) {
    return `${diffSecs}s ago`;
  } else if (diffMins < 60) {
    return `${diffMins}m ago`;
  } else if (diffHours < 24) {
    return `${diffHours}h ago`;
  } else {
    return date.toLocaleDateString("ko-KR");
  }
}

function AgentCard({ agent }: { agent: AgentState }) {
  const IconComponent = AGENT_ICONS[agent.name] || Activity;
  const StatusIcon = getStatusIcon(agent.status);

  return (
    <Card className="overflow-hidden">
      <CardContent className="p-4">
        <div className="flex items-start justify-between gap-2">
          {/* Left side: Icon and name */}
          <div className="flex items-center gap-3 flex-1 min-w-0">
            <div className={`p-2 rounded-lg ${
              agent.status === "running"
                ? "bg-green-500/10 text-green-500"
                : agent.status === "error"
                  ? "bg-red-500/10 text-red-500"
                  : "bg-muted text-muted-foreground"
            }`}>
              <IconComponent className="h-4 w-4" />
            </div>
            <div className="flex-1 min-w-0">
              <p className="font-medium text-sm truncate">{agent.name}</p>
              <p className="text-xs text-muted-foreground truncate">
                {agent.message || "Active"}
              </p>
            </div>
          </div>

          {/* Right side: Status badge */}
          <Badge variant={getStatusVariant(agent.status)} className="shrink-0">
            <StatusIcon className="h-3 w-3 mr-1" />
            {agent.status}
          </Badge>
        </div>

        {/* Last update timestamp */}
        <div className="mt-3 flex items-center justify-between text-xs text-muted-foreground">
          <span>Last update</span>
          <span>{formatLastUpdate(agent.last_update)}</span>
        </div>
      </CardContent>
    </Card>
  );
}

export function AgentStatusGrid({ agents }: AgentStatusGridProps) {
  // Ensure we have 6 agent slots (fill with defaults if missing)
  const defaultAgents: AgentState[] = [
    { name: "Market Monitor", status: "idle", last_update: new Date().toISOString() },
    { name: "Signal Detector", status: "idle", last_update: new Date().toISOString() },
    { name: "Decision Maker", status: "idle", last_update: new Date().toISOString() },
    { name: "Execution Agent", status: "idle", last_update: new Date().toISOString() },
    { name: "Risk Manager", status: "idle", last_update: new Date().toISOString() },
    { name: "Notification Agent", status: "idle", last_update: new Date().toISOString() },
  ];

  // Merge provided agents with defaults
  const mergedAgents = defaultAgents.map((def) => {
    const provided = agents.find((a) => a.name === def.name);
    return provided || def;
  });

  return (
    <div className="col-span-full">
      <h3 className="text-sm font-medium mb-3 flex items-center gap-2">
        <Activity className="h-4 w-4" />
        Agent Status
      </h3>
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
        {mergedAgents.map((agent, index) => (
          <AgentCard key={`${agent.name}-${index}`} agent={agent} />
        ))}
      </div>
    </div>
  );
}
