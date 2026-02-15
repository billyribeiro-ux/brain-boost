'use client';

import { motion } from 'framer-motion';
import { FiSeed, FiTrendingUp, FiBrain } from 'react-icons/fi';
import { useState } from 'react';

interface ExperienceLevelProps {
  onSelect: (level: string) => void;
}

const levels = [
  { id: 'beginner', label: 'Beginner', icon: FiSeed, description: 'New to cognitive training' },
  { id: 'intermediate', label: 'Intermediate', icon: FiTrendingUp, description: 'Some experience' },
  { id: 'advanced', label: 'Advanced', icon: FiBrain, description: 'Experienced practitioner' },
];

export default function ExperienceLevel({ onSelect }: ExperienceLevelProps) {
  const [selected, setSelected] = useState<string | null>(null);

  const handleSelect = (levelId: string) => {
    setSelected(levelId);
    onSelect(levelId);
  };

  return (
    <div className="flex flex-col items-center p-6 min-h-screen justify-center">
      <h2 className="text-2xl font-bold text-white mb-8 text-center">
        How experienced are you with cognitive training?
      </h2>

      <div className="flex gap-4 flex-wrap justify-center max-w-3xl">
        {levels.map((level, index) => {
          const Icon = level.icon;
          const isSelected = selected === level.id;
          
          return (
            <motion.button
              key={level.id}
              onClick={() => handleSelect(level.id)}
              className={`w-64 p-6 rounded-xl border-2 flex flex-col items-center justify-center gap-3 transition-all ${
                isSelected
                  ? 'border-neuralBlue bg-neuralBlue/10 shadow-glow-blue'
                  : 'border-gray-700 opacity-60 hover:opacity-100'
              }`}
              initial={{ opacity: 0, x: -20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ delay: index * 0.15 }}
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
            >
              <Icon className="w-16 h-16 text-neuralBlue" />
              <div className="text-center">
                <div className="text-white font-bold text-lg mb-1">{level.label}</div>
                <div className="text-sm text-textSecondary">{level.description}</div>
              </div>
            </motion.button>
          );
        })}
      </div>
    </div>
  );
}
