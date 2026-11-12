import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import dotenv from 'dotenv';

dotenv.config({ path: '../.env' }); // Load env variables from the root folder

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/socket.io': {
        target: process.env.BACKEND_URL ?? 'http://localhost:3100',
        ws: true,
      },
    },
  },
});
