import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';

export default defineConfig({
  plugins: [solidPlugin()],
  server: {
    port: 3000,
  },
  build: {
    minify: 'esbuild',
    outDir: '../static/marla-elements',
    emptyOutDir: true,
    lib: {
      name: "marlaone",
      entry: 'src/index.ts',
      fileName: "marla-elements"
    },
  },
});
