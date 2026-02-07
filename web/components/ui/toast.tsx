"use client";

/**
 * Toast Component
 *
 * Simple toast notification component for displaying success/error messages
 */

import React, { useEffect, useState } from "react";
import { cn } from "@/lib/utils";
import { IconCheck, IconX, IconWarning } from "@/components/ui/icons";

export type ToastType = "success" | "error" | "warning" | "info";

export interface ToastProps {
  message: string;
  type?: ToastType;
  duration?: number;
  onClose?: () => void;
}

export function Toast({
  message,
  type = "info",
  duration = 3000,
  onClose,
}: ToastProps) {
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    const timer = setTimeout(() => {
      setIsVisible(false);
      onClose?.();
    }, duration);

    return () => clearTimeout(timer);
  }, [duration, onClose]);

  const icons = {
    success: IconCheck,
    error: IconX,
    warning: IconWarning,
    info: IconWarning,
  };

  const styles = {
    success: "bg-green-500 text-white",
    error: "bg-destructive text-destructive-foreground",
    warning: "bg-yellow-500 text-white",
    info: "bg-primary text-primary-foreground",
  };

  const Icon = icons[type];

  if (!isVisible) return null;

  return (
    <div className="fixed top-4 left-1/2 -translate-x-1/2 z-50 animate-in slide-in-from-top fade-in duration-300">
      <div
        className={cn(
          "flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg",
          styles[type]
        )}
      >
        <Icon className="h-4 w-4 flex-shrink-0" />
        <span className="text-sm font-medium">{message}</span>
        <button
          onClick={() => {
            setIsVisible(false);
            onClose?.();
          }}
          className="ml-2 opacity-70 hover:opacity-100 transition-opacity"
        >
          <svg
            className="h-4 w-4"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>
  );
}

/**
 * Toast Hook
 *
 * Simple hook for managing toasts in components
 */
export function useToast() {
  const [toast, setToast] = useState<{
    message: string;
    type: ToastType;
  } | null>(null);

  const showToast = (message: string, type: ToastType = "info") => {
    setToast({ message, type });
  };

  const hideToast = () => {
    setToast(null);
  };

  const ToastComponent = toast ? (
    <Toast
      key={toast.message + toast.type}
      message={toast.message}
      type={toast.type}
      onClose={hideToast}
    />
  ) : null;

  return {
    showToast,
    hideToast,
    toast: ToastComponent,
  };
}
