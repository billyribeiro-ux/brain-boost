import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { progressStore } from '@/stores/progressStore';

export function useCurrentProgress() {
  return useQuery({
    queryKey: ['currentProgress'],
    queryFn: async () => {
      const response = await api.progress.getCurrent();
      const { current_level, today_sessions } = response.data;
      
      progressStore.getState().setProgress(
        current_level,
        today_sessions,
        current_level.streak_days || 0
      );
      
      return response.data;
    },
    refetchInterval: 30000,
    staleTime: 20000,
  });
}

export function useLevels() {
  return useQuery({
    queryKey: ['levels'],
    queryFn: async () => {
      const response = await api.progress.getLevels();
      return response.data;
    },
    staleTime: 5 * 60 * 1000,
  });
}

export function useAdvanceDay() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => api.progress.advanceDay(),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['currentProgress'] });
      queryClient.invalidateQueries({ queryKey: ['levels'] });
      queryClient.invalidateQueries({ queryKey: ['todayExercises'] });
      queryClient.invalidateQueries({ queryKey: ['todayMetrics'] });
    },
  });
}
