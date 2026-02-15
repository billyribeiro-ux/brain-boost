'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { authStore } from '@/stores/authStore';
import BottomNav from '@/components/layout/BottomNav';
import NeuralBackground from '@/components/layout/NeuralBackground';

export default function AppLayout({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const isAuthenticated = authStore((state) => state.isAuthenticated);

  useEffect(() => {
    if (!isAuthenticated) {
      router.push('/login');
    }
  }, [isAuthenticated, router]);

  if (!isAuthenticated) {
    return null;
  }

  return (
    <div className="min-h-screen bg-surface pb-20">
      <NeuralBackground />
      <main className="container mx-auto px-4 py-6 max-w-screen-xl">
        {children}
      </main>
      <BottomNav />
    </div>
  );
}
