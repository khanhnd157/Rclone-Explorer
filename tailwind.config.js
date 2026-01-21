export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        dark: {
          bg: '#1e1e1e',
          panel: '#252526',
          border: '#3e3e42',
          hover: '#2a2d2e',
          text: '#cccccc',
          'text-dim': '#858585',
          accent: '#0e639c'
        }
      }
    }
  },
  plugins: []
};
