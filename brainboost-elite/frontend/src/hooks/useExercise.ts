import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { exerciseStore } from '@/stores/exerciseStore';

export function useTodayPlan() {
  return useQuery({
    queryKey: ['todayExercises'],
    queryFn: async () => {
      const response = await api.exercises.getToday();
      exerciseStore.getState().setTodayPlan(response.data.sessions);
      return response.data;
    },
    staleTime: 60000,
  });
}

export function useCompleteExercise() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ sessionId, data }: { sessionId: string; data: any }) =>
      api.exercises.complete(sessionId, data),
    onMutate: async ({ sessionId, data }) => {
      await queryClient.cancelQueries({ queryKey: ['todayExercises'] });
      
      const previousData = queryClient.getQueryData(['todayExercises']);
      
      queryClient.setQueryData(['todayExercises'], (old: any) => {
        if (!old) return old;
        
        return {
          ...old,
          sessions: old.sessions.map((session: any) =>
            session.session_id === sessionId
              ? { ...session, completed_exercises: (session.completed_exercises || 0) + 1 }
              : session
          ),
        };
      });
      
      return { previousData };
    },
    onError: (err, variables, context) => {
      if (context?.previousData) {
        queryClient.setQueryData(['todayExercises'], context.previousData);
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['todayExercises'] });
      queryClient.invalidateQueries({ queryKey: ['currentProgress'] });
      queryClient.invalidateQueries({ queryKey: ['todayMetrics'] });
      queryClient.invalidateQueries({ queryKey: ['exerciseHistory'] });
    },
  });
}

export function useExerciseHistory(days: number = 30) {
  return useQuery({
    queryKey: ['exerciseHistory', days],
    queryFn: async () => {
      const response = await api.exercises.getHistory(days);
      return response.data;
    },
    staleTime: 2 * 60 * 1000,
  });
}
