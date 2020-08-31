use crate::core::{error::ScriptError, value::ScriptValue};
use rusty_v8 as v8;
use std::sync::Once;

static PLATFORM_INIT: Once = Once::new();

fn ensure_platform_init() {
    PLATFORM_INIT.call_once(|| {
        let platform = v8::new_default_platform().unwrap();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

fn trycatch_scope_to_scripterror(
    tc_scope: &mut v8::TryCatch<v8::HandleScope>,
    is_compile_step: bool,
) -> ScriptError {
    let exception = match tc_scope.exception() {
        Some(e) => e,
        None => {
            return ScriptError::CastError {
                type_from: "Option<v8::Exception>",
                type_to: "v8::Value",
            }
        }
    };
    let msg = v8::Exception::create_message(tc_scope, exception);
    if is_compile_step {
        ScriptError::CompileError(String::from(
            msg.get(tc_scope).to_rust_string_lossy(tc_scope),
        ))
    } else {
        ScriptError::RuntimeError(String::from(
            msg.get(tc_scope).to_rust_string_lossy(tc_scope),
        ))
    }
}

fn val_to_scriptvalue(
    scope: &mut v8::HandleScope,
    value: &v8::Local<v8::Value>,
) -> Result<ScriptValue, ScriptError> {
    if value.is_boolean() {
        return Ok(ScriptValue::Boolean(value.boolean_value(scope)));
    }
    if value.is_string() {
        let value = value.to_string(scope).ok_or(ScriptError::CastError {
            type_from: "v8::Value",
            type_to: "v8::String",
        })?;
        return Ok(ScriptValue::String(value.to_rust_string_lossy(scope)));
    }
    if value.is_number() {
        let value = value.number_value(scope).ok_or(ScriptError::CastError {
            type_from: "v8::Value",
            type_to: "f64",
        })?;
        return Ok(ScriptValue::Number(value));
    }
    if value.is_null() {
        return Ok(ScriptValue::Null);
    }
    if value.is_undefined() {
        return Ok(ScriptValue::Undefined);
    }

    Err(ScriptError::CastError {
        type_from: "v8::Value (probably object)",
        type_to: "ScriptValue",
    })
}

/// A V8 scripting environment. This API also exists on WASM but JS will execute insecurely there.
pub struct ScriptingEnvironment {
    isolate: v8::OwnedIsolate,
    global_context: v8::Global<v8::Context>,
}

impl ScriptingEnvironment {
    pub fn new() -> ScriptingEnvironment {
        ensure_platform_init();
        let mut isolate = v8::Isolate::new(Default::default());
        let global_context;
        {
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);
            // TODO: initialize the context here with our bindings
            global_context = v8::Global::new(scope, context);
        }
        ScriptingEnvironment {
            isolate,
            global_context,
        }
    }

    /// Evaluates some JS in the current environment
    pub fn eval(&mut self, source: &str) -> Result<ScriptValue, ScriptError> {
        let scope = &mut v8::HandleScope::with_context(&mut self.isolate, &self.global_context);
        let source = v8::String::new(scope, source).ok_or(ScriptError::CastError {
            type_from: "&str",
            type_to: "v8::String",
        })?;

        let tc_scope = &mut v8::TryCatch::new(scope);

        let script = match v8::Script::compile(tc_scope, source, None) {
            Some(script) => script,
            None => {
                return Err(trycatch_scope_to_scripterror(tc_scope, true));
            }
        };

        match script.run(tc_scope) {
            Some(value) => val_to_scriptvalue(tc_scope, &value),
            None => {
                return Err(trycatch_scope_to_scripterror(tc_scope, false));
            }
        }
    }
}
