export interface User {
  id: string;
  email: string;
  display_name: string;
  avatar_url?: string;
  subscription: 'free' | 'premium';
  chronotype?: string;
  primary_goal?: string;
  onboarding_completed: boolean;
  created_at: string;
}

export interface UserSchedule {
  id: string;
  user_id: string;
  morning_time: string;
  afternoon_time: string;
  evening_time: string;
  notification_enabled: boolean;
  haptic_enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateUserRequest {
  email: string;
  password: string;
  display_name: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  user: User;
  access_token: string;
  refresh_token: string;
  expires_at: number;
}

export interface UpdateProfileRequest {
  display_name?: string;
  avatar_url?: string;
  chronotype?: string;
  primary_goal?: string;
  timezone?: string;
}
