import { useCallback } from 'react';

type VibrationPattern = 'short' | 'medium' | 'long' | 'success' | 'error';

const patterns: Record<VibrationPattern, number | number[]> = {
  short: 50,
  medium: 100,
  long: 200,
  success: [100, 50, 100],
  error: [50, 100, 50, 100, 50],
};

export function useHaptic() {
  const vibrate = useCallback((pattern: VibrationPattern) => {
    if (typeof window === 'undefined') return;
    
    if (!('vibrate' in navigator)) {
      return;
    }

    try {
      const vibrationPattern = patterns[pattern];
      navigator.vibrate(vibrationPattern);
    } catch (error) {
      console.warn('Vibration API not supported or failed:', error);
    }
  }, []);

  return { vibrate };
}
