import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    allowedHosts: true,
    host: '127.0.0.1',
    port: 3000,
  },
  base: 'absproxy/3000',
  build: {
    target: 'esnext',
  },
});
