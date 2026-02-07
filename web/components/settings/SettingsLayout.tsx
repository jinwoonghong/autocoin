"use client";

/**
 * SettingsLayout Component
 *
 * Provides sidebar navigation for settings sections
 * Sections: Strategy, Risk Management, System, Notifications, About
 */

import React, { useState } from "react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { SettingsSection, SettingsNavItem } from "@/types/settings";
import {
  IconTrendingUp,
  IconShield,
  IconServer,
  IconBell,
  IconInfo,
} from "@/components/ui/icons";

interface SettingsLayoutProps {
  children: React.ReactNode;
  defaultSection?: SettingsSection;
}

const settingsNavItems: SettingsNavItem[] = [
  {
    id: "strategy",
    label: "Strategy",
    icon: "trending-up",
    description: "Configure trading strategy parameters",
  },
  {
    id: "risk",
    label: "Risk Management",
    icon: "shield",
    description: "Set stop loss, take profit, and position limits",
  },
  {
    id: "system",
    label: "System",
    icon: "server",
    description: "Control trading system and view status",
  },
  {
    id: "notifications",
    label: "Notifications",
    icon: "bell",
    description: "Configure Discord and alert preferences",
  },
  {
    id: "about",
    label: "About",
    icon: "info",
    description: "System information and version details",
  },
];

const iconMap: Record<string, React.ComponentType<{ className?: string }>> = {
  "trending-up": IconTrendingUp,
  shield: IconShield,
  server: IconServer,
  bell: IconBell,
  info: IconInfo,
};

export function SettingsLayout({
  children,
  defaultSection = "strategy",
}: SettingsLayoutProps) {
  const [activeSection, setActiveSection] = useState<SettingsSection>(defaultSection);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-8">
        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold tracking-tight">Settings</h1>
          <p className="text-muted-foreground mt-2">
            Configure your AutoCoin trading system
          </p>
        </div>

        {/* Mobile menu button */}
        <div className="lg:hidden mb-4">
          <Button
            variant="outline"
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            className="w-full justify-between"
          >
            <span>
              {settingsNavItems.find((item) => item.id === activeSection)?.label}
            </span>
            <svg
              className={cn(
                "h-4 w-4 transition-transform",
                isMobileMenuOpen ? "rotate-180" : ""
              )}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 9l-7 7-7-7"
              />
            </svg>
          </Button>
        </div>

        <div className="flex flex-col lg:flex-row gap-8">
          {/* Sidebar Navigation */}
          <nav
            className={cn(
              "lg:w-64 flex-shrink-0",
              !isMobileMenuOpen && "hidden lg:block"
            )}
          >
            <Card className="p-2">
              <ul className="space-y-1">
                {settingsNavItems.map((item) => {
                  const IconComponent = iconMap[item.icon];
                  const isActive = activeSection === item.id;

                  return (
                    <li key={item.id}>
                      <button
                        onClick={() => {
                          setActiveSection(item.id);
                          setIsMobileMenuOpen(false);
                        }}
                        className={cn(
                          "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors",
                          isActive
                            ? "bg-primary text-primary-foreground"
                            : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
                        )}
                      >
                        {IconComponent && (
                          <IconComponent className="h-4 w-4 flex-shrink-0" />
                        )}
                        <span className="truncate">{item.label}</span>
                      </button>
                    </li>
                  );
                })}
              </ul>
            </Card>
          </nav>

          {/* Content Area */}
          <main className="flex-1 min-w-0">
            {React.Children.map(children, (child) => {
              if (React.isValidElement(child)) {
                return React.cloneElement(child as React.ReactElement<any>, {
                  section: activeSection,
                });
              }
              return child;
            })}
          </main>
        </div>
      </div>
    </div>
  );
}
