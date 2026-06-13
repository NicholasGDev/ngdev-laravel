import { defineConfig } from "vite";

export default defineConfig({
  // Tauri espera a porta 1420 em modo dev
  server: {
    port: 1420,
    strictPort: true,
    // Não abre automaticamente — a webview do Tauri abre
    open: false,
  },
  // Evita polling de arquivos desnecessário
  clearScreen: false,
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // Tauri usa Chromium em Windows e WebKitGTK no Linux — ES2021 é seguro
    target: ["es2021", "chrome105", "safari15"],
    outDir: "dist",
    minify: process.env.TAURI_DEBUG ? false : "esbuild",
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
