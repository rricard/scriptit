#[cfg(feature = "js_v8")]
mod js_v8;

#[cfg(feature = "js_v8")]
#[path = "js_v8/mod.rs"]
mod backend;

#[cfg(feature = "js_wasm")]
compile_error!("`js_wasm` backend not implemented yet");

#[cfg(not(any(feature = "js_v8", feature = "js_wasm")))]
compile_error!("Please select at least a feature to build a default backend: `js_v8`, `js_wasm`");