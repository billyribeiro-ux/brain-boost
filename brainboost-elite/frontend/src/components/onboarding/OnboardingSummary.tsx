'use client';

import { motion } from 'framer-motion';
import { FiClock, FiCalendar, FiTarget } from 'react-icons/fi';
import { useState } from 'react';

interface OnboardingSummaryProps {
  chronotype: string;
  goals: string[];
  experienceLevel: string;
  schedule: {
    morning_time: string;
    afternoon_time: string;
    evening_time: string;
  };
  onComplete: () => void;
}

export default function OnboardingSummary({
  chronotype,
  goals,
  experienceLevel,
  schedule,
  onComplete,
}: OnboardingSummaryProps) {
  const [isLoading, setIsLoading] = useState(false);

  const handleBegin = async () => {
    setIsLoading(true);
    await onComplete();
  };

  return (
    <div className="flex flex-col items-center p-6 min-h-screen justify-center">
      <motion.h2
        className="text-3xl font-bold text-white mb-8 text-center"
        initial={{ opacity: 0, y: -20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        Your Personalized Plan
      </motion.h2>

      <motion.div
        className="w-full max-w-md bg-surface-card rounded-xl p-6 space-y-6 mb-8"
        initial={{ opacity: 0, scale: 0.9 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ delay: 0.2 }}
      >
        <div className="flex items-start gap-4">
          <div className="w-12 h-12 rounded-full bg-neuralBlue/20 flex items-center justify-center flex-shrink-0">
            <FiTarget className="w-6 h-6 text-neuralBlue" />
          </div>
          <div>
            <div className="text-white font-bold mb-1">Chronotype</div>
            <div className="text-textSecondary capitalize">{chronotype}</div>
          </div>
        </div>

        <div className="flex items-start gap-4">
          <div className="w-12 h-12 rounded-full bg-creativePurple/20 flex items-center justify-center flex-shrink-0">
            <FiTarget className="w-6 h-6 text-creativePurple" />
          </div>
          <div>
            <div className="text-white font-bold mb-1">Goals</div>
            <div className="text-textSecondary capitalize">{goals.join(', ')}</div>
          </div>
        </div>

        <div className="flex items-start gap-4">
          <div className="w-12 h-12 rounded-full bg-successGreen/20 flex items-center justify-center flex-shrink-0">
            <FiCalendar className="w-6 h-6 text-successGreen" />
          </div>
          <div>
            <div className="text-white font-bold mb-1">Starting Level</div>
            <div className="text-textSecondary">Level 1: Neural Ignition</div>
          </div>
        </div>

        <div className="flex items-start gap-4">
          <div className="w-12 h-12 rounded-full bg-orangeAccent/20 flex items-center justify-center flex-shrink-0">
            <FiClock className="w-6 h-6 text-orangeAccent" />
          </div>
          <div>
            <div className="text-white font-bold mb-1">Daily Commitment</div>
            <div className="text-textSecondary">45 minutes per day</div>
          </div>
        </div>

        <div className="border-t border-gray-700 pt-4">
          <div className="text-white font-bold mb-3">Your Schedule</div>
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-textSecondary">Morning</span>
              <span className="text-white font-mono">{schedule.morning_time}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-textSecondary">Afternoon</span>
              <span className="text-white font-mono">{schedule.afternoon_time}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-textSecondary">Evening</span>
              <span className="text-white font-mono">{schedule.evening_time}</span>
            </div>
          </div>
        </div>
      </motion.div>

      <motion.button
        onClick={handleBegin}
        disabled={isLoading}
        className="w-full max-w-md h-14 bg-gradient-to-r from-neuralBlue to-creativePurple text-white font-bold rounded-2xl shadow-lg disabled:opacity-50"
        whileHover={{ scale: 1.02, boxShadow: '0 0 30px rgba(0, 123, 255, 0.6)' }}
        whileTap={{ scale: 0.95 }}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.4 }}
      >
        {isLoading ? (
          <span className="flex items-center justify-center gap-2">
            <svg className="animate-spin h-5 w-5" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
            </svg>
            Setting up...
          </span>
        ) : (
          'Begin Your Journey'
        )}
      </motion.button>
    </div>
  );
}
