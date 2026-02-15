'use client';

import { motion } from 'framer-motion';
import { FiBook, FiTrendingUp, FiShield, FiZap } from 'react-icons/fi';
import { useState } from 'react';

interface GoalPickerProps {
  onSelect: (goals: string[]) => void;
}

const goals = [
  {
    id: 'focus',
    label: 'Rapid Learning',
    icon: FiBook,
    gradient: 'from-neuralBlue',
    description: 'Compress months into days',
  },
  {
    id: 'performance',
    label: 'Trading Mastery',
    icon: FiTrendingUp,
    gradient: 'from-successGreen',
    description: 'Elite decision-making',
  },
  {
    id: 'recovery',
    label: 'Stress-Proof',
    icon: FiShield,
    gradient: 'from-creativePurple',
    description: 'Calm under any pressure',
  },
  {
    id: 'longevity',
    label: 'Max Neural',
    icon: FiZap,
    gradient: 'from-orangeAccent',
    description: 'Maximum cognitive power',
  },
];

export default function GoalPicker({ onSelect }: GoalPickerProps) {
  const [selected, setSelected] = useState<string[]>([]);

  const toggleGoal = (goalId: string) => {
    const newSelected = selected.includes(goalId)
      ? selected.filter((id) => id !== goalId)
      : [...selected, goalId];
    
    setSelected(newSelected);
    onSelect(newSelected);
  };

  return (
    <div className="flex flex-col items-center p-6 min-h-screen justify-center">
      <h2 className="text-2xl font-bold text-white mb-8 text-center">
        What's your primary super-brain goal?
      </h2>

      <div className="grid grid-cols-2 gap-4 w-full max-w-2xl">
        {goals.map((goal, index) => {
          const Icon = goal.icon;
          const isSelected = selected.includes(goal.id);
          
          return (
            <motion.button
              key={goal.id}
              onClick={() => toggleGoal(goal.id)}
              className={`relative p-6 rounded-xl border-2 flex flex-col items-center justify-center gap-3 transition-all ${
                isSelected
                  ? 'border-neuralBlue bg-neuralBlue/10 shadow-glow-blue'
                  : 'border-gray-700 hover:border-gray-600'
              }`}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: index * 0.1 }}
              whileHover={{ rotateY: 3, scale: 1.02 }}
              whileTap={{ scale: 0.95 }}
            >
              <div className={`absolute top-0 left-0 right-0 h-1 rounded-t-xl bg-gradient-to-r ${goal.gradient} to-transparent`} />
              
              {isSelected && (
                <motion.div
                  className="absolute top-2 right-2 w-6 h-6 bg-neuralBlue rounded-full flex items-center justify-center"
                  initial={{ scale: 0 }}
                  animate={{ scale: 1 }}
                  transition={{ type: 'spring', stiffness: 500 }}
                >
                  <svg className="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                  </svg>
                </motion.div>
              )}

              <Icon className="w-12 h-12 text-neuralBlue" />
              <div className="text-center">
                <div className="text-white font-bold mb-1">{goal.label}</div>
                <div className="text-sm text-textSecondary">{goal.description}</div>
              </div>
            </motion.button>
          );
        })}
      </div>

      {selected.length > 0 && (
        <motion.p
          className="mt-6 text-textSecondary text-sm"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
        >
          {selected.length} goal{selected.length > 1 ? 's' : ''} selected
        </motion.p>
      )}
    </div>
  );
}
