use scriptit::{
    core::{error::ScriptError, value::ScriptValue},
    ScriptingEnvironment,
};

#[test]
fn trigger_compile_error() {
    let mut s_env = ScriptingEnvironment::new();
    match s_env.eval("import async return") {
        Err(ScriptError::CompileError(_)) => {}
        other => panic!("Expected a ScriptError::CompileError, got {:?}", other),
    }
}

#[test]
fn trigger_runtime_error() {
    let mut s_env = ScriptingEnvironment::new();
    match s_env.eval("unknown_variable") {
        Err(ScriptError::RuntimeError(_)) => {}
        other => panic!("Expected a ScriptError::RuntimeError got {:?}", other),
    }
}

#[test]
fn get_boolean_value() {
    let mut s_env = ScriptingEnvironment::new();
    let val = s_env.eval("true").unwrap();
    assert_eq!(val, ScriptValue::Boolean(true));
    let val = s_env.eval("false").unwrap();
    assert_eq!(val, ScriptValue::Boolean(false));
}
