'use client';

import { motion } from 'framer-motion';
import { useEffect, useState } from 'react';

export default function NeuralBackground() {
  const [flashingLines, setFlashingLines] = useState<number[]>([]);

  const lines = Array.from({ length: 20 }, (_, i) => ({
    id: i,
    x1: Math.random() * 100,
    y1: Math.random() * 100,
    length: 20 + Math.random() * 30,
    angle: 45 + (Math.random() * 90 - 45),
    opacity: 0.03 + Math.random() * 0.05,
    duration: 3 + Math.random() * 5,
  }));

  useEffect(() => {
    const interval = setInterval(() => {
      lines.forEach((line) => {
        if (Math.random() > 0.95) {
          setFlashingLines((prev) => [...prev, line.id]);
          setTimeout(() => {
            setFlashingLines((prev) => prev.filter((id) => id !== line.id));
          }, 1000);
        }
      });
    }, 2000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="fixed inset-0 z-0 pointer-events-none overflow-hidden">
      <svg className="absolute inset-0 w-full h-full">
        {lines.map((line) => {
          const x2 = line.x1 + Math.cos((line.angle * Math.PI) / 180) * line.length;
          const y2 = line.y1 + Math.sin((line.angle * Math.PI) / 180) * line.length;
          const isFlashing = flashingLines.includes(line.id);

          return (
            <motion.line
              key={line.id}
              x1={`${line.x1}%`}
              y1={`${line.y1}%`}
              x2={`${x2}%`}
              y2={`${y2}%`}
              stroke={isFlashing ? '#007BFF' : '#1E1E1E'}
              strokeWidth="1"
              animate={{
                opacity: isFlashing ? [line.opacity, 0.3, line.opacity] : [line.opacity, line.opacity + 0.05, line.opacity],
              }}
              transition={{
                duration: isFlashing ? 1 : line.duration,
                repeat: isFlashing ? 0 : Infinity,
                ease: 'easeInOut',
              }}
            />
          );
        })}
      </svg>
    </div>
  );
}
