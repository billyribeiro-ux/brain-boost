import clsx, { ClassValue } from 'clsx';

export function cn(...inputs: ClassValue[]) {
  return clsx(inputs);
}

export function formatDuration(seconds: number): string {
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
}

export function formatDate(date: string): string {
  return new Date(date).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric',
  });
}

export function calculateBrainScore(
  completionRate: number,
  accuracy: number,
  focus: number,
  stressIndex: number
): number {
  return (
    completionRate * 0.4 +
    accuracy * 0.3 +
    focus * 0.2 +
    (100 - stressIndex) * 0.1
  );
}
