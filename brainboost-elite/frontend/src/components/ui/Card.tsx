'use client';

import { HTMLAttributes, forwardRef } from 'react';
import { motion } from 'framer-motion';
import { cn } from '@/lib/utils';

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  hover?: boolean;
}

const Card = forwardRef<HTMLDivElement, CardProps>(
  ({ className, hover = false, children, ...props }, ref) => {
    const Component = hover ? motion.div : 'div';
    
    return (
      <Component
        ref={ref}
        className={cn(
          'bg-surface-card rounded-xl shadow-md p-6 max-w-full',
          hover && 'cursor-pointer',
          className
        )}
        {...(hover && {
          whileHover: { scale: 1.02, boxShadow: '0 0 20px rgba(0, 123, 255, 0.3)' },
          transition: { duration: 0.3 },
        })}
        {...props}
      >
        {children}
      </Component>
    );
  }
);

Card.displayName = 'Card';

export default Card;
