import * as React from "react";
import { cn } from "@/lib/utils";

export interface SelectProps
  extends React.SelectHTMLAttributes<HTMLSelectElement> {}

const Select = React.forwardRef<HTMLSelectElement, SelectProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <select
        className={cn(
          "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
          className
        )}
        ref={ref}
        {...props}
      >
        {children}
      </select>
    );
  }
);
Select.displayName = "Select";

export { Select };

export interface SelectOptionProps
  extends React.OptionHTMLAttributes<HTMLOptionElement> {}

const SelectOption = React.forwardRef<HTMLOptionElement, SelectOptionProps>(
  ({ className, children, ...props }, ref) => {
    return (
      <option
        className={cn(className)}
        ref={ref}
        {...props}
      >
        {children}
      </option>
    );
  }
);
SelectOption.displayName = "SelectOption";

export { SelectOption as Option };
