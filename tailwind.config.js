/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
  theme: {
    extend: {
      borderRadius: {
        '4xl': '2rem',
        '5xl': '2.5rem',
      },
      fontFamily: {
        sans: ['"Inter var"', 'Inter', 'system-ui', 'sans-serif'],
        mono: ['"JetBrains Mono"', 'monospace'],
      },
      colors: {
        brand: {
          50: '#f2f4f9',
          100: '#dbe1ed',
          200: '#bcc7da',
          300: '#92a2c1',
          400: '#667ea6',
          500: '#3d5a89',
          600: '#324b72',
          700: '#293e5d',
          800: '#203149',
          900: '#192739',
        },
        accent: {
          50: '#fff9eb',
          100: '#ffefc3',
          200: '#ffe08a',
          300: '#ffd056',
          400: '#f6b713',
          500: '#d99c05',
          600: '#b68004',
          700: '#8f6303',
          800: '#6a4902',
          900: '#493201',
        },
      },
      boxShadow: {
        card: '0 18px 40px -25px rgba(24, 40, 74, 0.35)',
      },
    },
  },
  plugins: [require('@tailwindcss/forms'), require('@tailwindcss/typography')],
}
