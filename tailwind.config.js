/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,ts,jsx,tsx,html,svelte}",
  ],
  theme: {
    extend: {
      colors: {
        dark: {
          bg: "#070708",
          border: "#25252b"
        },
        primary: "#affd1d"
      }
    },
  },
  plugins: [],
}

