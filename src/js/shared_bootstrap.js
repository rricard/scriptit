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

ScriptIt.core = {
    callToRust,
};
