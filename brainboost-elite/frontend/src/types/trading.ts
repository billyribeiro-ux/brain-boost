export interface TradingSession {
  id: string;
  scenario_type: string;
  starting_balance: number;
  ending_balance?: number;
  total_trades: number;
  winning_trades: number;
  expected_value_score?: number;
  process_score?: number;
  started_at: string;
  completed_at?: string;
}

export interface TradingDecision {
  id: string;
  trading_session_id: string;
  decision_type: 'buy' | 'sell' | 'hold';
  asset_symbol: string;
  price_at_decision: number;
  quantity?: number;
  reasoning?: string;
  ai_feedback?: string;
  bias_detected?: string[];
  was_optimal?: boolean;
  pnl?: number;
  decision_time_ms?: number;
  created_at: string;
}

export interface StartTradingRequest {
  scenario_type: string;
  starting_balance?: number;
}

export interface TradingStats {
  total_sessions: number;
  total_trades: number;
  win_rate: number;
  avg_expected_value: number;
}
