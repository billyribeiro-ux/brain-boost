'use client';

import { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import Card from '@/components/ui/Card';
import Button from '@/components/ui/Button';
import ProgressBar from '@/components/ui/ProgressBar';
import { api } from '@/lib/api';
import { authStore } from '@/stores/authStore';
import { progressStore } from '@/stores/progressStore';
import { metricsStore } from '@/stores/metricsStore';
import { FiBrain, FiZap, FiTrendingUp, FiTarget } from 'react-icons/fi';

export default function DashboardPage() {
  const user = authStore((state) => state.user);
  const { brainScore, stressIndex, longevityScore } = metricsStore();
  const { currentLevel, streak, compliance } = progressStore();

  const { data: progressData } = useQuery({
    queryKey: ['progress'],
    queryFn: async () => {
      const response = await api.progress.getCurrent();
      return response.data;
    },
  });

  const { data: metricsData } = useQuery({
    queryKey: ['metrics'],
    queryFn: async () => {
      const response = await api.metrics.getToday();
      return response.data;
    },
  });

  useEffect(() => {
    if (progressData) {
      progressStore.getState().setProgress(
        progressData.current_level,
        progressData.today_sessions,
        metricsData?.streak_days || 0
      );
    }
  }, [progressData, metricsData]);

  useEffect(() => {
    if (metricsData) {
      metricsStore.getState().setMetrics(metricsData);
    }
  }, [metricsData]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-textPrimary">
            Welcome back, {user?.display_name}
          </h1>
          <p className="text-textSecondary mt-1">
            Level {currentLevel?.level_number || 1} - Week {currentLevel?.current_week || 1}, Day {currentLevel?.current_day || 1}
          </p>
        </div>
        <div className="flex items-center gap-2">
          <div className="text-center">
            <div className="text-3xl font-bold text-orangeAccent">{streak}</div>
            <div className="text-xs text-textSecondary">Day Streak</div>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <Card className="text-center">
          <FiBrain className="w-12 h-12 text-neuralBlue mx-auto mb-3" />
          <div className="text-3xl font-bold text-textPrimary mb-1">
            {brainScore?.toFixed(0) || 0}
          </div>
          <div className="text-sm text-textSecondary">Brain Score</div>
        </Card>

        <Card className="text-center">
          <FiZap className="w-12 h-12 text-successGreen mx-auto mb-3" />
          <div className="text-3xl font-bold text-textPrimary mb-1">
            {(100 - (stressIndex || 0)).toFixed(0)}
          </div>
          <div className="text-sm text-textSecondary">Recovery Index</div>
        </Card>

        <Card className="text-center">
          <FiTrendingUp className="w-12 h-12 text-creativePurple mx-auto mb-3" />
          <div className="text-3xl font-bold text-textPrimary mb-1">
            {longevityScore?.toFixed(0) || 0}
          </div>
          <div className="text-sm text-textSecondary">Longevity Score</div>
        </Card>
      </div>

      <Card>
        <h2 className="text-xl font-bold text-textPrimary mb-4">Today's Progress</h2>
        <div className="space-y-4">
          <div>
            <div className="flex justify-between text-sm text-textSecondary mb-2">
              <span>Compliance Rate</span>
              <span>{compliance?.toFixed(0) || 0}%</span>
            </div>
            <ProgressBar value={compliance || 0} color="green" showLabel={false} />
          </div>

          <div className="grid grid-cols-3 gap-4 mt-6">
            {progressData?.today_sessions?.map((session: any) => (
              <div
                key={session.id}
                className={`p-4 rounded-lg text-center ${
                  session.status === 'completed'
                    ? 'bg-successGreen/20 border border-successGreen'
                    : session.status === 'in_progress'
                    ? 'bg-neuralBlue/20 border border-neuralBlue'
                    : 'bg-surface-elevated border border-textMuted'
                }`}
              >
                <div className="text-xs text-textSecondary mb-1 capitalize">
                  {session.slot}
                </div>
                <div className="text-sm font-medium text-textPrimary capitalize">
                  {session.status}
                </div>
              </div>
            ))}
          </div>
        </div>
      </Card>

      <div className="flex gap-4">
        <Button variant="primary" size="lg" className="flex-1">
          <FiTarget className="w-5 h-5 mr-2" />
          Start Session
        </Button>
        <Button variant="secondary" size="lg" className="flex-1">
          View Exercises
        </Button>
      </div>
    </div>
  );
}
