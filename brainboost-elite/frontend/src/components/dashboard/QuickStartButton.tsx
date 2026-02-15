'use client';

import { motion } from 'framer-motion';
import { FiPlay, FiCheck } from 'react-icons/fi';

interface QuickStartButtonProps {
  allDone: boolean;
  onClick: () => void;
}

export default function QuickStartButton({ allDone, onClick }: QuickStartButtonProps) {
  return (
    <motion.button
      onClick={onClick}
      disabled={allDone}
      className={`fixed bottom-20 left-1/2 -translate-x-1/2 z-30 w-14 h-14 rounded-full flex items-center justify-center shadow-lg ${
        allDone ? 'bg-successGreen' : 'bg-gradient-radial from-neuralBlue to-neuralBlue/80'
      }`}
      style={{
        boxShadow: allDone
          ? '0 0 20px rgba(40, 167, 69, 0.6)'
          : '0 0 20px rgba(0, 123, 255, 0.6)',
      }}
      whileHover={{ scale: 1.1 }}
      whileTap={{ scale: 0.9, rotate: 360 }}
      animate={allDone ? {} : {
        scale: [1, 1.05, 1],
      }}
      transition={{
        scale: { duration: 2, repeat: Infinity, ease: 'easeInOut' },
        rotate: { duration: 0.5, ease: 'easeOut' },
      }}
    >
      {allDone ? (
        <FiCheck className="w-6 h-6 text-white" />
      ) : (
        <FiPlay className="w-6 h-6 text-white ml-1" />
      )}

      {allDone && (
        <motion.div
          className="absolute -top-12 left-1/2 -translate-x-1/2 bg-surface-card px-3 py-1 rounded-lg text-sm text-white whitespace-nowrap"
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
        >
          All Done!
          <div className="absolute bottom-0 left-1/2 -translate-x-1/2 translate-y-full w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-surface-card" />
        </motion.div>
      )}
    </motion.button>
  );
}
