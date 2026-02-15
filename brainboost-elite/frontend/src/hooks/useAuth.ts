import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useRouter } from 'next/navigation';
import { api } from '@/lib/api';
import { authStore } from '@/stores/authStore';

export function useLogin() {
  const router = useRouter();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (credentials: { email: string; password: string }) =>
      api.auth.login(credentials),
    onSuccess: (response) => {
      const { user, access_token, refresh_token } = response.data;
      authStore.getState().login(user, access_token, refresh_token);
      
      localStorage.setItem('access_token', access_token);
      localStorage.setItem('refresh_token', refresh_token);
      
      queryClient.invalidateQueries({ queryKey: ['currentUser'] });
      
      if (!user.chronotype || !user.primary_goal) {
        router.push('/onboarding');
      } else {
        router.push('/dashboard');
      }
    },
  });
}

export function useRegister() {
  const router = useRouter();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: { email: string; password: string; display_name: string }) =>
      api.auth.register(data),
    onSuccess: (response) => {
      const { user, access_token, refresh_token } = response.data;
      authStore.getState().login(user, access_token, refresh_token);
      
      localStorage.setItem('access_token', access_token);
      localStorage.setItem('refresh_token', refresh_token);
      
      queryClient.invalidateQueries({ queryKey: ['currentUser'] });
      router.push('/onboarding');
    },
  });
}

export function useLogout() {
  const router = useRouter();
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => api.auth.logout(),
    onSuccess: () => {
      authStore.getState().logout();
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      queryClient.clear();
      router.push('/login');
    },
    onError: () => {
      authStore.getState().logout();
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      queryClient.clear();
      router.push('/login');
    },
  });
}

export function useRefreshToken() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (refreshToken: string) =>
      api.auth.refresh({ refresh_token: refreshToken }),
    onSuccess: (response) => {
      const { user, access_token, refresh_token } = response.data;
      authStore.getState().login(user, access_token, refresh_token);
      
      localStorage.setItem('access_token', access_token);
      localStorage.setItem('refresh_token', refresh_token);
      
      queryClient.invalidateQueries({ queryKey: ['currentUser'] });
    },
  });
}

export function useCurrentUser() {
  const isAuthenticated = authStore((state) => state.isAuthenticated);

  return useQuery({
    queryKey: ['currentUser'],
    queryFn: async () => {
      const response = await api.users.getMe();
      authStore.getState().updateUser(response.data);
      return response.data;
    },
    enabled: isAuthenticated,
    staleTime: 5 * 60 * 1000,
  });
}

export function useUpdateProfile() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: any) => api.users.updateMe(data),
    onSuccess: (response) => {
      authStore.getState().updateUser(response.data);
      queryClient.invalidateQueries({ queryKey: ['currentUser'] });
    },
  });
}
