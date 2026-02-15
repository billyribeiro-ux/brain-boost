'use client';

import { motion } from 'framer-motion';
import { FiWifiOff, FiRefreshCw } from 'react-icons/fi';
import { useRouter } from 'next/navigation';

export default function OfflinePage() {
  const router = useRouter();

  const handleRetry = () => {
    router.refresh();
  };

  return (
    <div className="min-h-screen bg-background flex flex-col items-center justify-center p-6">
      <motion.div
        className="flex flex-col items-center max-w-md text-center"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
      >
        <div className="w-32 h-32 mb-8 rounded-full bg-surface-elevated flex items-center justify-center">
          <FiWifiOff className="w-16 h-16 text-textMuted" />
        </div>

        <h1 className="text-3xl font-bold text-white mb-4">
          You're Offline
        </h1>

        <p className="text-textSecondary mb-8">
          Your progress is saved locally. Reconnect to sync your data and continue your cognitive training journey.
        </p>

        <motion.button
          onClick={handleRetry}
          className="w-full h-14 bg-neuralBlue text-white font-bold rounded-2xl flex items-center justify-center gap-2 shadow-glow-blue"
          whileHover={{ scale: 1.02 }}
          whileTap={{ scale: 0.95 }}
        >
          <FiRefreshCw className="w-5 h-5" />
          Retry Connection
        </motion.button>

        <div className="mt-8 p-4 bg-surface-card rounded-xl">
          <h3 className="text-white font-bold mb-2">Offline Features</h3>
          <ul className="text-sm text-textSecondary space-y-1 text-left">
            <li>• View cached exercise plans</li>
            <li>• Review completed sessions</li>
            <li>• Access saved flashcards</li>
            <li>• View your progress history</li>
          </ul>
        </div>
      </motion.div>
    </div>
  );
}
