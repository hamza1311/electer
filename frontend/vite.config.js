import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import sveltePreprocess from 'svelte-preprocess';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte({
    preprocess: sveltePreprocess()
  })],
  server: {
    proxy: {
      '/api/': 'http://localhost:8000'
    }
  },
  // build: {
  //   outDir: '../dist'
  // }
})
