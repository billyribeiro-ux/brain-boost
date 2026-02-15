export type ExerciseType =
  | 'meditation'
  | 'body_scan'
  | 'nsdr'
  | 'visualization'
  | 'qigong'
  | 'diet_log'
  | 'sleep_prep'
  | 'sleep_log'
  | 'retrieval_practice'
  | 'spaced_repetition'
  | 'interleaving'
  | 'feynman_technique'
  | 'reading'
  | 'speed_drill'
  | 'puzzle'
  | 'dual_nback'
  | 'bias_busting'
  | 'exercise_light'
  | 'exercise_brisk'
  | 'hiit'
  | 'cold_exposure'
  | 'new_skill'
  | 'creative_skill'
  | 'cbt_reframe'
  | 'journaling'
  | 'distraction_setup'
  | 'trading_sim'
  | 'rl_trading'
  | 'custom_combo';

export interface ExerciseCompletion {
  id: string;
  session_id: string;
  user_id: string;
  exercise_type: ExerciseType;
  exercise_name: string;
  duration_seconds: number;
  accuracy_score?: number;
  focus_rating?: number;
  difficulty_level?: number;
  metadata: Record<string, any>;
  completed_at: string;
  created_at: string;
}

export interface CompleteExerciseRequest {
  session_id: string;
  exercise_type: ExerciseType;
  exercise_name: string;
  duration_seconds: number;
  accuracy_score?: number;
  focus_rating?: number;
  difficulty_level?: number;
  metadata?: Record<string, any>;
}
