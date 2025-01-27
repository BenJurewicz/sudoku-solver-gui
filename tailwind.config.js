module.exports = {
  content: [
    // Include all Rust, HTML, and CSS files in the src directory
    './src/**/*.{rs,html,css}',
    // Include all HTML files in the output (dist) directory
    './dist/**/*.html',
  ],
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
