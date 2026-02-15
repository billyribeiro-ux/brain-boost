'use client';

import { useState } from 'react';
import { motion, AnimatePresence, PanInfo } from 'framer-motion';
import WelcomeHologram from './WelcomeHologram';
import ChronotypeSlider from './ChronotypeSlider';
import GoalPicker from './GoalPicker';
import ExperienceLevel from './ExperienceLevel';
import OnboardingSummary from './OnboardingSummary';

interface OnboardingCarouselProps {
  onComplete: (data: OnboardingData) => void;
}

interface OnboardingData {
  chronotype: string;
  goals: string[];
  experienceLevel: string;
  schedule: {
    morning_time: string;
    afternoon_time: string;
    evening_time: string;
  };
}

const screens = ['welcome', 'chronotype', 'goals', 'experience', 'summary'];

export default function OnboardingCarousel({ onComplete }: OnboardingCarouselProps) {
  const [currentScreen, setCurrentScreen] = useState(0);
  const [direction, setDirection] = useState(0);
  const [data, setData] = useState<Partial<OnboardingData>>({
    goals: [],
    schedule: { morning_time: '08:00', afternoon_time: '14:00', evening_time: '19:00' },
  });

  const nextScreen = () => {
    if (currentScreen < screens.length - 1) {
      setDirection(1);
      setCurrentScreen(currentScreen + 1);
    }
  };

  const prevScreen = () => {
    if (currentScreen > 0) {
      setDirection(-1);
      setCurrentScreen(currentScreen - 1);
    }
  };

  const skipToEnd = () => {
    setDirection(1);
    setCurrentScreen(screens.length - 1);
  };

  const handleSwipe = (e: any, info: PanInfo) => {
    if (info.offset.x < -100) {
      nextScreen();
    } else if (info.offset.x > 100) {
      prevScreen();
    }
  };

  const variants = {
    enter: (direction: number) => ({
      x: direction > 0 ? 1000 : -1000,
      opacity: 0,
    }),
    center: {
      x: 0,
      opacity: 1,
    },
    exit: (direction: number) => ({
      x: direction < 0 ? 1000 : -1000,
      opacity: 0,
    }),
  };

  const renderScreen = () => {
    switch (screens[currentScreen]) {
      case 'welcome':
        return <WelcomeHologram onGetStarted={nextScreen} />;
      
      case 'chronotype':
        return (
          <ChronotypeSlider
            onSelect={(chronotypeData) => {
              setData({ ...data, chronotype: chronotypeData.chronotype, schedule: {
                morning_time: chronotypeData.morning_time,
                afternoon_time: chronotypeData.afternoon_time,
                evening_time: chronotypeData.evening_time,
              }});
            }}
          />
        );
      
      case 'goals':
        return (
          <GoalPicker
            onSelect={(goals) => {
              setData({ ...data, goals });
            }}
          />
        );
      
      case 'experience':
        return (
          <ExperienceLevel
            onSelect={(level) => {
              setData({ ...data, experienceLevel: level });
            }}
          />
        );
      
      case 'summary':
        return (
          <OnboardingSummary
            chronotype={data.chronotype || 'flexible'}
            goals={data.goals || []}
            experienceLevel={data.experienceLevel || 'beginner'}
            schedule={data.schedule!}
            onComplete={() => onComplete(data as OnboardingData)}
          />
        );
      
      default:
        return null;
    }
  };

  return (
    <div className="relative min-h-screen bg-background overflow-hidden">
      {currentScreen > 0 && currentScreen < screens.length - 1 && (
        <button
          onClick={skipToEnd}
          className="absolute top-6 right-6 z-50 text-textMuted hover:text-white transition-colors underline"
        >
          Skip
        </button>
      )}

      <AnimatePresence initial={false} custom={direction} mode="wait">
        <motion.div
          key={currentScreen}
          custom={direction}
          variants={variants}
          initial="enter"
          animate="center"
          exit="exit"
          transition={{
            x: { type: 'spring', stiffness: 300, damping: 30 },
            opacity: { duration: 0.2 },
          }}
          drag={currentScreen > 0 && currentScreen < screens.length - 1 ? 'x' : false}
          dragConstraints={{ left: 0, right: 0 }}
          dragElastic={0.2}
          onDragEnd={handleSwipe}
          className="absolute inset-0"
        >
          {renderScreen()}
        </motion.div>
      </AnimatePresence>

      {currentScreen > 0 && currentScreen < screens.length - 1 && (
        <div className="absolute bottom-24 left-0 right-0 flex justify-center items-center gap-2 z-40">
          {screens.slice(1, -1).map((_, index) => (
            <motion.div
              key={index}
              className={`w-2 h-2 rounded-full transition-all ${
                index + 1 === currentScreen
                  ? 'bg-neuralBlue w-8'
                  : 'bg-gray-600 ring-1 ring-gray-600'
              }`}
              initial={{ scale: 0 }}
              animate={{ scale: 1 }}
              transition={{ delay: index * 0.1 }}
            />
          ))}
        </div>
      )}

      {currentScreen > 0 && currentScreen < screens.length - 1 && (
        <motion.button
          onClick={nextScreen}
          className="absolute bottom-6 right-6 z-40 h-12 px-8 bg-neuralBlue text-white font-bold rounded-2xl shadow-glow-blue"
          whileHover={{ scale: 1.05 }}
          whileTap={{ scale: 0.95 }}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
        >
          Next
        </motion.button>
      )}
    </div>
  );
}
