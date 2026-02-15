import { useQuery } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { metricsStore } from '@/stores/metricsStore';

export function useTodayMetrics() {
  return useQuery({
    queryKey: ['todayMetrics'],
    queryFn: async () => {
      const response = await api.metrics.getToday();
      metricsStore.getState().setMetrics(response.data);
      return response.data;
    },
    refetchInterval: 60000,
    staleTime: 45000,
  });
}

export function useMetricsTrend(days: number = 30) {
  return useQuery({
    queryKey: ['metricsTrend', days],
    queryFn: async () => {
      const response = await api.metrics.getTrend(days);
      return response.data;
    },
    staleTime: 5 * 60 * 1000,
  });
}

export function useBrainScore() {
  return useQuery({
    queryKey: ['brainScore'],
    queryFn: async () => {
      const response = await api.metrics.getBrainScore();
      return response.data;
    },
    refetchInterval: 60000,
    staleTime: 45000,
  });
}
