use scriptit::{
    core::{error::ScriptError, value::ScriptValue},
    ScriptingEnvironment,
};
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn trigger_compile_error() {
    let mut s_env = ScriptingEnvironment::new();
    match s_env.eval_expression("import async return") {
        Err(ScriptError::CompileError(_)) => {}
        other => panic!("Expected a ScriptError::CompileError, got {:?}", other),
    }
}

#[test]
#[wasm_bindgen_test]
fn trigger_runtime_error() {
    let mut s_env = ScriptingEnvironment::new();
    match s_env.eval_expression("unknown_variable") {
        Err(ScriptError::RuntimeError(_)) => {}
        other => panic!("Expected a ScriptError::RuntimeError got {:?}", other),
    }
}

#[test]
#[wasm_bindgen_test]
fn get_boolean_value() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("true").unwrap();
    assert_eq!(val, ScriptValue::Boolean(true));
    let val = s_env.eval_expression("false").unwrap();
    assert_eq!(val, ScriptValue::Boolean(false));
}

#[test]
#[wasm_bindgen_test]
fn get_string_value() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("`hello`").unwrap();
    assert_eq!(val, ScriptValue::String("hello".to_string()));
    let val = s_env.eval_expression("`hello ${'foo'}!`").unwrap();
    assert_eq!(val, ScriptValue::String("hello foo!".to_string()));
}

#[test]
#[wasm_bindgen_test]
fn get_number_value() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("123").unwrap();
    assert_eq!(val, ScriptValue::Number(123.0));
    let val = s_env.eval_expression("12 + 3 ").unwrap();
    assert_eq!(val, ScriptValue::Number(15.0));
    let val = s_env.eval_expression("NaN").unwrap();
    match val {
        ScriptValue::Number(val) => assert!(val.is_nan()),
        _ => panic!("Expected a ScriptValue::Number, got a  {:?}", val),
    }
}

#[test]
#[wasm_bindgen_test]
fn get_null_value() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("null").unwrap();
    assert_eq!(val, ScriptValue::Null);
}

#[test]
#[wasm_bindgen_test]
fn get_undefined_value() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval_expression("undefined").unwrap();
    assert_eq!(val, ScriptValue::Undefined);
}
