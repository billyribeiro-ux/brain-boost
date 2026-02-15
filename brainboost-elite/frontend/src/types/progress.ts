export interface UserLevel {
  id: string;
  level_number: number;
  status: 'locked' | 'active' | 'completed';
  current_week: number;
  current_day: number;
  grace_skip_used: boolean;
  compliance_rate: number;
  started_at?: string;
  completed_at?: string;
}

export interface DailySession {
  id: string;
  level_number: number;
  week_number: number;
  day_number: number;
  day_date: string;
  slot: 'morning' | 'afternoon' | 'evening';
  status: 'pending' | 'in_progress' | 'completed' | 'skipped' | 'missed';
  scheduled_time?: string;
  started_at?: string;
  completed_at?: string;
  duration_planned_seconds: number;
  duration_actual_seconds?: number;
}

export interface CurrentProgress {
  current_level: UserLevel;
  today_sessions: DailySession[];
}
