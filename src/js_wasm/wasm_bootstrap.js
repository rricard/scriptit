class JSScriptingEnvironment {
    constructor() {}

    eval(s) {
        return eval(s);
    }
}

globalThis.JSScriptingEnvironment = JSScriptingEnvironment;
