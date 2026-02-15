'use client';

import { motion } from 'framer-motion';
import { useEffect, useState } from 'react';

interface BrainScoreGaugeProps {
  score: number;
}

export default function BrainScoreGauge({ score }: BrainScoreGaugeProps) {
  const [displayScore, setDisplayScore] = useState(0);
  const radius = 52;
  const circumference = 2 * Math.PI * radius;
  const progress = (score / 100) * circumference;

  useEffect(() => {
    const timer = setTimeout(() => setDisplayScore(score), 100);
    return () => clearTimeout(timer);
  }, [score]);

  return (
    <div className="flex flex-col items-center">
      <div className="relative w-32 h-32">
        <svg className="w-full h-full transform -rotate-90" viewBox="0 0 120 120">
          <circle
            cx="60"
            cy="60"
            r={radius}
            stroke="#2A2A2A"
            strokeWidth="8"
            fill="none"
          />
          
          <motion.circle
            cx="60"
            cy="60"
            r={radius}
            stroke="#007BFF"
            strokeWidth="8"
            fill="none"
            strokeLinecap="round"
            strokeDasharray={circumference}
            initial={{ strokeDashoffset: circumference }}
            animate={{ strokeDashoffset: circumference - progress }}
            transition={{ duration: 1.5, ease: 'easeOut' }}
          />

          {[...Array(5)].map((_, i) => (
            <motion.circle
              key={i}
              cx={60 + Math.cos((progress / circumference) * 2 * Math.PI - Math.PI / 2) * radius}
              cy={60 + Math.sin((progress / circumference) * 2 * Math.PI - Math.PI / 2) * radius}
              r="2"
              fill="#007BFF"
              initial={{ opacity: 0, scale: 0 }}
              animate={{
                opacity: [0, 1, 0],
                scale: [0, 1.5, 0],
                x: [0, Math.random() * 10 - 5],
                y: [0, Math.random() * 10 - 5],
              }}
              transition={{
                duration: 0.8,
                delay: 1.5 + i * 0.1,
                ease: 'easeOut',
              }}
            />
          ))}
        </svg>

        <div className="absolute inset-0 flex flex-col items-center justify-center">
          <motion.div
            className="text-3xl font-bold text-white"
            initial={{ opacity: 0, scale: 0.5 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.5, duration: 0.5 }}
          >
            {Math.round(displayScore)}
          </motion.div>
          <div className="text-sm text-textMuted">/100</div>
        </div>
      </div>

      <div className="text-sm text-textSecondary mt-2">Brain Score</div>
    </div>
  );
}
