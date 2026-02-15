import { create } from 'zustand';
import { UserLevel, DailySession } from '@/types/progress';

interface ProgressState {
  currentLevel: UserLevel | null;
  currentWeek: number;
  currentDay: number;
  compliance: number;
  streak: number;
  todaySessions: DailySession[];
  setProgress: (
    level: UserLevel,
    sessions: DailySession[],
    streak: number
  ) => void;
  updateSession: (session: DailySession) => void;
  advanceDay: () => void;
}

export const progressStore = create<ProgressState>((set) => ({
  currentLevel: null,
  currentWeek: 1,
  currentDay: 1,
  compliance: 0,
  streak: 0,
  todaySessions: [],
  setProgress: (level, sessions, streak) =>
    set({
      currentLevel: level,
      currentWeek: level.current_week,
      currentDay: level.current_day,
      compliance: level.compliance_rate,
      streak,
      todaySessions: sessions,
    }),
  updateSession: (session) =>
    set((state) => ({
      todaySessions: state.todaySessions.map((s) =>
        s.id === session.id ? session : s
      ),
    })),
  advanceDay: () =>
    set((state) => ({
      currentDay: state.currentDay + 1,
    })),
}));
