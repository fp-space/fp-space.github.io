/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.rs','./styles/**/*.{css, scss}'],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}

