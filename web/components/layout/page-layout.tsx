/**
 * Page Layout Component
 *
 * Standard layout wrapper for pages
 */

import * as React from "react";
import { cn } from "@/lib/utils";

export interface PageLayoutProps {
  children: React.ReactNode;
  title: string;
  description?: string;
  actions?: React.ReactNode;
  className?: string;
}

export function PageLayout({
  children,
  title,
  description,
  actions,
  className,
}: PageLayoutProps) {
  return (
    <div className={cn("py-6 px-4 max-w-7xl mx-auto", className)}>
      {/* Page Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">{title}</h1>
          {description && (
            <p className="text-muted-foreground mt-1">{description}</p>
          )}
        </div>
        {actions && <div className="flex items-center gap-2">{actions}</div>}
      </div>

      {/* Page Content */}
      {children}
    </div>
  );
}
