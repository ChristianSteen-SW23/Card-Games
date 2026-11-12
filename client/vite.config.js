import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import dotenv from 'dotenv';

dotenv.config({ path: '../.env' }); // Load env variables from the root folder

// Extra hostnames the server may be reached by, e.g. "example.com,www.example.com".
// Vite only allows localhost/IPs by default, to protect against DNS rebinding.
const allowedHosts = process.env.ALLOWED_HOSTS
  ? process.env.ALLOWED_HOSTS.split(',').map((h) => h.trim())
  : [];

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  preview: {
    allowedHosts,
  },
  server: {
    proxy: {
      '/socket.io': {
        target: process.env.BACKEND_URL ?? 'http://localhost:3100',
        ws: true,
      },
    },
  },
});
