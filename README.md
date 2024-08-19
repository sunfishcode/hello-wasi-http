# Hello WASI HTTP!

This is a simple tutorial to get started with WASI HTTP using the `wasmtime serve` command introduced in [Wasmtime] 18.0.
It runs an HTTP server and forwards requests to a Wasm component via the [WASI HTTP] API.

[Wasmtime]: https://wasmtime.dev
[WASI HTTP]: https://github.com/WebAssembly/wasi-http/

The WASI HTTP API is now stable, and part of WASI 0.2.

So without further ado...

## Let's go!

First, [install `cargo-component`](https://github.com/bytecodealliance/cargo-component#requirements) (version 0.13.2 or later). `cargo-component` is a tool for building Wasm components implemented in Rust. For more
information on building Wasm components from different languages, check [here]!

[here]: https://component-model.bytecodealliance.org/language-support.html

With that, build the Wasm component from source in this repository:
```sh
$ cargo component build
  Compiling hello-wasi-http v0.0.0 (/home/wasm/hello-wasi-http)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.01s
    Creating component target/wasm32-wasip1/debug/hello_wasi_http.wasm
```

This builds a Wasm component at `target/wasm32-wasip1/debug/hello_wasi_http.wasm`.

To run it, we'll need at least Wasmtime `18.0`. Installation instructions are on [wasmtime.dev]. For example, on Linux or macOS, use the command below:

```sh
$ curl https://wasmtime.dev/install.sh -sSf | bash
```

[wasmtime.dev]: https://wasmtime.dev/

Then, run in your terminal:
```sh
$ cargo component serve
```
This starts up an HTTP server that, by default, listens on `0.0.0.0:8080`.

With that running, in another window, we can now make requests!
```sh
$ curl http://localhost:8080
Hello, wasi:http/proxy world!
```

## Optimizing!

The above uses a `debug` build. To make a component that runs faster, build with `cargo component build --release`.

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

## Creating this repo

Here are my notes on how I created this repository, in case you're interested in recreating it.

To create a new project, run:

```sh
$ cargo component new --proxy --lib hello-wasi-http
    Created binary (application) `hello-wasi-http` package
    Updated manifest of package `hello-wasi-http`
    Generated source file `src/main.rs`
$ cd hello-wasi-http
```

Copy the `wit` directory from your version of  Wasmtime, to ensure that we're using the same version of the API that
Wasmtime is built with (e.g., for Wasmtime 18.0.0: https://github.com/bytecodealliance/wasmtime/tree/release-18.0.0).

Then, I manually trimmed the filesystem and sockets dependencies out.

In the future, we'll have wit dependencies stored in a registry, which will make this all much easier.

I derived `src/lib.rs` from Wasmtime's `crates/test-programs/src/bin/api_proxy.rs` contents on the `main` branch,
adapted it to work with cargo component, in particular by adding:

```rust
cargo_component_bindings::generate!();
```

Then, I renamed the `T` type to `Component`, which the bindings expect.

Finally, add dependencies:
```
$ cargo component add --target --path wit/deps/clocks wasi:clocks
$ cargo component add --target --path wit/deps/io wasi:io
$ cargo component add --target --path wit/deps/random wasi:random
$ cargo component add --target --path wit/deps/cli wasi:cli
$ cargo component add --target --path wit/deps/logging wasi:logging
```

These don't all actually get used in this tutorial, but they're currently needed because of some of the interfaces we
copied in from the Wasmtime tree reference them.

> TODO: I should also make a `api_proxy_streaming.rs` version to show streaming.
