use std::{cell::RefCell, rc::Rc};

use scriptit::ScriptingEnvironment;
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn register_count_fn() {
    let count = Rc::new(RefCell::new(0 as u32));
    let closure_count = Rc::clone(&count);

    let mut s_env = ScriptingEnvironment::new();

    s_env.register_fn(
        "count",
        Box::new(move || {
            *closure_count.borrow_mut() += 1;
        }),
    );

    s_env.run("count();count();").unwrap();

    assert_eq!(*count.borrow(), 2);
}
