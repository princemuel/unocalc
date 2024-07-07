import * as wasm from "./lendycalc_bg.wasm";
import { __wbg_set_wasm } from "./lendycalc_bg.js";
__wbg_set_wasm(wasm);
export * from "./lendycalc_bg.js";
