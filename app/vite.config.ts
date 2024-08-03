import { defineConfig } from "vite";
import { VitePWA as pwa, VitePWAOptions } from "vite-plugin-pwa";
import top_level_await from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
// import manifest from "./manifest.json"

const manifest = (function () {
  type ManifestPromise = Promise<VitePWAOptions["manifest"]>;
  try {
    return import("./manifest.json").then((r) => r.default) as ManifestPromise;
  } catch (error) {
    return {} as ManifestPromise;
  }
})();

export default defineConfig({
  server: { port: 3000 },
  plugins: [
    wasm(),
    top_level_await(),
    pwa({
      registerType: "autoUpdate",
      pwaAssets: { disabled: false, config: true, overrideManifestIcons: true },
      manifest: await manifest,
      devOptions: { enabled: true, suppressWarnings: true, type: "module" },
      workbox: {
        cleanupOutdatedCaches: true,
        clientsClaim: true,
        runtimeCaching: [
          {
            urlPattern: /^https:\/\/fonts\.googleapis\.com\/.*/i,
            handler: "CacheFirst",
            options: {
              cacheName: "google-fonts-cache",
              expiration: {
                maxEntries: 10,
                maxAgeSeconds: 60 * 60 * 24 * 365, // <== 365 days
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
          {
            urlPattern: /^https:\/\/fonts\.gstatic\.com\/.*/i,
            handler: "CacheFirst",
            options: {
              cacheName: "gstatic-fonts-cache",
              expiration: {
                maxEntries: 10,
                maxAgeSeconds: 60 * 60 * 24 * 365, // <== 365 days
              },
              cacheableResponse: {
                statuses: [0, 200],
              },
            },
          },
          {
            urlPattern: ({ request }) => request.destination === "image",
            handler: "StaleWhileRevalidate",
            options: {
              cacheName: "images-cache",
              expiration: {
                maxEntries: 10,
              },
            },
          },
        ],
      },
    }),
  ],
});
