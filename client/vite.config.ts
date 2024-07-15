import { defineConfig } from "vite";
import { VitePWA as pwa } from "vite-plugin-pwa";
import top_level_await from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), top_level_await(), pwa({ registerType: "autoUpdate" })],
  server: { port: 3000 },
});
