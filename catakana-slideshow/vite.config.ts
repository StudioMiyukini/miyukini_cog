import { defineConfig } from 'vite';
import { viteSingleFile } from 'vite-plugin-singlefile';

export default defineConfig({
  base: './',
  plugins: [viteSingleFile()],
  build: {
    outDir: 'dist',
    // Inline tout dans un seul dist/index.html — fonctionne depuis file://
    assetsInlineLimit: 1_000_000,
    cssCodeSplit: false,
  },
});
