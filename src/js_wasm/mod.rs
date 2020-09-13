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

    #[wasm_bindgen(constructor)]
    fn new() -> JSScriptingEnvironment;

    #[wasm_bindgen(method, catch)]
    fn eval(this: &JSScriptingEnvironment, s: &str) -> Result<JsValue, Error>;
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

fn jsvalue_to_scripterror(error: Error) -> ScriptError {
    ScriptError::RuntimeError(error.message())
}

/// A mocked environment that just proxies to the host
pub struct ScriptingEnvironment(JSScriptingEnvironment);

impl ScriptingEnvironment {
    pub fn new() -> ScriptingEnvironment {
        ensure_js_bootstrap();
        ScriptingEnvironment(JSScriptingEnvironment::new())
    }

    /// Evaluates some JS in the host
    pub fn eval(&mut self, source: &str) -> Result<ScriptValue, ScriptError> {
        match self.0.eval(source) {
            Ok(value) => jsvalue_to_scriptvalue(value),
            Err(value) => Err(jsvalue_to_scripterror(value)),
        }
    }
}
