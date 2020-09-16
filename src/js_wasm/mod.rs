use crate::core::{error::ScriptError, value::ScriptValue};
use std::sync::Once;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = eval, catch)]
    fn bootstrap_eval(s: &str) -> Result<JsValue, Error>;

    type Error;

    #[wasm_bindgen(method, getter)]
    fn message(this: &Error) -> String;

    type JSScriptingEnvironment;
    type CompiledFunction;

    #[wasm_bindgen(constructor)]
    fn new() -> JSScriptingEnvironment;

    #[wasm_bindgen(js_name = addToGlobal, method)]
    fn add_to_global(this: &JSScriptingEnvironment, name: &str, func: JsValue);

    #[wasm_bindgen(method, catch)]
    fn compile(his: &JSScriptingEnvironment, s: &str) -> Result<CompiledFunction, Error>;

    #[wasm_bindgen(method, catch)]
    fn run(this: &JSScriptingEnvironment, fun: &CompiledFunction) -> Result<JsValue, Error>;
}

static JS_BOOTSTRAP: Once = Once::new();

fn ensure_js_bootstrap() {
    JS_BOOTSTRAP.call_once(|| {
        bootstrap_eval(include_str!("./wasm_bootstrap.js"))
            .map_err(|e| e.message())
            .unwrap();
    });
}

fn jsvalue_to_scriptvalue(value: JsValue) -> Result<ScriptValue, ScriptError> {
    if value.is_function() || value.is_object() {
        return Err(ScriptError::CastError {
            type_from: "JsValue (object or function)",
            type_to: "ScriptValue",
        });
    }
    if value.is_string() {
        let value = value.as_string().ok_or(ScriptError::CastError {
            type_from: "JsValue",
            type_to: "String",
        })?;
        return Ok(ScriptValue::String(value));
    }
    if value.is_null() {
        return Ok(ScriptValue::Null);
    }
    if value.is_undefined() {
        return Ok(ScriptValue::Undefined);
    }
    if let Some(value) = value.as_f64() {
        return Ok(ScriptValue::Number(value));
    }
    if let Some(value) = value.as_bool() {
        return Ok(ScriptValue::Boolean(value));
    }

    Err(ScriptError::CastError {
        type_from: "JsValue",
        type_to: "ScriptValue",
    })
}

fn scriptvalue_to_jsvalue(value: ScriptValue) -> JsValue {
    match value {
        ScriptValue::Number(n) => JsValue::from_f64(n),
        ScriptValue::Null => JsValue::null(),
        _ => JsValue::undefined(),
    }
}

fn jsvalue_to_script_compile_error(error: Error) -> ScriptError {
    ScriptError::CompileError(error.message())
}

fn jsvalue_to_script_runtime_error(error: Error) -> ScriptError {
    ScriptError::RuntimeError(error.message())
}

/// A mocked environment that just proxies to the host
pub struct ScriptingEnvironment(JSScriptingEnvironment);

impl ScriptingEnvironment {
    pub fn new() -> ScriptingEnvironment {
        ensure_js_bootstrap();
        ScriptingEnvironment(JSScriptingEnvironment::new())
    }

    pub fn register_fn0(&mut self, name: &str, mut func: impl FnMut() -> ScriptValue + 'static) {
        let closure = move || scriptvalue_to_jsvalue(func());
        let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut() -> JsValue>);
        self.0.add_to_global(name, closure.into_js_value());
    }

    pub fn register_fn1(
        &mut self,
        name: &str,
        mut func: impl FnMut(ScriptValue) -> ScriptValue + 'static,
    ) {
        let closure = move |v1| scriptvalue_to_jsvalue(func(jsvalue_to_scriptvalue(v1).unwrap()));
        let closure = Closure::wrap(Box::new(closure) as Box<dyn FnMut(JsValue) -> JsValue>);
        self.0.add_to_global(name, closure.into_js_value());
    }

    fn internal_eval(&mut self, source: &str) -> Result<ScriptValue, ScriptError> {
        let func = self
            .0
            .compile(source)
            .map_err(|e| jsvalue_to_script_compile_error(e))?;
        match self.0.run(&func) {
            Ok(value) => jsvalue_to_scriptvalue(value),
            Err(value) => Err(jsvalue_to_script_runtime_error(value)),
        }
    }

    /// Evaluates a single JS expression
    pub fn eval_expression(&mut self, source: &str) -> Result<ScriptValue, ScriptError> {
        self.internal_eval(&format!("return {}", source))
    }

    /// Runs JavaScript code
    pub fn run(&mut self, source: &str) -> Result<(), ScriptError> {
        self.internal_eval(source)?;
        Ok(())
    }
}
