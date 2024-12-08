/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.rs', './node_modules/yew-styles/**/*.css'],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}

