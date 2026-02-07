/**
 * Main navigation component for the dashboard
 */

"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { cn } from "@/lib/utils";
import {
  LayoutDashboard,
  TrendingUp,
  History,
  FlaskConical,
  Settings,
  Menu,
  X,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { ThemeToggle } from "./theme-toggle";

const navItems = [
  {
    name: "Dashboard",
    href: "/dashboard",
    icon: LayoutDashboard,
  },
  {
    name: "Markets",
    href: "/markets",
    icon: TrendingUp,
  },
  {
    name: "Trades",
    href: "/trades",
    icon: History,
  },
  {
    name: "Backtest",
    href: "/backtest",
    icon: FlaskConical,
  },
  {
    name: "Settings",
    href: "/settings",
    icon: Settings,
  },
];

export function Navigation() {
  const pathname = usePathname();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <>
      {/* Mobile Header */}
      <header className="sticky top-0 z-50 flex h-16 items-center gap-4 border-b bg-background px-4 lg:hidden">
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
        >
          {mobileMenuOpen ? <X className="h-5 w-5" /> : <Menu className="h-5 w-5" />}
        </Button>
        <div className="flex-1">
          <h1 className="text-lg font-bold">AutoCoin</h1>
        </div>
        <ThemeToggle />
      </header>

      {/* Mobile Menu Overlay */}
      {mobileMenuOpen && (
        <div
          className="fixed inset-0 z-40 bg-black/50 lg:hidden"
          onClick={() => setMobileMenuOpen(false)}
        />
      )}

      {/* Sidebar Navigation */}
      <aside
        className={cn(
          "fixed inset-y-0 left-0 z-50 w-64 transform border-r bg-background transition-transform duration-200 ease-in-out lg:translate-x-0",
          mobileMenuOpen ? "translate-x-0" : "-translate-x-full"
        )}
      >
        <div className="flex h-full flex-col">
          {/* Logo */}
          <div className="flex h-16 items-center justify-between border-b px-6">
            <h1 className="text-xl font-bold">AutoCoin</h1>
            <Button
              variant="ghost"
              size="icon"
              className="lg:hidden"
              onClick={() => setMobileMenuOpen(false)}
            >
              <X className="h-5 w-5" />
            </Button>
          </div>

          {/* Navigation */}
          <nav className="flex-1 overflow-y-auto p-4">
            <ul className="space-y-1">
              {navItems.map((item) => {
                const Icon = item.icon;
                const isActive = pathname === item.href;

                return (
                  <li key={item.href}>
                    <Link
                      href={item.href}
                      onClick={() => setMobileMenuOpen(false)}
                      className={cn(
                        "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors",
                        isActive
                          ? "bg-primary text-primary-foreground"
                          : "text-muted-foreground hover:bg-accent hover:text-accent-foreground"
                      )}
                    >
                      <Icon className="h-5 w-5" />
                      {item.name}
                    </Link>
                  </li>
                );
              })}
            </ul>
          </nav>

          {/* Footer */}
          <div className="border-t p-4">
            <div className="flex items-center justify-between">
              <span className="text-xs text-muted-foreground">Theme</span>
              <ThemeToggle />
            </div>
          </div>
        </div>
      </aside>
    </>
  );
}
