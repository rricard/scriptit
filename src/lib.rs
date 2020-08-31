#[cfg(feature = "js_v8")]
#[path = "js_v8/mod.rs"]
mod backend;

#[cfg(not(any(feature = "js_v8", feature = "js_wasm")))]
compile_error!("Please select one feature to build a default backend: `js_v8`, `js_wasm`");
