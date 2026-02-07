/**
 * Agent status grid component
 * Displays the status of all trading agents
 */

"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { useAgentStatus } from "@/lib/api";
import { Badge } from "@/components/ui/badge";
import { formatTimestamp } from "@/lib/utils";
import { Activity, Clock, AlertCircle } from "lucide-react";
import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";
import { useWebSocket } from "@/app/providers/websocket-provider";
import { useEffect, useState } from "react";
import type { AgentStatus as AgentStatusType } from "@/lib/types";

export function AgentStatusGrid() {
  const { data: agents, loading, error, refetch } = useAgentStatus();
  const { lastMessage } = useWebSocket();
  const [localAgents, setLocalAgents] = useState<AgentStatusType[]>([]);

  // Update local state when WebSocket messages arrive
  useEffect(() => {
    if (lastMessage?.type === "agent_status") {
      const agentData = lastMessage.data;
      setLocalAgents((prev) => {
        const updated = [...prev];
        const index = updated.findIndex((a) => a.name === agentData.agent);
        if (index !== -1) {
          updated[index] = {
            ...updated[index],
            status: agentData.status,
            last_update: Date.now(),
            message: agentData.message,
          };
        }
        return updated;
      });
    }
  }, [lastMessage]);

  // Update local agents when API data loads
  useEffect(() => {
    if (agents) {
      setLocalAgents(agents);
    }
  }, [agents]);

  if (loading) {
    return <AgentStatusGridSkeleton />;
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle>Agent Status</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">{error}</p>
        </CardContent>
      </Card>
    );
  }

  const displayAgents = localAgents.length > 0 ? localAgents : agents || [];

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Activity className="h-5 w-5" />
          Agent Status
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-6">
          {displayAgents.map((agent) => (
            <AgentCard key={agent.name} agent={agent} />
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

function AgentCard({ agent }: { agent: AgentStatusType }) {
  const statusConfig = {
    running: {
      icon: Activity,
      color: "text-green-500",
      bgColor: "bg-green-500/10",
      borderColor: "border-green-500/20",
    },
    idle: {
      icon: Clock,
      color: "text-yellow-500",
      bgColor: "bg-yellow-500/10",
      borderColor: "border-yellow-500/20",
    },
    error: {
      icon: AlertCircle,
      color: "text-red-500",
      bgColor: "bg-red-500/10",
      borderColor: "border-red-500/20",
    },
    paused: {
      icon: Clock,
      color: "text-gray-500",
      bgColor: "bg-gray-500/10",
      borderColor: "border-gray-500/20",
    },
  };

  const config = statusConfig[agent.status];
  const Icon = config.icon;

  return (
    <div
      className={cn(
        "flex flex-col items-center rounded-lg border p-3 text-center",
        config.bgColor,
        config.borderColor
      )}
    >
      <Icon className={cn("h-6 w-6", config.color)} />
      <p className="mt-2 text-xs font-medium">{formatAgentName(agent.name)}</p>
      <Badge variant="outline" className={cn("mt-1 text-xs", config.color)}>
        {agent.status}
      </Badge>
      {agent.message && (
        <p className="mt-1 text-xs text-muted-foreground line-clamp-2">
          {agent.message}
        </p>
      )}
    </div>
  );
}

function formatAgentName(name: string): string {
  return name
    .split("_")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

function AgentStatusGridSkeleton() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Agent Status</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-6">
          {Array.from({ length: 6 }).map((_, i) => (
            <div key={i} className="flex flex-col items-center gap-2">
              <Skeleton className="h-12 w-12 rounded-full" />
              <Skeleton className="h-4 w-16" />
              <Skeleton className="h-5 w-12" />
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
