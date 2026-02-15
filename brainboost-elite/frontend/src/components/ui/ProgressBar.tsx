'use client';

import { motion } from 'framer-motion';
import { cn } from '@/lib/utils';

interface ProgressBarProps {
  value: number;
  max?: number;
  className?: string;
  showLabel?: boolean;
  color?: 'blue' | 'purple' | 'green' | 'orange';
}

export default function ProgressBar({
  value,
  max = 100,
  className,
  showLabel = true,
  color = 'blue',
}: ProgressBarProps) {
  const percentage = Math.min((value / max) * 100, 100);

  const colors = {
    blue: 'bg-neuralBlue',
    purple: 'bg-creativePurple',
    green: 'bg-successGreen',
    orange: 'bg-orangeAccent',
  };

  return (
    <div className={cn('w-full', className)}>
      <div className="relative h-3 bg-surface-elevated rounded-full overflow-hidden">
        <motion.div
          className={cn('h-full rounded-full', colors[color])}
          initial={{ width: 0 }}
          animate={{ width: `${percentage}%` }}
          transition={{ duration: 0.8, ease: 'easeOut' }}
        />
      </div>
      {showLabel && (
        <div className="flex justify-between mt-2 text-sm text-textSecondary">
          <span>{value}</span>
          <span>{max}</span>
        </div>
      )}
    </div>
  );
}
