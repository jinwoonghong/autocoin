"use client";

/**
 * AboutSettings Component
 *
 * System information and about section including:
 * - Application version and build info
 * - Documentation links
 * - License information
 * - System requirements
 */

import React from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  IconInfo,
  IconServer,
  IconShield,
  IconBook,
  IconGitHub,
  IconZap,
} from "@/components/ui/icons";

interface AboutSettingsProps {
  version?: string;
  commitHash?: string;
  buildTime?: string;
}

export function AboutSettings({
  version = "0.1.0",
  commitHash = "unknown",
  buildTime = new Date().toISOString(),
}: AboutSettingsProps) {
  const docsLinks = [
    { href: "/docs", label: "Documentation", icon: IconBook },
    { href: "https://github.com", label: "GitHub Repository", icon: IconGitHub },
  ];

  const features = [
    { icon: IconZap, title: "Real-time Trading", description: "Automated trading on Upbit exchange" },
    { icon: IconServer, title: "Multi-Strategy", description: "Momentum, RSI, MACD strategies" },
    { icon: IconShield, title: "Risk Management", description: "Stop loss, take profit, trailing stops" },
  ];

  return (
    <div className="space-y-6">
      {/* App Info Card */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <IconInfo className="h-5 w-5" />
            AutoCoin Trading System
          </CardTitle>
          <CardDescription>
            Automated cryptocurrency trading bot for Upbit exchange
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Version Badge */}
          <div className="flex items-center gap-3">
            <Badge variant="secondary" className="text-base px-3 py-1">
              v{version}
            </Badge>
            <span className="text-sm text-muted-foreground">
              Build: {commitHash.slice(0, 8)}
            </span>
          </div>

          <Separator />

          {/* Build Information */}
          <div className="space-y-2">
            <h4 className="text-sm font-semibold">Build Information</h4>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
              <div>
                <p className="text-muted-foreground">Version</p>
                <p className="font-mono">{version}</p>
              </div>
              <div>
                <p className="text-muted-foreground">Commit Hash</p>
                <p className="font-mono">{commitHash}</p>
              </div>
              <div>
                <p className="text-muted-foreground">Build Date</p>
                <p className="font-mono">
                  {new Date(buildTime).toLocaleDateString()}
                </p>
              </div>
              <div>
                <p className="text-muted-foreground">Build Time</p>
                <p className="font-mono">
                  {new Date(buildTime).toLocaleTimeString()}
                </p>
              </div>
            </div>
          </div>

          <Separator />

          {/* Key Features */}
          <div className="space-y-3">
            <h4 className="text-sm font-semibold">Key Features</h4>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {features.map((feature) => {
                const Icon = feature.icon;
                return (
                  <div key={feature.title} className="flex items-start gap-3">
                    <div className="p-2 rounded-md bg-primary/10">
                      <Icon className="h-4 w-4 text-primary" />
                    </div>
                    <div>
                      <p className="font-medium text-sm">{feature.title}</p>
                      <p className="text-xs text-muted-foreground">
                        {feature.description}
                      </p>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>

          <Separator />

          {/* Tech Stack */}
          <div className="space-y-2">
            <h4 className="text-sm font-semibold">Technology Stack</h4>
            <div className="flex flex-wrap gap-2">
              <Badge variant="outline">Rust</Badge>
              <Badge variant="outline">Next.js 16</Badge>
              <Badge variant="outline">React 19</Badge>
              <Badge variant="outline">TypeScript</Badge>
              <Badge variant="outline">Tailwind CSS</Badge>
              <Badge variant="outline">Upbit API</Badge>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Documentation Links */}
      <Card>
        <CardHeader>
          <CardTitle>Documentation & Support</CardTitle>
          <CardDescription>
            Find help and learn more about AutoCoin
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="space-y-3">
            {docsLinks.map((link) => {
              const Icon = link.icon;
              return (
                <Button
                  key={link.href}
                  variant="outline"
                  className="w-full justify-start"
                  asChild
                >
                  <a href={link.href} target="_blank" rel="noopener noreferrer">
                    <Icon className="h-4 w-4 mr-2" />
                    {link.label}
                  </a>
                </Button>
              );
            })}
          </div>
        </CardContent>
      </Card>

      {/* License */}
      <Card>
        <CardHeader>
          <CardTitle>License</CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            AutoCoin is released under the MIT License. You are free to use, modify,
            and distribute this software for personal and commercial purposes.
          </p>
          <p className="text-xs text-muted-foreground mt-2">
            Trading cryptocurrencies involves substantial risk of loss. AutoCoin is
            provided as-is without any warranty. Use at your own risk.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
