use std::{cell::RefCell, rc::Rc};

use scriptit::{core::value::ScriptValue, ScriptingEnvironment};
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn register_count_fn() {
    let count = Rc::new(RefCell::new(0 as u32));
    let closure_count = Rc::clone(&count);

    let mut s_env = ScriptingEnvironment::new();

    s_env.register_fn0(
        "count",
        Box::new(move || {
            *closure_count.borrow_mut() += 1;
            ScriptValue::Number(*closure_count.borrow() as f64)
        }),
    );

    let val = s_env
        .eval_expression("(count(), count(), count())")
        .unwrap();

    assert_eq!(val, ScriptValue::Number(3.0));
}

#[test]
#[wasm_bindgen_test]
fn register_data_and_avg_fn() {
    let count = Rc::new(RefCell::new(0 as u32));
    let data_count = Rc::clone(&count);
    let total = Rc::new(RefCell::new(0.0 as f64));
    let data_total = Rc::clone(&total);

    let mut s_env = ScriptingEnvironment::new();

    s_env.register_fn1(
        "data",
        Box::new(move |val| {
            if let ScriptValue::Number(val) = val {
                *data_count.borrow_mut() += 1;
                *data_total.borrow_mut() += val;
            }
            ScriptValue::Undefined
        }),
    );

    s_env.register_fn0(
        "avg",
        Box::new(move || ScriptValue::Number(*total.borrow() / (*count.borrow() as f64))),
    );

    let val = s_env
        .eval_expression("(data(12.5), data(13.0), data(13.5), avg())")
        .unwrap();

    assert_eq!(val, ScriptValue::Number(13.0));
}
