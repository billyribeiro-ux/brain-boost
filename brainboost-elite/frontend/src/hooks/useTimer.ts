import { useState, useEffect, useCallback, useRef } from 'react';
import { useHaptic } from './useHaptic';

interface UseTimerReturn {
  timeRemaining: number;
  isRunning: boolean;
  isPaused: boolean;
  progress: number;
  formattedTime: string;
  startTimer: () => void;
  pauseTimer: () => void;
  resumeTimer: () => void;
  resetTimer: () => void;
}

export function useTimer(
  durationSeconds: number,
  onComplete?: () => void
): UseTimerReturn {
  const [timeRemaining, setTimeRemaining] = useState(durationSeconds);
  const [isRunning, setIsRunning] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const startTimeRef = useRef<number | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  const { vibrate } = useHaptic();
  const halfwayNotifiedRef = useRef(false);

  const progress = 1 - timeRemaining / durationSeconds;
  
  const formatTime = (seconds: number): string => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const formattedTime = formatTime(timeRemaining);

  const tick = useCallback(() => {
    if (!startTimeRef.current) return;

    const now = Date.now();
    const elapsed = (now - startTimeRef.current) / 1000;
    const remaining = Math.max(0, durationSeconds - elapsed);

    setTimeRemaining(remaining);

    if (!halfwayNotifiedRef.current && remaining <= durationSeconds / 2) {
      vibrate('short');
      halfwayNotifiedRef.current = true;
    }

    if (remaining <= 0) {
      setIsRunning(false);
      setIsPaused(false);
      vibrate('success');
      if (onComplete) {
        onComplete();
      }
      return;
    }

    animationFrameRef.current = requestAnimationFrame(tick);
  }, [durationSeconds, onComplete, vibrate]);

  const startTimer = useCallback(() => {
    if (isRunning) return;
    
    startTimeRef.current = Date.now();
    halfwayNotifiedRef.current = false;
    setIsRunning(true);
    setIsPaused(false);
    animationFrameRef.current = requestAnimationFrame(tick);
  }, [isRunning, tick]);

  const pauseTimer = useCallback(() => {
    if (!isRunning || isPaused) return;
    
    setIsPaused(true);
    setIsRunning(false);
    
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
      animationFrameRef.current = null;
    }
  }, [isRunning, isPaused]);

  const resumeTimer = useCallback(() => {
    if (!isPaused) return;
    
    startTimeRef.current = Date.now() - (durationSeconds - timeRemaining) * 1000;
    setIsRunning(true);
    setIsPaused(false);
    animationFrameRef.current = requestAnimationFrame(tick);
  }, [isPaused, durationSeconds, timeRemaining, tick]);

  const resetTimer = useCallback(() => {
    setIsRunning(false);
    setIsPaused(false);
    setTimeRemaining(durationSeconds);
    startTimeRef.current = null;
    halfwayNotifiedRef.current = false;
    
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
      animationFrameRef.current = null;
    }
  }, [durationSeconds]);

  useEffect(() => {
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, []);

  return {
    timeRemaining,
    isRunning,
    isPaused,
    progress,
    formattedTime,
    startTimer,
    pauseTimer,
    resumeTimer,
    resetTimer,
  };
}
