// @ts-check

/**
 * @type {(string | number | symbol)[]}
 */
const GLOBAL_INCLUDE_LIST = ["NaN", "Math", "Date"];

/**
 * @type {ProxyHandler}
 */
const ISOLATION_PROXY_HANDLER = {
    get(target, attr) {
        if (GLOBAL_INCLUDE_LIST.includes(attr)) {
            return globalThis[attr];
        }
        return target[attr];
    },
    has(target, attr) {
        return attr in globalThis || attr in target;
    },
};

class JSScriptingEnvironment {
    constructor() {
        /**
         * @type {object}
         */
        this.sandbox = {};
        /**
         * @type {Proxy<object, any>}
         */
        this.sandboxProxy = new Proxy(this.sandbox, ISOLATION_PROXY_HANDLER);
    }

    /**
     * @param {string} name
     * @param {any} value
     */
    addToGlobal(name, value) {
        this.sandbox[name] = value;
    }

    /**
     * @param {string} stringSrc
     * @returns {(sandbox: Proxy<object, any>) => any}
     */
    compile(stringSrc) {
        const wrappedSource = `with (globalThis) { ${stringSrc} }`;
        /**
         * @type {any}
         */
        const compiledFunction = new Function("globalThis", wrappedSource);
        return compiledFunction;
    }

    /**
     * @param {(sandbox: Proxy<object, any>) => any} compiledFunction
     * @returns {any}
     */
    run(compiledFunction) {
        return compiledFunction(this.sandboxProxy);
    }
}

// @ts-ignore: we extend the global
globalThis.JSScriptingEnvironment = JSScriptingEnvironment;
