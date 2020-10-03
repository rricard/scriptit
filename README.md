# scriptit

![cargo build & test](https://github.com/rricard/scriptit/workflows/cargo%20build%20&%20test/badge.svg) ![wasm-pack build & test](https://github.com/rricard/scriptit/workflows/wasm-pack%20build%20&%20test/badge.svg)

scriptit is a simple way to run JavaScript code in Rust

scriptit will run your JS differently depending on your platform:

-   Run in a V8 interpreter for "native" targets
-   Run in the WASM host interpreter for "wasm32" targets

You can call rust functions from JS.

### Why?

I wanted to be able to do scripting in my rust applications, I do not need a fully-fledged embedding of v8 like node or deno and the only use here is as a library, so you get to choose what to inject.

Additionally I want to write most of my rust apps with a possible wasm target: as we likely have a js engine when running rust code on wasm targets, I thought about using the js interpreter on the host. This makes scriptit an extremely lightweight way to run scripts in wasm as we use host capabilities to do so!

### Limitations

Due to those goals, scriptit will not give you the same amount of control that you would have embedding v8 yourself and will give you worst ergonomics than just using wasm_bindgen. It is unfortunately ruled by the lowest common denominators on both apis (v8 & wasm_bindgen).

## Example

See the documentation for an example.

## Roadmap

scriptit is extremely experimental, I wouldn't use it for anything now, at least not before the following is done:

-   ES Modules support
-   Event-loop support
-   Promises/Futures support
-   Events from rust support
-   Passing to functions by reference instead of a serialized JSON copy

See [the issues](https://github.com/rricard/scriptit/issues) for more details.
