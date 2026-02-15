import { create } from 'zustand';
import { DailyMetrics } from '@/types/metrics';

interface MetricsState {
  brainScore: number;
  stressIndex: number;
  longevityScore: number;
  trendData: DailyMetrics[];
  setMetrics: (metrics: DailyMetrics) => void;
  setTrendData: (data: DailyMetrics[]) => void;
}

export const metricsStore = create<MetricsState>((set) => ({
  brainScore: 0,
  stressIndex: 0,
  longevityScore: 0,
  trendData: [],
  setMetrics: (metrics) =>
    set({
      brainScore: metrics.brain_score || 0,
      stressIndex: metrics.stress_index || 0,
      longevityScore: metrics.longevity_score || 0,
    }),
  setTrendData: (data) => set({ trendData: data }),
}));
