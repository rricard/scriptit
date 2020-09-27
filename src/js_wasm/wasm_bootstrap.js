// @ts-check
(() => {
    /**
     * Globals available in every engine to pass through
     * @see https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects
     * @type {(string | symbol | number)[]}
     */
    const passthroughGlobals = [
        "Infinity",
        "NaN",
        "undefined",
        // "globalThis", // Will get replaced by our own version
        "eval",
        "isFinite",
        "isNaN",
        "parseFloat",
        "parseInt",
        "encodeURI",
        "encodeURIComponent",
        "decodeURI",
        "decodeURIComponent",
        "Object",
        "Function",
        "Boolean",
        "Symbol",
        "Error",
        "AggregateError",
        "EvalError",
        "InternalError",
        "RangeError",
        "ReferenceError",
        "SyntaxError",
        "TypeError",
        "URIError",
        "Number",
        "BigInt",
        "Math",
        "Date",
        "String",
        "RegExp",
        "Array",
        "Int8Array",
        "Uint8Array",
        "Uint8ClampedArray",
        "Int16Array",
        "Uint16Array",
        "Uint16ClampedArray",
        "Int32Array",
        "Uint32Array",
        "Uint32ClampedArray",
        "Float32Array",
        "Float64Array",
        "BigInt64Array",
        "BigUint64Array",
        "Map",
        "Set",
        "WeakMap",
        "WeakSet",
        "ArrayBuffer",
        "SharedArrayBuffer",
        "Atomics",
        "DataView",
        "JSON",
        "Promise",
        "Generator",
        "GeneratorFunction",
        "AsyncFunction",
        "Reflect",
        "Proxy",
        "Intl",
        "WebAssembly",
    ];

    /**
     * The embedding sandbox
     */
    const sandbox = {
        ScriptIt: {},
    };
    const sandboxProxy = new Proxy(sandbox, {
        get(target, attr) {
            if (passthroughGlobals.includes(attr)) {
                return globalThis[attr];
            }
            return target[attr];
        },
        has(target, attr) {
            return attr in globalThis || attr in target;
        },
    });

    /**
     * @param {string} stringSrc
     * @returns {(sbx: typeof sandbox) => any}
     */
    function compile(stringSrc) {
        const wrappedSource = `with (globalThis) { ${stringSrc} }`;
        /** @type {any} */
        const compiledFunction = new Function("globalThis", wrappedSource);
        return compiledFunction;
    }

    /**
     * @param {(sbx: typeof sandbox) => any} compiledFunction
     * @returns {any}
     */
    function run(compiledFunction) {
        return compiledFunction(sandboxProxy);
    }

    /**
     * @param {(handler: string, data: string) => string} callToRust
     */
    function setCallToRust(callToRust) {
        sandbox.ScriptIt.core.callToRust = callToRust;
    }

    return {
        compile,
        run,
        setCallToRust,
    };
})();
