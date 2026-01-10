/** @type {import('tailwindcss').Config} */
const { addDynamicIconSelectors } = require('@iconify/tailwind')

module.exports = {
  content: [
    './src/**/*.{js,ts,jsx,tsx,mdx}',
    './node_modules/flyonui/dist/js/*.js',
  ],
  theme: {
    extend: {
      // Miyukini Framework custom tokens
      colors: {
        miyukini: {
          background: '#ffffff',
          'text-primary': '#1f2937',
          'text-secondary': '#6b7280',
          accent: '#6366f1',
          border: 'rgba(0, 0, 0, 0.1)',
          card: 'rgba(255, 255, 255, 0.9)',
        },
      },
      spacing: {
        'header': '64px',
        'bottom-nav': '70px',
        'touch': '48px',
      },
      zIndex: {
        'header': '20',
        'modal': '40',
        'bottom-nav': '50',
        'toast': '60',
      },
    },
  },
  plugins: [
    require('flyonui'),
    addDynamicIconSelectors(),
  ],
  flyonui: {
    themes: [
      'light', // Default theme - clair
      'dark',
      {
        miyukini: {
          'primary': '#6366f1',
          'secondary': '#818cf8',
          'accent': '#5eead4',
          'neutral': '#374151',
          'base-100': '#ffffff',
          'base-200': '#f9fafb',
          'base-300': '#e5e7eb',
          'base-content': '#1f2937',
          'info': '#3b82f6',
          'success': '#22c55e',
          'warning': '#f59e0b',
          'error': '#ef4444',
        },
      },
    ],
  },
}
