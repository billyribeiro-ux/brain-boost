'use client';

import { motion, AnimatePresence } from 'framer-motion';
import { FiSunrise, FiSun, FiMoon, FiChevronDown, FiCheck, FiX } from 'react-icons/fi';
import { useState } from 'react';

interface Exercise {
  id: string;
  name: string;
  duration_seconds: number;
  completed: boolean;
  accuracy?: number;
}

interface Session {
  id: string;
  slot: 'morning' | 'afternoon' | 'evening';
  time: string;
  status: 'pending' | 'in_progress' | 'completed' | 'missed';
  exercises: Exercise[];
}

interface DailyChecklistProps {
  sessions: Session[];
  onExerciseClick: (sessionId: string, exerciseId: string) => void;
}

const slotConfig = {
  morning: { icon: FiSunrise, label: 'Morning', color: 'text-orangeAccent' },
  afternoon: { icon: FiSun, label: 'Afternoon', color: 'text-yellow-500' },
  evening: { icon: FiMoon, label: 'Evening', color: 'text-creativePurple' },
};

const statusConfig = {
  pending: { label: 'Pending', color: 'bg-gray-600', textColor: 'text-gray-300' },
  in_progress: { label: 'In Progress', color: 'bg-neuralBlue', textColor: 'text-neuralBlue', pulse: true },
  completed: { label: 'Completed', color: 'bg-successGreen', textColor: 'text-successGreen' },
  missed: { label: 'Missed', color: 'bg-warningRed', textColor: 'text-warningRed' },
};

export default function DailyChecklist({ sessions, onExerciseClick }: DailyChecklistProps) {
  const [expandedSession, setExpandedSession] = useState<string | null>(null);

  const toggleSession = (sessionId: string) => {
    setExpandedSession(expandedSession === sessionId ? null : sessionId);
  };

  return (
    <div className="space-y-3">
      {sessions.map((session, index) => {
        const SlotIcon = slotConfig[session.slot].icon;
        const isExpanded = expandedSession === session.id;
        const statusInfo = statusConfig[session.status];

        return (
          <motion.div
            key={session.id}
            className="bg-surface-card rounded-xl overflow-hidden"
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: index * 0.1 }}
            layout
          >
            <button
              onClick={() => toggleSession(session.id)}
              className="w-full p-4 flex items-center justify-between hover:bg-surface-elevated/50 transition-colors"
            >
              <div className="flex items-center gap-3">
                <SlotIcon className={`w-5 h-5 ${slotConfig[session.slot].color}`} />
                <div className="text-left">
                  <div className="text-white font-medium">{slotConfig[session.slot].label}</div>
                  <div className="text-sm text-textSecondary">{session.time}</div>
                </div>
              </div>

              <div className="flex items-center gap-3">
                <div className={`px-3 py-1 rounded-full text-xs font-medium ${statusInfo.color} ${statusInfo.textColor}`}>
                  {statusInfo.pulse && (
                    <motion.span
                      className="inline-block w-2 h-2 rounded-full bg-current mr-1"
                      animate={{ opacity: [1, 0.3, 1] }}
                      transition={{ duration: 1.5, repeat: Infinity }}
                    />
                  )}
                  {statusInfo.label}
                </div>

                <motion.div
                  animate={{ rotate: isExpanded ? 180 : 0 }}
                  transition={{ duration: 0.3 }}
                >
                  <FiChevronDown className="w-5 h-5 text-textSecondary" />
                </motion.div>
              </div>
            </button>

            <AnimatePresence>
              {isExpanded && (
                <motion.div
                  initial={{ height: 0, opacity: 0 }}
                  animate={{ height: 'auto', opacity: 1 }}
                  exit={{ height: 0, opacity: 0 }}
                  transition={{ duration: 0.3, ease: 'easeInOut' }}
                  className="border-t border-gray-800"
                >
                  <div className="p-4 space-y-2">
                    {session.exercises.map((exercise) => (
                      <motion.button
                        key={exercise.id}
                        onClick={() => !exercise.completed && onExerciseClick(session.id, exercise.id)}
                        className={`w-full p-3 rounded-lg flex items-center gap-3 transition-all ${
                          exercise.completed
                            ? 'opacity-70'
                            : 'hover:bg-surface-elevated border-l-3 border-neuralBlue'
                        }`}
                        disabled={exercise.completed}
                        whileHover={!exercise.completed ? { x: 4 } : {}}
                        whileTap={!exercise.completed ? { scale: 0.98 } : {}}
                      >
                        <div className={`w-6 h-6 rounded-full border-2 flex items-center justify-center flex-shrink-0 ${
                          exercise.completed ? 'bg-successGreen border-successGreen' : 'border-gray-600'
                        }`}>
                          {exercise.completed && (
                            <motion.div
                              initial={{ scale: 0 }}
                              animate={{ scale: [0, 1.2, 1] }}
                              transition={{ type: 'spring', stiffness: 300 }}
                            >
                              <FiCheck className="w-4 h-4 text-white" />
                            </motion.div>
                          )}
                        </div>

                        <div className="flex-1 text-left">
                          <div className={`text-white ${exercise.completed ? 'line-through' : ''}`}>
                            {exercise.name}
                          </div>
                          <div className="text-sm text-textSecondary">
                            {Math.floor(exercise.duration_seconds / 60)} min
                            {exercise.accuracy && ` • ${exercise.accuracy}% accuracy`}
                          </div>
                        </div>
                      </motion.button>
                    ))}
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </motion.div>
        );
      })}
    </div>
  );
}
