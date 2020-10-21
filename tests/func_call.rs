use std::{cell::RefCell, rc::Rc};

use scriptit::{
    core::{
        value::{ScriptNumber, ScriptValue},
        ScriptingEnvironment,
    },
    platform::PlatformScriptingEnvironment,
};
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn register_count_handler() {
    let count = Rc::new(RefCell::new(0 as u32));
    let closure_count = Rc::clone(&count);
    let mut s_env = PlatformScriptingEnvironment::new();
    s_env.register_func(
        "count",
        Box::new(move |_| {
            *closure_count.borrow_mut() += 1;
            let res = *closure_count.borrow();
            Ok(ScriptValue::Number(ScriptNumber::from(res)))
        }),
    );
    let val = s_env
        .eval_expression(
            "(
                ScriptIt.funcs.count(),
                ScriptIt.funcs.count(),
                ScriptIt.funcs.count()
            )",
        )
        .unwrap();

    assert_eq!(val, ScriptValue::Number(ScriptNumber::from(3)));
}

#[test]
#[wasm_bindgen_test]
fn register_data_and_avg_handlers() {
    let count = Rc::new(RefCell::new(0 as u32));
    let data_count = Rc::clone(&count);
    let total = Rc::new(RefCell::new(0.0 as f64));
    let data_total = Rc::clone(&total);

    let mut s_env = PlatformScriptingEnvironment::new();

    s_env.register_func(
        "data",
        Box::new(move |val| {
            let val = val.get(0).unwrap().as_f64().unwrap();
            *data_count.borrow_mut() += 1;
            *data_total.borrow_mut() += val;
            Ok(ScriptValue::Null)
        }),
    );

    s_env.register_func(
        "avg",
        Box::new(move |_| {
            let val = *total.borrow() / (*count.borrow() as f64);
            Ok(ScriptValue::Number(ScriptNumber::from_f64(val).unwrap()))
        }),
    );

    let val = s_env
        .eval_expression(
            "(
                ScriptIt.funcs.data(12.5),
                ScriptIt.funcs.data(13.0),
                ScriptIt.funcs.data(13.5),
                ScriptIt.funcs.avg()
            )",
        )
        .unwrap();

    assert_eq!(val, ScriptValue::Number(ScriptNumber::from(13)));
}
