export interface DailyMetrics {
  brain_score?: number;
  stress_index?: number;
  longevity_score?: number;
  completion_rate?: number;
  streak_days: number;
  metric_date: string;
}

export interface MetricsTrend {
  metrics: DailyMetrics[];
}
