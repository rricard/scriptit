// @ts-check

/**
 * Calls synchronously a rust primtive handler
 * @param {string} handler Name of the primitive handler
 * @param {string} data String Data to send to the primitive handler
 * @returns {string} String Data received from the primitive handler
 */
function callToRust(handler, data) {
    // Placeholder function that will be replaced by Rust
    return data;
}

/**
 * Create and attach a function bound to `ScriptIt.funcs`
 * @param {string} funcName Name of the function to attach
 * @param {string} handler Name of the `callToRust` handler
 */
function registerFunc(funcName, handler) {
    ScriptIt.funcs[funcName] = (...args) => {
        const data = JSON.stringify(args);
        const res = ScriptIt.core.callToRust(handler, data);
        return JSON.parse(res);
    };
}

ScriptIt.core = {
    callToRust,
    registerFunc,
};

ScriptIt.funcs = {};
