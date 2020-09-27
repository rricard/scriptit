use scriptit::{
    core::{error::ScriptError, value::ScriptValue, ScriptingEnvironment},
    platform::PlatformScriptingEnvironment,
};
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn ensure_no_global_prototype_leakage() {
    let mut s_env = PlatformScriptingEnvironment::new();
    match s_env.eval_expression("prototype") {
        Err(ScriptError::RuntimeError(_)) => {}
        other => panic!("Expected a ScriptError::RuntimeError got {:?}", other),
    }
}

#[test]
#[wasm_bindgen_test]
fn ensure_no_console() {
    let mut s_env = PlatformScriptingEnvironment::new();
    let val = s_env.eval_expression("console").unwrap();
    assert_eq!(val, ScriptValue::Undefined);
}
