import { create } from 'zustand';
import { ExerciseType } from '@/types/exercise';

interface ExerciseState {
  activeExercise: {
    type: ExerciseType;
    name: string;
    sessionId: string;
  } | null;
  timerState: {
    isRunning: boolean;
    elapsed: number;
    total: number;
  };
  isSessionActive: boolean;
  startExercise: (
    type: ExerciseType,
    name: string,
    sessionId: string,
    duration: number
  ) => void;
  completeExercise: () => void;
  pauseTimer: () => void;
  resumeTimer: () => void;
  updateTimer: (elapsed: number) => void;
}

export const exerciseStore = create<ExerciseState>((set) => ({
  activeExercise: null,
  timerState: {
    isRunning: false,
    elapsed: 0,
    total: 0,
  },
  isSessionActive: false,
  startExercise: (type, name, sessionId, duration) =>
    set({
      activeExercise: { type, name, sessionId },
      timerState: {
        isRunning: true,
        elapsed: 0,
        total: duration,
      },
      isSessionActive: true,
    }),
  completeExercise: () =>
    set({
      activeExercise: null,
      timerState: {
        isRunning: false,
        elapsed: 0,
        total: 0,
      },
      isSessionActive: false,
    }),
  pauseTimer: () =>
    set((state) => ({
      timerState: {
        ...state.timerState,
        isRunning: false,
      },
    })),
  resumeTimer: () =>
    set((state) => ({
      timerState: {
        ...state.timerState,
        isRunning: true,
      },
    })),
  updateTimer: (elapsed) =>
    set((state) => ({
      timerState: {
        ...state.timerState,
        elapsed,
      },
    })),
}));
