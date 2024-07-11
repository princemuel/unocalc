let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Calculator.prototype);
        obj.__wbg_ptr = ptr;
        CalculatorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

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
    /**
    * @returns {Calculator}
    */
    static new() {
        const ret = wasm.calculator_new();
        return Calculator.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    has_decimal() {
        const ret = wasm.calculator_has_decimal(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {number | undefined}
    */
    calculate() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.calculator_calculate(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r2 = getFloat64Memory0()[retptr / 8 + 1];
            return r0 === 0 ? undefined : r2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {Operation} operation
    */
    input_operation(operation) {
        wasm.calculator_input_operation(this.__wbg_ptr, operation);
    }
    /**
    * @param {number} digit
    */
    input_digit(digit) {
        wasm.calculator_input_digit(this.__wbg_ptr, digit);
    }
    /**
    */
    input_decimal() {
        wasm.calculator_input_decimal(this.__wbg_ptr);
    }
    /**
    */
    reset() {
        wasm.calculator_reset(this.__wbg_ptr);
    }
    /**
    */
    delete_last_digit() {
        wasm.calculator_delete_last_digit(this.__wbg_ptr);
    }
}

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_new_abda76e883ba8a5f() {
    const ret = new Error();
    return addHeapObject(ret);
};

export function __wbg_stack_658279fe44541cf6(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

export function __wbg_error_f851667af71bcfc6(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

