'use client';

import { motion } from 'framer-motion';

interface ReadinessGaugeProps {
  stressIndex: number;
}

export default function ReadinessGauge({ stressIndex }: ReadinessGaugeProps) {
  const readiness = Math.max(0, Math.min(100, 100 - stressIndex));
  
  const getColor = () => {
    if (readiness >= 70) return '#28A745';
    if (readiness >= 40) return '#FFC107';
    return '#DC3545';
  };

  const getTextColor = () => {
    if (readiness >= 70) return 'text-successGreen';
    if (readiness >= 40) return 'text-yellow-500';
    return 'text-warningRed';
  };

  return (
    <div className="w-full">
      <div className="flex items-center justify-between mb-2">
        <span className="text-sm text-textSecondary">Readiness</span>
        <span className={`text-sm font-bold ${getTextColor()}`}>
          {Math.round(readiness)}%
        </span>
      </div>

      <div className="relative w-full h-3 bg-surface-elevated rounded-full overflow-hidden">
        <motion.div
          className="absolute inset-y-0 left-0 rounded-full"
          style={{
            background: `linear-gradient(to right, ${getColor()}, ${getColor()})`,
          }}
          initial={{ width: 0 }}
          animate={{ width: `${readiness}%` }}
          transition={{ duration: 1, ease: 'easeOut' }}
        />

        {readiness < 40 && (
          <motion.div
            className="absolute inset-0 bg-warningRed/30 rounded-full"
            animate={{ opacity: [0.3, 0.7, 0.3] }}
            transition={{ duration: 1.5, repeat: Infinity }}
          />
        )}
      </div>

      {readiness < 40 && (
        <motion.p
          className="text-xs text-warningRed mt-2"
          initial={{ opacity: 0, y: -5 }}
          animate={{ opacity: 1, y: 0 }}
        >
          Low readiness detected — NSDR recommended
        </motion.p>
      )}
    </div>
  );
}
