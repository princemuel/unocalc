// A dependency graph that contains any wasm must all be imported
// asynchronously. This `main.js` file does the single async import, so
// that no one else needs to worry about it again.
import { inject } from "@vercel/analytics";
import "./assets/styles/global.css";
import("./assets/scripts/index.ts").catch((e) =>
  console.error("Error importing `index.ts`:", e)
);
inject();
