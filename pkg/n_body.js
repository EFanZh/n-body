const __exports = {};

let wasm;

/**
* @returns {void}
*/
export function start() {
    return wasm.start();
}

__exports.start = start;

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function __wbg_error_4bb6c2a97407129a(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);

    varg0 = varg0.slice();
    wasm.__wbindgen_free(arg0, arg1 * 1);

    console.error(varg0);
}

__exports.__wbg_error_4bb6c2a97407129a = __wbg_error_4bb6c2a97407129a;

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function __wbg_new_59cb74e423758ede() {
    return addHeapObject(new Error());
}

__exports.__wbg_new_59cb74e423758ede = __wbg_new_59cb74e423758ede;

function getObject(idx) { return heap[idx]; }

let cachedTextEncoder = new TextEncoder('utf-8');

let WASM_VECTOR_LEN = 0;

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {

        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let writeOffset = 0;
        while (true) {
            const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
            const { read, written } = cachedTextEncoder.encodeInto(arg, view);
            arg = arg.substring(read);
            writeOffset += written;
            if (arg.length === 0) {
                break;
            }
            ptr = wasm.__wbindgen_realloc(ptr, size, size * 2);
            size *= 2;
        }
        WASM_VECTOR_LEN = writeOffset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    };
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function __wbg_stack_558ba5917b466edd(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).stack);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

__exports.__wbg_stack_558ba5917b466edd = __wbg_stack_558ba5917b466edd;

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

