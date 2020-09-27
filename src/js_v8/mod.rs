use crate::core::{error::ScriptError, value::ScriptValue, ScriptingEnvironment};
use rusty_v8 as v8;
use std::{collections::HashMap, sync::Once};

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

struct V8ScriptingState {
    handlers: HashMap<String, Box<dyn FnMut(&str) -> String>>,
}

fn core_call_to_rust_receiver(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    assert_eq!(args.length(), 2);
    let handler_name = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let handler_data = args
        .get(1)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    let handler_result = {
        let mut state = scope.get_slot_mut::<V8ScriptingState>().unwrap();
        let handler_closure = state.handlers.get_mut(&handler_name).unwrap();
        let handler_result = handler_closure(&handler_data);
        handler_result
    };
    let handler_result = v8::String::new(scope, &handler_result).unwrap();
    rv.set(handler_result.into());
}

/// A V8 scripting environment. This API also exists on WASM but JS will execute insecurely there.
pub struct V8ScriptingEnvironment {
    isolate: v8::OwnedIsolate,
    global_context: v8::Global<v8::Context>,
}

impl V8ScriptingEnvironment {
    pub fn new() -> V8ScriptingEnvironment {
        ensure_platform_init();
        let mut isolate = v8::Isolate::new(Default::default());
        let global_context;
        {
            // Create Scope & Context and associate them to global_context
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);
            let scope = &mut v8::ContextScope::new(scope, context);
            global_context = v8::Global::new(scope, context);

            // Run the bootstrap scripts
            let bs_src = format!(
                "{}\n{}",
                include_str!("./v8_bootstrap.js"),
                include_str!("../js/shared_bootstrap.js")
            );
            let bs_src = v8::String::new(scope, &bs_src).unwrap();
            let bs_script = v8::Script::compile(scope, bs_src, None).unwrap();
            bs_script.run(scope).unwrap();

            // Go set ScriptIt.core.callToRust to core_call_to_rust_receiver
            let scriptit_str = v8::String::new(scope, "ScriptIt").unwrap();
            let core_str = v8::String::new(scope, "core").unwrap();
            let call_to_rust_str = v8::String::new(scope, "callToRust").unwrap();
            let call_to_rust_fn = v8::FunctionTemplate::new(scope, core_call_to_rust_receiver);
            let call_to_rust_fn = call_to_rust_fn.get_function(scope).unwrap();
            global_context
                .get(scope)
                .global(scope)
                .get(scope, scriptit_str.into())
                .unwrap()
                .to_object(scope)
                .unwrap()
                .get(scope, core_str.into())
                .unwrap()
                .to_object(scope)
                .unwrap()
                .set(scope, call_to_rust_str.into(), call_to_rust_fn.into());
        };

        // Initialize scripting state
        isolate.set_slot::<V8ScriptingState>(V8ScriptingState {
            handlers: HashMap::new(),
        });

        V8ScriptingEnvironment {
            isolate,
            global_context,
        }
    }
}

impl ScriptingEnvironment for V8ScriptingEnvironment {
    fn eval_expression(&mut self, source: &str) -> Result<ScriptValue, ScriptError> {
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

    fn run(&mut self, source: &str) -> Result<(), ScriptError> {
        match self.eval_expression(source) {
            Err(ScriptError::CastError { .. }) => Ok(()),
            Err(e) => Err(e),
            Ok(_) => Ok(()),
        }
    }

    fn register_core_handler(
        &mut self,
        handler_name: &str,
        handler_closure: Box<dyn FnMut(&str) -> String>,
    ) {
        let scope = &mut v8::HandleScope::with_context(&mut self.isolate, &self.global_context);
        scope
            .get_slot_mut::<V8ScriptingState>()
            .unwrap()
            .handlers
            .insert(handler_name.to_string(), handler_closure);
    }
}

pub type PlatformScriptingEnvironment = V8ScriptingEnvironment;
