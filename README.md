# Hello WASI HTTP!

This is a simple tutorial to get started with WASI HTTP using the
`wasmtime serve` command that can be enabled in [Wasmtime] 14.0. It runs an
HTTP server and forwards requests to a Wasm component via the [WASI HTTP] API.

[Wasmtime]: https://wasmtime.dev
[WASI HTTP]: https://github.com/WebAssembly/wasi-http/

The WASI HTTP API is settling down but as of this writing not quite stable.
This tutorial uses a snapshot of it that's implemented in Wasmtime 14.0.0.

With that said...

## Let's go!

First, [install `cargo component`](https://github.com/bytecodealliance/cargo-component#requirements),
version 0.3.1, which is a tool for building Wasm components implemented in Rust.

With that, build the Wasm component from the source in this repository:
```
$ cargo component build

TODO: output here
```

This builds a component in TODO: the path.

To run it, we'll use Wasmtime 14.0. We'll need to use a special `cargo install`
invocation rather than the usual installation instructions so that we can
enable the "serve" feature. To install:

```
$ cargo install --locked --version=14.0.1 wasmtime-cli --features=serve
```

Then, we can run `wasmtime serve` on our Wasm component:
```
$ wasmtime serve target/debug/hello-wasi-http.wasm
```
This starts up an HTTP server on `0.0.0.0:8080` (the specific address and port
can be configuted with the `--addr=` flag.

With that running, in another window, we can now make requests!
```
$ curl http://localhost:8080

TODO: hopefully stuff here like "Hello, wasi:http/proxy world!"
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

If you're interested in tutorials for either of these options, please reach out
and say hi!

[proxy]: https://github.com/WebAssembly/wasi-http/blob/main/wit/proxy.wit

## Creating this repo

Here are my notes on how I created this repository, in case you're intersted
in recreating it.

Run `cargo-component new --reactor` to create a new project:

```sh
$ cargo component new --reactor hello-wasi-http
     Created binary (application) `hello-wasi-http` package
     Updated manifest of package `hello-wasi-http`
   Generated source file `src/main.rs`
$ cd hello-wasi-http
```

Copy the `wit` directory from Wasmtime 14.0.0, to ensure that you're using the
same version of the API that Wasmtime is built with:

<https://github.com/bytecodealliance/wasmtime/tree/release-14.0.0>

TODO: Describe this more, and describe our changes.

In the future, we'll have wit dependencies stored in a registry, which will
make it much easier to add dependencies.

Copy Wasmtime's `api_proxy.rs` contents from Wasmtime trunk at
`crates/test-programs/src/bin/api_proxy.rs` into src/main.rs, and add the
wit-bindgen macro to it:

```rust
cargo_component_bindings::generate!();
```

I also tidied up the code slightly.

Add dependencies:
```
$ cargo component add --target --path wit/deps/clocks wasi:clocks
$ cargo component add --target --path wit/deps/filesystem wasi:filesystemm
$ cargo component add --target --path wit/deps/sockets wasi:sockets
$ cargo component add --target --path wit/deps/io wasi:io
$ cargo component add --target --path wit/deps/random wasi:random
$ cargo component add --target --path wit/deps/cli wasi:cli
$ cargo component add --target --path wit/deps/logging wasi:logging
```
TODO: we really shouldn't need *all* of these.

TODO: make a api_proxy_streaming.rs version
