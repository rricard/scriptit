pub mod core;

#[cfg(not(target_arch = "wasm32"))]
#[path = "js_v8/mod.rs"]
mod backend;

#[cfg(target_arch = "wasm32")]
#[path = "js_wasm/mod.rs"]
mod backend;

pub use backend::ScriptingEnvironment;
