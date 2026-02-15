'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Card from '@/components/ui/Card';
import Button from '@/components/ui/Button';
import { CHRONOTYPES, PRIMARY_GOALS } from '@/lib/constants';
import { FiBrain } from 'react-icons/fi';

export default function OnboardingPage() {
  const router = useRouter();
  const [chronotype, setChronotype] = useState('');
  const [goal, setGoal] = useState('');

  const handleComplete = () => {
    router.push('/dashboard');
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-surface p-4">
      <Card className="w-full max-w-2xl">
        <div className="text-center mb-8">
          <FiBrain className="w-16 h-16 text-neuralBlue mx-auto mb-4 animate-pulse-glow" />
          <h1 className="text-3xl font-bold text-textPrimary mb-2">Welcome to BrainBoost Elite</h1>
          <p className="text-textSecondary">Let's personalize your cognitive training journey</p>
        </div>

        <div className="space-y-6">
          <div>
            <h3 className="text-lg font-medium text-textPrimary mb-3">What's your chronotype?</h3>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
              {CHRONOTYPES.map((type) => (
                <button
                  key={type.value}
                  onClick={() => setChronotype(type.value)}
                  className={`p-4 rounded-xl border-2 transition-all ${
                    chronotype === type.value
                      ? 'border-neuralBlue bg-neuralBlue/10'
                      : 'border-surface-elevated hover:border-textMuted'
                  }`}
                >
                  <div className="text-textPrimary font-medium">{type.label}</div>
                </button>
              ))}
            </div>
          </div>

          <div>
            <h3 className="text-lg font-medium text-textPrimary mb-3">What's your primary goal?</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {PRIMARY_GOALS.map((g) => (
                <button
                  key={g.value}
                  onClick={() => setGoal(g.value)}
                  className={`p-4 rounded-xl border-2 transition-all ${
                    goal === g.value
                      ? 'border-creativePurple bg-creativePurple/10'
                      : 'border-surface-elevated hover:border-textMuted'
                  }`}
                >
                  <div className="text-textPrimary font-medium">{g.label}</div>
                </button>
              ))}
            </div>
          </div>

          <Button
            onClick={handleComplete}
            variant="primary"
            size="lg"
            className="w-full"
            disabled={!chronotype || !goal}
          >
            Start Your Journey
          </Button>
        </div>
      </Card>
    </div>
  );
}
