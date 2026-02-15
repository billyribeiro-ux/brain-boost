'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { authStore } from '@/stores/authStore';

export default function HomePage() {
  const router = useRouter();
  const isAuthenticated = authStore((state) => state.isAuthenticated);

  useEffect(() => {
    if (isAuthenticated) {
      router.push('/dashboard');
    } else {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  return (
    <div className="flex items-center justify-center min-h-screen bg-surface">
      <div className="text-center">
        <h1 className="text-4xl font-bold text-neuralBlue mb-4">BrainBoost Elite</h1>
        <p className="text-textSecondary">Loading...</p>
      </div>
    </div>
  );
}
