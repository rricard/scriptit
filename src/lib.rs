//! scriptit is a simple way to run JavaScript code in Rust
//!
//! scriptit will run your JS differently depending on your platform:
//!
//! - Run in a V8 interpreter for "native" targets
//! - Run in the WASM host interpreter for "wasm32" targets
//!
//! ## Example
//!
//! ```
//! use scriptit::{
//!     core::{ value::ScriptValue, ScriptingEnvironment },
//!     platform::PlatformScriptingEnvironment,
//! };
//!
//! let mut s_env = PlatformScriptingEnvironment::new();
//!
//! s_env.register_func("greet", Box::new(|args| {
//!     let name = args.get(0).unwrap().as_str().unwrap();
//!     return ScriptValue::String(format!("Hello {}!", name));
//! }));
//!
//! let src = "(function() {
//!     const greeter = 'JS';
//!     const greeted = 'Rust';
//!     return `${ScriptIt.funcs.greet(greeted)} (from ${greeter}...)`;
//! })()";
//! let res = s_env.eval_expression(src).unwrap();
//!
//! assert_eq!(res, ScriptValue::String("Hello Rust! (from JS...)".to_string()));
//! ```

pub mod core;

#[cfg(not(target_arch = "wasm32"))]
#[path = "js_v8/mod.rs"]
pub mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "js_wasm/mod.rs"]
pub mod platform;
