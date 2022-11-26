/** @type {import('tailwindcss').Config} */

const colors = require('tailwindcss/colors');

module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,svelte}",
  ],
  theme: {
      colors: {
          white: colors.white,
          primary: "#212529",
      }
  },
  plugins: [],
}
