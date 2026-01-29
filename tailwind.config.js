/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/**/*.html"],
  darkMode: 'class',
  theme: {
    extend: {
      fontFamily: {
        'serif': ['DM Serif Display', 'Georgia', 'serif'],
        'sans': ['DM Sans', 'system-ui', 'sans-serif'],
        'mono': ['JetBrains Mono', 'monospace'],
      },
      colors: {
        'cream': '#faf9f7',
        'charcoal': '#1a1a1a',
        'accent': {
          'gold': '#c9a227',
          'emerald': '#10b981',
          'sapphire': '#3b82f6',
        }
      },
      animation: {
        'fade-in-up': 'fade-in-up 0.6s ease-out forwards',
        'fade-in': 'fade-in 0.4s ease-out forwards',
        'slide-in-left': 'slide-in-left 0.5s ease-out forwards',
        'ticker': 'ticker 0.1s steps(1) infinite',
        'pulse-subtle': 'pulse-subtle 2s ease-in-out infinite',
        'rotate-toggle': 'rotate-toggle 0.4s ease-out forwards',
      }
    },
  },
  plugins: [],
}
