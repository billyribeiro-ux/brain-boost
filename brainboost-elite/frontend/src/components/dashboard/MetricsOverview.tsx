'use client';

import { motion } from 'framer-motion';
import { FiActivity, FiShield, FiTrendingUp } from 'react-icons/fi';
import { FaFire } from 'react-icons/fa';

interface MetricsOverviewProps {
  brainScore: number;
  streak: number;
  stressIndex: number;
  longevityScore: number;
}

export default function MetricsOverview({
  brainScore,
  streak,
  stressIndex,
  longevityScore,
}: MetricsOverviewProps) {
  const metrics = [
    {
      label: 'Brain Score',
      value: Math.round(brainScore),
      icon: FiActivity,
      color: 'text-neuralBlue',
      bgColor: 'bg-neuralBlue/10',
    },
    {
      label: 'Streak',
      value: `${streak}d`,
      icon: FaFire,
      color: 'text-orangeAccent',
      bgColor: 'bg-orangeAccent/10',
    },
    {
      label: 'Stress',
      value: Math.round(stressIndex),
      icon: FiActivity,
      color: stressIndex < 40 ? 'text-successGreen' : stressIndex < 70 ? 'text-yellow-500' : 'text-warningRed',
      bgColor: stressIndex < 40 ? 'bg-successGreen/10' : stressIndex < 70 ? 'bg-yellow-500/10' : 'bg-warningRed/10',
    },
    {
      label: 'Longevity',
      value: Math.round(longevityScore),
      icon: FiShield,
      color: 'text-successGreen',
      bgColor: 'bg-successGreen/10',
    },
  ];

  return (
    <div className="flex gap-3 overflow-x-auto pb-2 scrollbar-hide">
      {metrics.map((metric, index) => {
        const Icon = metric.icon;
        
        return (
          <motion.div
            key={metric.label}
            className={`flex-shrink-0 w-20 h-20 ${metric.bgColor} rounded-lg flex flex-col items-center justify-center gap-1`}
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            whileHover={{ scale: 1.05 }}
          >
            <Icon className={`w-5 h-5 ${metric.color}`} />
            <div className={`text-lg font-bold ${metric.color}`}>
              {metric.value}
            </div>
            <div className="text-xs text-textSecondary">{metric.label}</div>
          </motion.div>
        );
      })}
    </div>
  );
}
