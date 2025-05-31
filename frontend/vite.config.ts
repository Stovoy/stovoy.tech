import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { fileURLToPath } from 'url';
import path from 'path';

export default defineConfig({
  plugins: [sveltekit()],
  resolve: {
    alias: {
      content: path.resolve(fileURLToPath(new URL('.', import.meta.url)), '../content')
    }
  },
  server: {
    fs: {
      allow: ['..']
    }
  }
});
