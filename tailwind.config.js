/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Inter var"', 'Inter', 'system-ui', 'sans-serif'],
        mono: ['"JetBrains Mono"', 'monospace'],
      },
      colors: {
        brand: {
          50: '#f2f7ff',
          100: '#e0ecff',
          200: '#bcd4ff',
          300: '#8ab4ff',
          400: '#5b92ff',
          500: '#326ffc',
          600: '#1d55d8',
          700: '#1642a8',
          800: '#123580',
          900: '#102f68',
        },
      },
    },
  },
  plugins: [require('@tailwindcss/forms'), require('@tailwindcss/typography')],
}
