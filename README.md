# Hello WASI HTTP!

This is a simple tutorial to get started with WASI HTTP using the
`wasmtime serve` command in [Wasmtime] 17.0. It runs an HTTP server and
forwards requests to a Wasm component via the [WASI HTTP] API.

[Wasmtime]: https://wasmtime.dev
[WASI HTTP]: https://github.com/WebAssembly/wasi-http/

The WASI HTTP API is now stable, and part of WASI Preview 2.

So without further ado...

## Let's go!

First, [install `cargo component`](https://github.com/bytecodealliance/cargo-component#requirements),
version 0.7.1, which is a tool for building Wasm components implemented in
Rust. (See [here] for information about building Wasm components from other
languages too!)

[here]: https://component-model.bytecodealliance.org/language-support.html

With that, build the Wasm component from the source in this repository:
```sh
$ cargo component build
   Compiling hello-wasi-http v0.0.0 (/home/wasm/hello-wasi-http)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
    Creating component /home/wasm/hello-wasi-http/target/wasm32-wasi/debug/hello_wasi_http.wasm
$
```

This builds a Wasm component, `target/wasm32-wasi/debug/hello_wasi_http.wasm`.

To run it, we'll need at least Wasmtime 17.0.0. Installation instructions are
on [wasmtime.dev]:

```sh
$ curl https://wasmtime.dev/install.sh -sSf | bash
```

[wasmtime.dev]: https://wasmtime.dev/

Then, in a new terminal, we can run `wasmtime serve` on our Wasm component:
```
$ wasmtime serve target/wasm32-wasi/debug/hello_wasi_http.wasm
```
This starts up an HTTP server on `0.0.0.0:8080` (the specific address and port
can be configured with the `--addr=` flag).

With that running, in another window, we can now make requests!
```
$ curl http://localhost:8080
Hello, wasi:http/proxy world!
```

## Notes

`wasmtime serve` uses the [proxy] world, which is a specialized world just for
accepting requests and producing responses. One interesting thing about the proxy
world is that it doesn't have a filesystem or network API. If you add code to the
example that tries to access files or network sockets, it won't be able to build,
because those APIs are not available in this world. This allows proxy components
to run in many different places, including specialized serverless environments
which may not provide traditional filesystem and network access.

But what if you do want to have it serve some files? One option will be to use
[WASI-Virt](https://github.com/bytecodealliance/WASI-Virt), which is a tool
that can bundle a filesystem with a component.

Another option is to use a custom world. The proxy world is meant to be able
to run in many different environments, but if you know your environment and
you know it has a filesystem, you could create your own world, by including
both the "wasi:http/proxy" and "wasi:filesystem/types" or any other APIs you want
the Wasm to be able to access. This would require a custom embedding of Wasmtime,
as it wouldn't run under plain `wasmtime serve`, so it's a little more work to
set up.

In the future, we expect to see standard worlds emerge that combine WASI HTTP
with many other APIs, such as [wasi-cloud-core].

[wasi-cloud-core]: https://github.com/WebAssembly/wasi-cloud-core

If you're interested in tutorials for any of these options, please reach out
and say hi!

[proxy]: https://github.com/WebAssembly/wasi-http/blob/main/wit/proxy.wit

## Creating this repo

Here are my notes on how I created this repository, in case you're interested
in recreating it.

Run `cargo-component new --proxy --lib` to create a new project:

```sh
$ cargo component new --reactor hello-wasi-http
     Created binary (application) `hello-wasi-http` package
     Updated manifest of package `hello-wasi-http`
   Generated source file `src/main.rs`
$ cd hello-wasi-http
```

Copy the `wit` directory from Wasmtime 17.0.0, to ensure that we're using the
same version of the API that Wasmtime is built with:

<https://github.com/bytecodealliance/wasmtime/tree/release-17.0.0>

I then manually trimmed the filesystem and sockets dependencies out.

In the future, we'll have wit dependencies stored in a registry, which will
make this all much easier.

I derived `src/lib.rs` from Wasmtime's
`crates/test-programs/src/bin/api_proxy.rs` contents on the `main` branch,
adapted it to work with cargo component, in particular by adding:

```rust
cargo_component_bindings::generate!();
```

and renaming the `T` type to `Component`, which the bindings expect.

Add dependencies:
```
$ cargo component add --target --path wit/deps/clocks wasi:clocks
$ cargo component add --target --path wit/deps/io wasi:io
$ cargo component add --target --path wit/deps/random wasi:random
$ cargo component add --target --path wit/deps/cli wasi:cli
$ cargo component add --target --path wit/deps/logging wasi:logging
```
These don't all actually get used in this tutorial, but they're currently
needed because of some of the interfaces we copied in from the Wasmtime tree
reference them.

TODO: I should also make a api_proxy_streaming.rs version to show streaming.
