import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    allowedHosts: true,
    port: 3000,
  },
  build: {
    target: 'esnext',
  },
});
