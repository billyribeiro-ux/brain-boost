import axios, { AxiosInstance, AxiosError } from 'axios';
import { authStore } from '@/stores/authStore';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

const apiClient: AxiosInstance = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

apiClient.interceptors.request.use(
  (config) => {
    const token = authStore.getState().accessToken;
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    if (error.response?.status === 401) {
      authStore.getState().logout();
      if (typeof window !== 'undefined') {
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);

export default apiClient;

export const api = {
  auth: {
    register: (data: any) => apiClient.post('/auth/register', data),
    login: (data: any) => apiClient.post('/auth/login', data),
    logout: () => apiClient.post('/auth/logout'),
    refresh: () => apiClient.post('/auth/refresh'),
  },
  users: {
    getMe: () => apiClient.get('/users/me'),
    updateMe: (data: any) => apiClient.patch('/users/me', data),
    getSchedule: () => apiClient.get('/users/me/schedule'),
    updateSchedule: (data: any) => apiClient.put('/users/me/schedule', data),
  },
  progress: {
    getCurrent: () => apiClient.get('/progress/current'),
    getLevels: () => apiClient.get('/progress/levels'),
    advanceDay: () => apiClient.post('/progress/advance-day'),
  },
  exercises: {
    getToday: () => apiClient.get('/exercises/today'),
    complete: (data: any) => apiClient.post('/exercises/complete', data),
    getHistory: () => apiClient.get('/exercises/history'),
  },
  metrics: {
    getToday: () => apiClient.get('/metrics/today'),
    getTrend: (days: number) => apiClient.get(`/metrics/trend?days=${days}`),
    getBrainScore: () => apiClient.get('/metrics/brain-score'),
  },
  trading: {
    start: (data: any) => apiClient.post('/trading/start', data),
    decide: (data: any) => apiClient.post('/trading/decide', data),
    getHistory: () => apiClient.get('/trading/history'),
    getStats: () => apiClient.get('/trading/stats'),
  },
  coach: {
    sendMessage: (data: any) => apiClient.post('/coach/message', data),
    getSuggestions: () => apiClient.get('/coach/suggestions'),
    getHistory: () => apiClient.get('/coach/history'),
  },
};
