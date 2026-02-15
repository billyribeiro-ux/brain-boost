'use client';

import { motion } from 'framer-motion';
import { FiSunrise, FiSun, FiMoon } from 'react-icons/fi';
import { useState } from 'react';

interface ChronotypeSliderProps {
  onSelect: (data: { chronotype: string; morning_time: string; afternoon_time: string; evening_time: string }) => void;
}

const chronotypes = [
  { id: 'morning', label: 'Early Bird', icon: FiSunrise, color: 'orange', morning: '06:30', afternoon: '13:00', evening: '18:00' },
  { id: 'flexible', label: 'Flexible', icon: FiSun, color: 'blue', morning: '08:00', afternoon: '14:00', evening: '19:00' },
  { id: 'evening', label: 'Night Owl', icon: FiMoon, color: 'purple', morning: '09:30', afternoon: '15:00', evening: '20:30' },
];

export default function ChronotypeSlider({ onSelect }: ChronotypeSliderProps) {
  const [selected, setSelected] = useState<string | null>(null);
  const [morningTime, setMorningTime] = useState('08:00');
  const [afternoonTime, setAfternoonTime] = useState('14:00');
  const [eveningTime, setEveningTime] = useState('19:00');

  const handleSelect = (type: typeof chronotypes[0]) => {
    setSelected(type.id);
    setMorningTime(type.morning);
    setAfternoonTime(type.afternoon);
    setEveningTime(type.evening);
    onSelect({
      chronotype: type.id,
      morning_time: type.morning,
      afternoon_time: type.afternoon,
      evening_time: type.evening,
    });
  };

  const adjustTime = (timeType: 'morning' | 'afternoon' | 'evening', delta: number) => {
    const timeMap = { morning: morningTime, afternoon: afternoonTime, evening: eveningTime };
    const setterMap = { morning: setMorningTime, afternoon: setAfternoonTime, evening: setEveningTime };
    
    const [hours, minutes] = timeMap[timeType].split(':').map(Number);
    const totalMinutes = hours * 60 + minutes + delta;
    const newHours = Math.floor(totalMinutes / 60) % 24;
    const newMinutes = totalMinutes % 60;
    const newTime = `${String(newHours).padStart(2, '0')}:${String(newMinutes).padStart(2, '0')}`;
    
    setterMap[timeType](newTime);
  };

  return (
    <div className="flex flex-col items-center p-6 min-h-screen justify-center">
      <h2 className="text-2xl font-bold text-white mb-8 text-center">
        When do you feel most energized?
      </h2>

      <div className="flex gap-4 mb-8 flex-wrap justify-center">
        {chronotypes.map((type) => {
          const Icon = type.icon;
          const isSelected = selected === type.id;
          
          return (
            <motion.button
              key={type.id}
              onClick={() => handleSelect(type)}
              className={`w-40 h-32 rounded-xl border-2 flex flex-col items-center justify-center gap-2 transition-all ${
                isSelected
                  ? 'border-neuralBlue bg-neuralBlue/10 shadow-glow-blue'
                  : 'border-gray-700 opacity-60 hover:opacity-100'
              }`}
              whileHover={{ scale: 1.05 }}
              whileTap={{ scale: 0.95 }}
              animate={isSelected ? { scale: 1.05 } : { scale: 1 }}
            >
              <Icon className={`w-12 h-12 text-${type.color}Accent`} />
              <span className="text-white font-medium">{type.label}</span>
            </motion.button>
          );
        })}
      </div>

      {selected && (
        <motion.div
          className="w-full max-w-md space-y-4"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
        >
          <div className="bg-surface-card rounded-xl p-4">
            <div className="flex items-center justify-between mb-2">
              <span className="text-textSecondary">Morning</span>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => adjustTime('morning', -30)}
                  className="w-8 h-8 rounded-lg bg-surface-elevated text-white hover:bg-neuralBlue transition-colors"
                >
                  -
                </button>
                <span className="text-white font-mono w-16 text-center">{morningTime}</span>
                <button
                  onClick={() => adjustTime('morning', 30)}
                  className="w-8 h-8 rounded-lg bg-surface-elevated text-white hover:bg-neuralBlue transition-colors"
                >
                  +
                </button>
              </div>
            </div>

            <div className="flex items-center justify-between mb-2">
              <span className="text-textSecondary">Afternoon</span>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => adjustTime('afternoon', -30)}
                  className="w-8 h-8 rounded-lg bg-surface-elevated text-white hover:bg-neuralBlue transition-colors"
                >
                  -
                </button>
                <span className="text-white font-mono w-16 text-center">{afternoonTime}</span>
                <button
                  onClick={() => adjustTime('afternoon', 30)}
                  className="w-8 h-8 rounded-lg bg-surface-elevated text-white hover:bg-neuralBlue transition-colors"
                >
                  +
                </button>
              </div>
            </div>

            <div className="flex items-center justify-between">
              <span className="text-textSecondary">Evening</span>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => adjustTime('evening', -30)}
                  className="w-8 h-8 rounded-lg bg-surface-elevated text-white hover:bg-neuralBlue transition-colors"
                >
                  -
                </button>
                <span className="text-white font-mono w-16 text-center">{eveningTime}</span>
                <button
                  onClick={() => adjustTime('evening', 30)}
                  className="w-8 h-8 rounded-lg bg-surface-elevated text-white hover:bg-neuralBlue transition-colors"
                >
                  +
                </button>
              </div>
            </div>
          </div>
        </motion.div>
      )}
    </div>
  );
}
