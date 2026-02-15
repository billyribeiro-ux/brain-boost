'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { motion } from 'framer-motion';
import { FiGrid, FiLayers, FiMessageSquare, FiUsers, FiUser } from 'react-icons/fi';

const navItems = [
  { href: '/dashboard', icon: FiGrid, label: 'Dashboard' },
  { href: '/levels', icon: FiLayers, label: 'Levels' },
  { href: '/coach', icon: FiMessageSquare, label: 'Coach' },
  { href: '/community', icon: FiUsers, label: 'Community' },
  { href: '/profile', icon: FiUser, label: 'Profile' },
];

export default function BottomNav() {
  const pathname = usePathname();

  return (
    <nav className="fixed bottom-0 left-0 right-0 h-16 bg-surface-card border-t border-surface-elevated z-50">
      <div className="flex items-center justify-around h-full max-w-screen-xl mx-auto px-4">
        {navItems.map((item) => {
          const isActive = pathname === item.href;
          const Icon = item.icon;

          return (
            <Link key={item.href} href={item.href} className="relative flex flex-col items-center justify-center flex-1">
              <motion.div
                className="flex flex-col items-center"
                animate={{
                  scale: isActive ? 1.1 : 1,
                  color: isActive ? '#007BFF' : '#B0B0B0',
                }}
                whileTap={{ scale: 0.95 }}
              >
                <Icon className="w-6 h-6 mb-1" />
                <span className="text-xs">{item.label}</span>
              </motion.div>
              {isActive && (
                <motion.div
                  className="absolute -top-1 left-1/2 w-1 h-1 bg-neuralBlue rounded-full"
                  layoutId="activeIndicator"
                  initial={false}
                  style={{ x: '-50%' }}
                />
              )}
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
