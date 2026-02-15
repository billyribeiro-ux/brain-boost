'use client';

import { motion } from 'framer-motion';
import { FaFire } from 'react-icons/fa';

interface StreakFlameProps {
  streak: number;
}

export default function StreakFlame({ streak }: StreakFlameProps) {
  const scale = Math.min(2, 1 + (streak * 0.1));
  const isMilestone = streak === 7 || streak === 30 || streak === 100;

  return (
    <motion.div
      className="relative inline-flex items-center justify-center"
      animate={{
        rotate: [-3, 3, -3],
        scale: [0.95, 1.05, 0.95],
      }}
      transition={{
        duration: 2,
        repeat: Infinity,
        ease: 'easeInOut',
      }}
    >
      <motion.div
        className="relative"
        style={{ transform: `scale(${scale})` }}
        animate={isMilestone ? {
          filter: [
            'drop-shadow(0 0 10px rgba(253, 126, 20, 0.8))',
            'drop-shadow(0 0 30px rgba(0, 123, 255, 1))',
            'drop-shadow(0 0 10px rgba(253, 126, 20, 0.8))',
          ],
        } : {}}
        transition={isMilestone ? { duration: 2, repeat: 3 } : {}}
      >
        <FaFire
          className={`w-6 h-6 ${isMilestone ? 'text-neuralBlue' : 'text-orangeAccent'}`}
          style={{
            filter: `drop-shadow(0 0 ${8 + streak * 0.5}px rgba(253, 126, 20, ${0.6 + Math.min(0.4, streak * 0.02)}))`,
          }}
        />
        
        <motion.div
          className="absolute -bottom-1 -right-1 bg-background rounded-full px-1.5 py-0.5 border border-orangeAccent"
          initial={{ scale: 0 }}
          animate={{ scale: 1 }}
          transition={{ type: 'spring', stiffness: 500, delay: 0.3 }}
        >
          <span className="text-xs font-bold text-white">{streak}</span>
        </motion.div>
      </motion.div>

      {isMilestone && (
        <motion.div
          className="absolute inset-0"
          initial={{ scale: 1, opacity: 1 }}
          animate={{ scale: 2, opacity: 0 }}
          transition={{ duration: 1, repeat: 2 }}
        >
          <div className="w-full h-full rounded-full bg-neuralBlue/30" />
        </motion.div>
      )}
    </motion.div>
  );
}
