import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        neuralBlue: '#007BFF',
        creativePurple: '#6F42C1',
        successGreen: '#28A745',
        warningRed: '#DC3545',
        orangeAccent: '#FD7E14',
        surface: {
          DEFAULT: '#121212',
          card: '#1E1E1E',
          elevated: '#2A2A2A',
        },
        textPrimary: '#FFFFFF',
        textSecondary: '#B0B0B0',
        textMuted: '#6C757D',
      },
      boxShadow: {
        'glow-blue': '0 0 8px rgba(0, 123, 255, 0.5)',
        'glow-purple': '0 0 8px rgba(111, 66, 193, 0.5)',
        'glow-green': '0 0 8px rgba(40, 167, 69, 0.5)',
      },
      animation: {
        'pulse-glow': 'pulseGlow 2s ease-in-out infinite',
        'synaptic-fire': 'synapticFire 1.5s ease-in-out infinite',
        'streak-burn': 'streakBurn 0.8s ease-out',
        'shake-error': 'shakeError 0.5s ease-in-out',
      },
      keyframes: {
        pulseGlow: {
          '0%, 100%': { opacity: '0.5', transform: 'scale(1)' },
          '50%': { opacity: '1', transform: 'scale(1.05)' },
        },
        synapticFire: {
          '0%': { opacity: '0.3' },
          '50%': { opacity: '0.8' },
          '100%': { opacity: '0.3' },
        },
        streakBurn: {
          '0%': { transform: 'scale(0.8)', opacity: '0' },
          '50%': { transform: 'scale(1.2)', opacity: '1' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
        shakeError: {
          '0%, 100%': { transform: 'translateX(0)' },
          '25%': { transform: 'translateX(-10px)' },
          '75%': { transform: 'translateX(10px)' },
        },
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
}

export default config
