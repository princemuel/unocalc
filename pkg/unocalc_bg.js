let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
*/
export const Operation = Object.freeze({ Add:0,"0":"Add",Subtract:1,"1":"Subtract",Multiply:2,"2":"Multiply",Divide:3,"3":"Divide", });

const CalculatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_calculator_free(ptr >>> 0));
/**
*/
export class Calculator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CalculatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_calculator_free(ptr);
    }
    /**
    * @returns {number}
    */
    get current_value() {
        const ret = wasm.__wbg_get_calculator_current_value(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set current_value(arg0) {
        wasm.__wbg_set_calculator_current_value(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number | undefined}
    */
    get stored_value() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.__wbg_get_calculator_stored_value(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r2 = getFloat64Memory0()[retptr / 8 + 1];
            return r0 === 0 ? undefined : r2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number | undefined} [arg0]
    */
    set stored_value(arg0) {
        wasm.__wbg_set_calculator_stored_value(this.__wbg_ptr, !isLikeNone(arg0), isLikeNone(arg0) ? 0 : arg0);
    }
    /**
    * @returns {Operation | undefined}
    */
    get current_operation() {
        const ret = wasm.__wbg_get_calculator_current_operation(this.__wbg_ptr);
        return ret === 4 ? undefined : ret;
    }
    /**
    * @param {Operation | undefined} [arg0]
    */
    set current_operation(arg0) {
        wasm.__wbg_set_calculator_current_operation(this.__wbg_ptr, isLikeNone(arg0) ? 4 : arg0);
    }
    /**
    * @returns {boolean}
    */
    get has_decimal() {
        const ret = wasm.__wbg_get_calculator_has_decimal(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set has_decimal(arg0) {
        wasm.__wbg_set_calculator_has_decimal(this.__wbg_ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get decimal_place() {
        const ret = wasm.__wbg_get_calculator_decimal_place(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set decimal_place(arg0) {
        wasm.__wbg_set_calculator_decimal_place(this.__wbg_ptr, arg0);
    }
}

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

