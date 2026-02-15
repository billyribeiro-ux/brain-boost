'use client';

import { InputHTMLAttributes, forwardRef } from 'react';
import { cn } from '@/lib/utils';

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
}

const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ className, label, error, type = 'text', ...props }, ref) => {
    return (
      <div className="w-full">
        {label && (
          <label className="block text-sm font-medium text-textSecondary mb-2">
            {label}
          </label>
        )}
        <input
          ref={ref}
          type={type}
          className={cn(
            'w-full h-12 px-4 bg-surface-elevated text-textPrimary rounded-xl border-2 border-transparent',
            'focus:outline-none focus:border-neuralBlue focus:ring-2 focus:ring-neuralBlue/20',
            'transition-all duration-300',
            'placeholder:text-textMuted',
            error && 'border-warningRed focus:border-warningRed focus:ring-warningRed/20 animate-shake-error',
            className
          )}
          {...props}
        />
        {error && (
          <p className="mt-2 text-sm text-warningRed">{error}</p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

export default Input;
