# Hello WASI HTTP!

This is a simple tutorial to get started with WASI HTTP using the `wasmtime serve` command introduced in [Wasmtime] 18.0.
It runs an HTTP server and forwards requests to a Wasm component via the [WASI HTTP] API.

[Wasmtime]: https://wasmtime.dev
[WASI HTTP]: https://github.com/WebAssembly/wasi-http/

The WASI HTTP API is now stable, and part of WASI 0.2.

So without further ado...

## Let's go!

First, [install Rust](https://www.rust-lang.org/learn/get-started) (version 1.82 or later).
For more information on building Wasm components from different languages, check [here]!

[here]: https://component-model.bytecodealliance.org/language-support.html

With that, build the Wasm component from source in this repository:
```sh
$ RUSTFLAGS="-Clink-arg=--wasi-adapter=proxy" cargo build --target=wasm32-wasip2
   Compiling wit-bindgen-rt v0.35.0
   Compiling bitflags v2.6.0
   Compiling hello-wasi-http v0.0.0 (/home/dev/hello-wasi-http)
    Finished `dev` profile [unoptimized + debuginfo] target(s)
```

This builds a Wasm component at `target/wasm32-wasip2/debug/hello_wasi_http.wasm`.

To run it, we'll need at least Wasmtime `18.0`. Installation instructions are on [wasmtime.dev]. For example, on Linux or macOS, use the command below:

```sh
$ curl https://wasmtime.dev/install.sh -sSf | bash
```

[wasmtime.dev]: https://wasmtime.dev/

Then, run in your terminal:
```sh
$ wasmtime serve target/wasm32-wasip2/debug/hello_wasi_http.wasm
```
This starts up an HTTP server that, by default, listens on `0.0.0.0:8080`.

With that running, in another window, we can now make requests!
```sh
$ curl http://localhost:8080
Hello, wasi:http/proxy world!
```

## Optimizing!

The above uses a `debug` build. To make a component that runs faster, run `cargo` with the `--release` flag.

It's also worth making sure you have a release build of Wasmtime; if you installed it from the instructions above
with wasmtime.dev, you're good.

Wasmtime has several tuning options that can improve performance in different situations—pass `-O help` for a
list. One that's especially useful here is `-O pooling-allocator`.

## Notes

`wasmtime serve` uses the [proxy] world, which is a specialized world just for accepting requests and producing
responses. One interesting thing about the proxy world is that it doesn't have a filesystem or network API. If you add
code to the example that tries to access files or network sockets, it won't be able to build, because those APIs are
not available in this world. This allows proxy components to run in many different places, including specialized
serverless environments which may not provide traditional filesystem and network access.

But, what if you do want to have it serve some files? One option will be to use 
[WASI-Virt](https://github.com/bytecodealliance/WASI-Virt), which is a tool that can bundle a filesystem with a 
component.

Another option is to use a custom `world`. The proxy world is meant to be able to run in many different environments,
but if you know your environment, and you know it has a filesystem, you could create your own world, by including both
the `"wasi:http/proxy"` and `"wasi:filesystem/types"` or any other APIs you want the Wasm to be able to access. This
would require a custom embedding of Wasmtime, as it wouldn't run under plain `wasmtime serve`, so it's a little more
work to set up.

In the future, we expect to see standard `world`s emerge that combine WASI HTTP with many other APIs, such as
[wasi-cloud-core].

If you're interested in tutorials for any of these options, please reach out and say hi!

[proxy]: https://github.com/WebAssembly/wasi-http/blob/main/wit/proxy.wit
[wasi-cloud-core]: https://github.com/WebAssembly/wasi-cloud-core

> TODO: I should also make a `api_proxy_streaming.rs` version to show streaming.
