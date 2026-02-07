import * as React from "react";
import { cn } from "@/lib/utils";

interface SwitchProps extends Omit<React.InputHTMLAttributes<HTMLInputElement>, 'type'> {
  thumbClassName?: string;
}

const Switch = React.forwardRef<HTMLInputElement, SwitchProps>(
  ({ className, thumbClassName, ...props }, ref) => {
    return (
      <label className="relative inline-flex items-center cursor-pointer">
        <input
          type="checkbox"
          className="sr-only peer"
          ref={ref}
          {...props}
        />
        <div
          className={cn(
            "peer h-6 w-11 rounded-full border-2 border-input bg-muted transition-all after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:h-5 after:w-5 after:rounded-full after:border after:border-muted-foreground/20 after:bg-background after:transition-all peer-focus-visible:outline-none peer-focus-visible:ring-2 peer-focus-visible:ring-ring peer-focus-visible:ring-offset-2 peer-disabled:cursor-not-allowed peer-disabled:opacity-50 peer-checked:bg-primary peer-checked:border-primary peer-checked:after:translate-x-full peer-checked:after:border-white",
            className
          )}
        />
      </label>
    );
  }
);
Switch.displayName = "Switch";

export { Switch };
