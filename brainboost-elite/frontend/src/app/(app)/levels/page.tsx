'use client';

import { useQuery } from '@tanstack/react-query';
import Card from '@/components/ui/Card';
import ProgressBar from '@/components/ui/ProgressBar';
import { api } from '@/lib/api';
import { LEVEL_NAMES } from '@/lib/constants';
import { FiLock, FiCheck, FiZap } from 'react-icons/fi';

export default function LevelsPage() {
  const { data: levels } = useQuery({
    queryKey: ['levels'],
    queryFn: async () => {
      const response = await api.progress.getLevels();
      return response.data;
    },
  });

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold text-textPrimary">Your Journey</h1>
        <p className="text-textSecondary mt-1">
          6 levels × 6 weeks each = 36-week transformation
        </p>
      </div>

      <div className="space-y-4">
        {levels?.map((level: any) => {
          const isLocked = level.status === 'locked';
          const isActive = level.status === 'active';
          const isCompleted = level.status === 'completed';

          return (
            <Card
              key={level.id}
              hover={!isLocked}
              className={`${
                isLocked ? 'opacity-50' : ''
              } ${isActive ? 'border-2 border-neuralBlue' : ''}`}
            >
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    {isCompleted && <FiCheck className="w-6 h-6 text-successGreen" />}
                    {isActive && <FiZap className="w-6 h-6 text-neuralBlue animate-pulse" />}
                    {isLocked && <FiLock className="w-6 h-6 text-textMuted" />}
                    <h3 className="text-xl font-bold text-textPrimary">
                      Level {level.level_number}: {LEVEL_NAMES[level.level_number as keyof typeof LEVEL_NAMES]}
                    </h3>
                  </div>

                  {isActive && (
                    <div className="mt-4">
                      <div className="flex justify-between text-sm text-textSecondary mb-2">
                        <span>Week {level.current_week}, Day {level.current_day}</span>
                        <span>{level.compliance_rate?.toFixed(0)}% Complete</span>
                      </div>
                      <ProgressBar
                        value={level.compliance_rate || 0}
                        color="blue"
                        showLabel={false}
                      />
                    </div>
                  )}

                  {isCompleted && level.completed_at && (
                    <p className="text-sm text-successGreen mt-2">
                      Completed on {new Date(level.completed_at).toLocaleDateString()}
                    </p>
                  )}
                </div>
              </div>
            </Card>
          );
        })}
      </div>
    </div>
  );
}
