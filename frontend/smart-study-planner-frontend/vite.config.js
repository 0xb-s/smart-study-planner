import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    proxy: {
      '/api': {
        target: 'put everything in your env here', 
        changeOrigin: true,
    
      },
    },
  },
});
