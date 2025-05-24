import { defineConfig } from 'tsup'

export default defineConfig({
  entry: ['js/index.ts'],
  format: ['esm'],
  outDir: 'pkg',
  dts: true,
  clean: false,
  target: 'es2020',
  external: ['block-parser-wasm.js']
})