function __widl_f_set_property_CSSStyleDeclaration(arg0, arg1, arg2, arg3, arg4, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    let varg3 = getStringFromWasm(arg3, arg4);
    try {
        getObject(arg0).setProperty(varg1, varg3);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_set_property_CSSStyleDeclaration = __widl_f_set_property_CSSStyleDeclaration;

function __widl_instanceof_CanvasRenderingContext2D(idx) { return getObject(idx) instanceof CanvasRenderingContext2D ? 1 : 0; }

__exports.__widl_instanceof_CanvasRenderingContext2D = __widl_instanceof_CanvasRenderingContext2D;

function __widl_f_set_global_composite_operation_CanvasRenderingContext2D(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).globalCompositeOperation = varg1;
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_set_global_composite_operation_CanvasRenderingContext2D = __widl_f_set_global_composite_operation_CanvasRenderingContext2D;

function __widl_f_begin_path_CanvasRenderingContext2D(arg0) {
    getObject(arg0).beginPath();
}

__exports.__widl_f_begin_path_CanvasRenderingContext2D = __widl_f_begin_path_CanvasRenderingContext2D;

function __widl_f_stroke_CanvasRenderingContext2D(arg0) {
    getObject(arg0).stroke();
}

__exports.__widl_f_stroke_CanvasRenderingContext2D = __widl_f_stroke_CanvasRenderingContext2D;

function __widl_f_set_stroke_style_CanvasRenderingContext2D(arg0, arg1) {
    getObject(arg0).strokeStyle = getObject(arg1);
}

__exports.__widl_f_set_stroke_style_CanvasRenderingContext2D = __widl_f_set_stroke_style_CanvasRenderingContext2D;

function __widl_f_set_fill_style_CanvasRenderingContext2D(arg0, arg1) {
    getObject(arg0).fillStyle = getObject(arg1);
}

__exports.__widl_f_set_fill_style_CanvasRenderingContext2D = __widl_f_set_fill_style_CanvasRenderingContext2D;

function __widl_f_set_line_width_CanvasRenderingContext2D(arg0, arg1) {
    getObject(arg0).lineWidth = arg1;
}

__exports.__widl_f_set_line_width_CanvasRenderingContext2D = __widl_f_set_line_width_CanvasRenderingContext2D;

function __widl_f_line_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    getObject(arg0).lineTo(arg1, arg2);
}

__exports.__widl_f_line_to_CanvasRenderingContext2D = __widl_f_line_to_CanvasRenderingContext2D;

function __widl_f_move_to_CanvasRenderingContext2D(arg0, arg1, arg2) {
    getObject(arg0).moveTo(arg1, arg2);
}

__exports.__widl_f_move_to_CanvasRenderingContext2D = __widl_f_move_to_CanvasRenderingContext2D;

function __widl_f_fill_rect_CanvasRenderingContext2D(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
}

__exports.__widl_f_fill_rect_CanvasRenderingContext2D = __widl_f_fill_rect_CanvasRenderingContext2D;

function __widl_f_scale_CanvasRenderingContext2D(arg0, arg1, arg2, exnptr) {
    try {
        getObject(arg0).scale(arg1, arg2);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_scale_CanvasRenderingContext2D = __widl_f_scale_CanvasRenderingContext2D;

function __widl_f_translate_CanvasRenderingContext2D(arg0, arg1, arg2, exnptr) {
    try {
        getObject(arg0).translate(arg1, arg2);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_translate_CanvasRenderingContext2D = __widl_f_translate_CanvasRenderingContext2D;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function __widl_f_get_element_by_id_Document(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_get_element_by_id_Document = __widl_f_get_element_by_id_Document;

function __widl_f_ready_state_Document(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).readyState);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

__exports.__widl_f_ready_state_Document = __widl_f_ready_state_Document;

function __widl_f_add_event_listener_with_callback_EventTarget(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).addEventListener(varg1, getObject(arg3));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_add_event_listener_with_callback_EventTarget = __widl_f_add_event_listener_with_callback_EventTarget;

function __widl_instanceof_HTMLCanvasElement(idx) { return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0; }

__exports.__widl_instanceof_HTMLCanvasElement = __widl_instanceof_HTMLCanvasElement;

function __widl_f_get_context_HTMLCanvasElement(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_get_context_HTMLCanvasElement = __widl_f_get_context_HTMLCanvasElement;

function __widl_f_set_width_HTMLCanvasElement(arg0, arg1) {
    getObject(arg0).width = arg1;
}

__exports.__widl_f_set_width_HTMLCanvasElement = __widl_f_set_width_HTMLCanvasElement;

function __widl_f_set_height_HTMLCanvasElement(arg0, arg1) {
    getObject(arg0).height = arg1;
}

__exports.__widl_f_set_height_HTMLCanvasElement = __widl_f_set_height_HTMLCanvasElement;

function __widl_f_style_HTMLElement(arg0) {
    return addHeapObject(getObject(arg0).style);
}

__exports.__widl_f_style_HTMLElement = __widl_f_style_HTMLElement;

function __widl_f_replace_state_with_url_History(arg0, arg1, arg2, arg3, arg4, arg5, exnptr) {
    let varg2 = getStringFromWasm(arg2, arg3);
    let varg4 = arg4 == 0 ? undefined : getStringFromWasm(arg4, arg5);
    try {
        getObject(arg0).replaceState(getObject(arg1), varg2, varg4);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_replace_state_with_url_History = __widl_f_replace_state_with_url_History;

function __widl_f_key_KeyboardEvent(ret, arg0) {

    const retptr = passStringToWasm(getObject(arg0).key);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}

__exports.__widl_f_key_KeyboardEvent = __widl_f_key_KeyboardEvent;

function __widl_f_assign_Location(arg0, arg1, arg2, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {
        getObject(arg0).assign(varg1);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_assign_Location = __widl_f_assign_Location;

function __widl_f_search_Location(ret, arg0, exnptr) {
    try {

        const retptr = passStringToWasm(getObject(arg0).search);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_search_Location = __widl_f_search_Location;

function __widl_f_width_Screen(arg0, exnptr) {
    try {
        return getObject(arg0).width;
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_width_Screen = __widl_f_width_Screen;

function __widl_f_height_Screen(arg0, exnptr) {
    try {
        return getObject(arg0).height;
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_height_Screen = __widl_f_height_Screen;

function __widl_instanceof_Window(idx) { return getObject(idx) instanceof Window ? 1 : 0; }

__exports.__widl_instanceof_Window = __widl_instanceof_Window;

function __widl_f_request_animation_frame_Window(arg0, arg1, exnptr) {
    try {
        return getObject(arg0).requestAnimationFrame(getObject(arg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_request_animation_frame_Window = __widl_f_request_animation_frame_Window;

function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}

__exports.__widl_f_document_Window = __widl_f_document_Window;

function __widl_f_location_Window(arg0) {
    return addHeapObject(getObject(arg0).location);
}

__exports.__widl_f_location_Window = __widl_f_location_Window;

function __widl_f_history_Window(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).history);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_history_Window = __widl_f_history_Window;

function __widl_f_screen_Window(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).screen);
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__widl_f_screen_Window = __widl_f_screen_Window;

function __widl_f_device_pixel_ratio_Window(arg0) {
    return getObject(arg0).devicePixelRatio;
}

__exports.__widl_f_device_pixel_ratio_Window = __widl_f_device_pixel_ratio_Window;

function __wbg_newnoargs_b4526aa2a6db81de(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

__exports.__wbg_newnoargs_b4526aa2a6db81de = __wbg_newnoargs_b4526aa2a6db81de;

function __wbg_call_a7a8823c404228ab(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}

__exports.__wbg_call_a7a8823c404228ab = __wbg_call_a7a8823c404228ab;

function __wbg_new_3a746f2619705add(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}

__exports.__wbg_new_3a746f2619705add = __wbg_new_3a746f2619705add;

function __wbg_call_f54d3a6dadb199ca(arg0, arg1) {
    return addHeapObject(getObject(arg0).call(getObject(arg1)));
}

__exports.__wbg_call_f54d3a6dadb199ca = __wbg_call_f54d3a6dadb199ca;

function __wbg_self_ac379e780a0d8b94(arg0) {
    return addHeapObject(getObject(arg0).self);
}

__exports.__wbg_self_ac379e780a0d8b94 = __wbg_self_ac379e780a0d8b94;

function __wbg_crypto_1e4302b85d4f64a2(arg0) {
    return addHeapObject(getObject(arg0).crypto);
}

__exports.__wbg_crypto_1e4302b85d4f64a2 = __wbg_crypto_1e4302b85d4f64a2;

function __wbg_getRandomValues_1b4ba144162a5c9e(arg0) {
    return addHeapObject(getObject(arg0).getRandomValues);
}

__exports.__wbg_getRandomValues_1b4ba144162a5c9e = __wbg_getRandomValues_1b4ba144162a5c9e;

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

function __wbg_getRandomValues_1ef11e888e5228e9(arg0, arg1, arg2) {
    let varg1 = getArrayU8FromWasm(arg1, arg2);
    getObject(arg0).getRandomValues(varg1);
}

__exports.__wbg_getRandomValues_1ef11e888e5228e9 = __wbg_getRandomValues_1ef11e888e5228e9;

function __wbg_require_6461b1e9a0d7c34a(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(require(varg0));
}

__exports.__wbg_require_6461b1e9a0d7c34a = __wbg_require_6461b1e9a0d7c34a;

function __wbg_randomFillSync_1b52c8482374c55b(arg0, arg1, arg2) {
    let varg1 = getArrayU8FromWasm(arg1, arg2);
    getObject(arg0).randomFillSync(varg1);
}

__exports.__wbg_randomFillSync_1b52c8482374c55b = __wbg_randomFillSync_1b52c8482374c55b;

function __wbindgen_string_new(p, l) { return addHeapObject(getStringFromWasm(p, l)); }

__exports.__wbindgen_string_new = __wbindgen_string_new;

function __wbindgen_is_undefined(i) { return getObject(i) === undefined ? 1 : 0; }

__exports.__wbindgen_is_undefined = __wbindgen_is_undefined;

function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}

__exports.__wbindgen_debug_string = __wbindgen_debug_string;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function __wbindgen_cb_drop(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}

__exports.__wbindgen_cb_drop = __wbindgen_cb_drop;

const __wbindgen_cb_forget = dropObject;

__exports.__wbindgen_cb_forget = __wbindgen_cb_forget;

function __wbindgen_jsval_eq(a, b) { return getObject(a) === getObject(b) ? 1 : 0; }

__exports.__wbindgen_jsval_eq = __wbindgen_jsval_eq;

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

__exports.__wbindgen_throw = __wbindgen_throw;

function __wbindgen_closure_wrapper95(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(41);
    const d = wasm.__wbg_function_table.get(40);
    const cb = function(arg0) {
        this.cnt++;
        try {
            return f(this.a, b, addHeapObject(arg0));

        } finally {
        if (--this.cnt === 0) { d(this.a, b); this.a = 0; }

    }

};
cb.a = a;
cb.cnt = 1;
let real = cb.bind(cb);
real.original = cb;
return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper95 = __wbindgen_closure_wrapper95;

function __wbindgen_closure_wrapper102(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(42);
    const d = wasm.__wbg_function_table.get(40);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, arg0);

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper102 = __wbindgen_closure_wrapper102;

function __wbindgen_closure_wrapper111(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(39);
    const d = wasm.__wbg_function_table.get(40);
    const cb = function() {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b);

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}

__exports.__wbindgen_closure_wrapper111 = __wbindgen_closure_wrapper111;

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}

__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref;

function __wbindgen_object_drop_ref(i) { dropObject(i); }

__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref;

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './n_body': __exports };
    if (module_or_path instanceof URL || typeof module_or_path === 'string' || module_or_path instanceof Request) {

        const response = fetch(module_or_path);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module_or_path, imports)
        .then(instance => {
            return { instance, module: module_or_path };
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    });
}

export default init;

