//! scriptit is a simple way to run JavaScript code in Rust
//!
//! scriptit will run your JS differently depending on your platform:
//!
//! - Run in a V8 interpreter for "native" targets
//! - Run in the WASM host interpreter for "wasm32" targets

pub mod core;

#[cfg(not(target_arch = "wasm32"))]
#[path = "js_v8/mod.rs"]
mod backend;

#[cfg(target_arch = "wasm32")]
#[path = "js_wasm/mod.rs"]
mod backend;

pub use backend::ScriptingEnvironment;
