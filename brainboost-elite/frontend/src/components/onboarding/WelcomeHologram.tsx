'use client';

import { motion } from 'framer-motion';
import { FiBrain } from 'react-icons/fi';

interface WelcomeHologramProps {
  onGetStarted: () => void;
}

export default function WelcomeHologram({ onGetStarted }: WelcomeHologramProps) {
  return (
    <div className="relative min-h-screen flex flex-col items-center justify-center p-6 overflow-hidden">
      <div className="absolute inset-0 bg-gradient-to-b from-neuralBlue/30 via-creativePurple/20 to-transparent pointer-events-none" />
      
      <motion.div
        className="relative z-10 flex flex-col items-center"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 0.8 }}
      >
        <motion.div
          className="mb-8"
          animate={{
            scale: [1, 1.05, 1],
            filter: [
              'drop-shadow(0 0 20px rgba(0, 123, 255, 0.5))',
              'drop-shadow(0 0 40px rgba(0, 123, 255, 0.8))',
              'drop-shadow(0 0 20px rgba(0, 123, 255, 0.5))',
            ],
          }}
          transition={{
            duration: 2,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        >
          <FiBrain className="w-48 h-48 text-neuralBlue" />
        </motion.div>

        <motion.h1
          className="text-4xl font-bold text-white text-center mb-4"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5, duration: 0.6 }}
        >
          Unlock Your Super Brain
        </motion.h1>

        <motion.p
          className="text-lg text-textSecondary text-center mb-12 max-w-md"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.8, duration: 0.6 }}
        >
          36 weeks to genius-level cognitive performance
        </motion.p>

        <motion.button
          onClick={onGetStarted}
          className="w-full max-w-sm h-14 bg-neuralBlue text-white font-bold rounded-2xl shadow-glow-blue"
          whileHover={{ scale: 1.02, boxShadow: '0 0 30px rgba(0, 123, 255, 0.6)' }}
          whileTap={{ scale: 0.95 }}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 1.1, duration: 0.6 }}
        >
          Get Started
        </motion.button>
      </motion.div>
    </div>
  );
}
