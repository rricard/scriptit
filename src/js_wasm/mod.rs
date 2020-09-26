use crate::core::{error::ScriptError, value::ScriptValue};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = eval, catch)]
    fn bootstrap_eval(s: &str) -> Result<BootstrapResult, Error>;

    type Error;

    #[wasm_bindgen(method, getter)]
    fn message(this: &Error) -> String;

    type BootstrapResult;
    type CompiledFunction;

    #[wasm_bindgen(method, catch)]
    fn compile(his: &BootstrapResult, s: &str) -> Result<CompiledFunction, Error>;

    #[wasm_bindgen(method, catch)]
    fn run(this: &BootstrapResult, fun: &CompiledFunction) -> Result<JsValue, Error>;
}

fn js_bootstrap() -> BootstrapResult {
    bootstrap_eval(include_str!("./wasm_bootstrap.js"))
        .map_err(|e| e.message())
        .unwrap()
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

fn jsvalue_to_script_compile_error(error: Error) -> ScriptError {
    ScriptError::CompileError(error.message())
}

fn jsvalue_to_script_runtime_error(error: Error) -> ScriptError {
    ScriptError::RuntimeError(error.message())
}

/// A mocked environment that just proxies to the host
pub struct ScriptingEnvironment(BootstrapResult);

impl ScriptingEnvironment {
    pub fn new() -> ScriptingEnvironment {
        ScriptingEnvironment(js_bootstrap())
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
