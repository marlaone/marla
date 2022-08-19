/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "../**/*.html",
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: [
      {
        marlaone: {
          "primary": "#F7A614",
          "secondary": "#E72380",
          "accent": "#333333",
          "neutral": "#333333",
          "base-100": "#FFFFFF",
          "info": "#3ABFF8",
          "success": "#36D399",
          "warning": "#FBBD23",
          "error": "#F87272",
        },
      },
    ],
  },
  plugins: [require('@tailwindcss/typography'), require('daisyui')],
}
