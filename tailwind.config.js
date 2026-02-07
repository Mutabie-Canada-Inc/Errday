/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: 'class',
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        space: {
          900: '#0B0D17', // Deep Space (Background)
          800: '#15192b', // Space Panel (Cards)
          700: '#232942', // Orbital (Borders/Inactive)
        },
        neon: {
          cyan: '#00f0ff', // Primary Action
          amber: '#ffaa00', // Warning/Attention
          green: '#00ff9d', // Success
          pink: '#ff00d4', // Urgent
        }
      },
      fontFamily: {
        mono: ['"JetBrains Mono"', '"Fira Code"', 'monospace'],
        sans: ['"Inter"', 'system-ui', 'sans-serif'],
      },
      backdropBlur: {
        xs: '2px',
      }
    },
  },
  plugins: [],
};
