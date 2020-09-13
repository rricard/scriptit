use scriptit::{core::error::ScriptError, core::value::ScriptValue, ScriptingEnvironment};
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn ensure_no_global_prototype_leakage() {
    let mut s_env = ScriptingEnvironment::new();
    match s_env.eval_expression("prototype") {
        Err(ScriptError::RuntimeError(_)) => {}
        other => panic!("Expected a ScriptError::RuntimeError got {:?}", other),
    }
}

#[test]
#[wasm_bindgen_test]
fn ensure_no_console() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("console").unwrap();
    assert_eq!(val, ScriptValue::Undefined);
}

#[test]
#[wasm_bindgen_test]
fn ensure_no_js_scripting_environment() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("JSScriptingEnvironment").unwrap();
    assert_eq!(val, ScriptValue::Undefined);
}
