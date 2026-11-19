import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import svgLoader from "vite-svg-loader";
// @ts-expect-error type error without @types/node package
import process from "node:process";
import { execSync } from "node:child_process";
const host = process.env.TAURI_DEV_HOST;

const svgTypesPlugin = () => ({
  name: "generate-svg-types",
  buildStart() {
    execSync("deno --allow-read --allow-write build/generate-svg-types.ts", { stdio: "inherit" });
  },
  configureServer(server) { 
    server.watcher.on("all", (event, filePath) => {
      if (filePath.endsWith(".svg") && (event === "add" || event === "unlink")) {
        execSync("deno --allow-read --allow-write build/generate-svg-types.ts", { stdio: "inherit" });
      }
    });
  }
});

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [vue(), svgLoader(), svgTypesPlugin()],

  resolve: {
    alias: {
      "@": "/src"
    }
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
