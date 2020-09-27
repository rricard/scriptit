use std::{cell::RefCell, rc::Rc};

use scriptit::{
    core::{value::ScriptValue, ScriptingEnvironment},
    platform::PlatformScriptingEnvironment,
};
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn register_count_handler() {
    let count = Rc::new(RefCell::new(0 as u32));
    let closure_count = Rc::clone(&count);
    let mut s_env = PlatformScriptingEnvironment::new();
    s_env.register_core_handler(
        "count",
        Box::new(move |_| {
            *closure_count.borrow_mut() += 1;
            String::from(format!("{}", *closure_count.borrow()))
        }),
    );
    let val = s_env
        .eval_expression(
            "(
                ScriptIt.core.callToRust('count', ''),
                ScriptIt.core.callToRust('count', ''),
                ScriptIt.core.callToRust('count', '')
            )",
        )
        .unwrap();

    assert_eq!(val, ScriptValue::String("3".to_string()));
}

#[test]
#[wasm_bindgen_test]
fn register_data_and_avg_handlers() {
    let count = Rc::new(RefCell::new(0 as u32));
    let data_count = Rc::clone(&count);
    let total = Rc::new(RefCell::new(0.0 as f64));
    let data_total = Rc::clone(&total);

    let mut s_env = PlatformScriptingEnvironment::new();

    s_env.register_core_handler(
        "data",
        Box::new(move |val| {
            let val: f64 = val.parse().unwrap();
            *data_count.borrow_mut() += 1;
            *data_total.borrow_mut() += val;
            "".to_string()
        }),
    );

    s_env.register_core_handler(
        "avg",
        Box::new(move |_| {
            let val = *total.borrow() / (*count.borrow() as f64);
            format!("{}", val).to_string()
        }),
    );

    let val = s_env
        .eval_expression(
            "(
                ScriptIt.core.callToRust('data', '12.5'),
                ScriptIt.core.callToRust('data', '13.0'),
                ScriptIt.core.callToRust('data', '13.5'),
                ScriptIt.core.callToRust('avg', '')
            )",
        )
        .unwrap();

    assert_eq!(val, ScriptValue::String("13".to_string()));
}
