# Hello wasi-http!

This is a simple tutorial to get started with wasi-http using the
`wasmtime serve` command.

## Let's go!

First, [install `cargo component`](https://github.com/bytecodealliance/cargo-component#requirements) version 0.3.0.

Next, use `cargo-component new` to create a new project:

```sh
$ cargo component new hello-wasi-http
     Created binary (application) `hello-wasi-http` package
     Updated manifest of package `hello-wasi-http`
   Generated source file `src/main.rs`
$ cd hello-wasi-http
```

Copy the `wit` directory from
https://github.com/bytecodealliance/wasmtime/tree/release-14.0.0
TODO: Describe this more.

Copy Wasmtime's `api_proxy.rs` contents from trunk into src/main.rs.
crates/test-programs/src/bin/api_proxy.rs
TODO: Describe this more.

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
TODO: ...
TODO: we really shouldn't need *all* of these

Build it with:
```
$ cargo component build
error: failed to generate bindings from `/home/dev/wasm/hello-wasi-http/target/bindings/hello-wasi-http/target.wasm`: expected `exports` map to contain key `wasi:http/incoming-handler`
 --> src/lib.rs:1:1
  |
1 | cargo_component_bindings::generate!();
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: this error originates in the macro `cargo_component_bindings::generate` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0433]: failed to resolve: use of undeclared crate or module `bindings`
  --> src/lib.rs:17:5
   |
17 | use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};
   |     ^^^^^^^^ use of undeclared crate or module `bindings`

error[E0433]: failed to resolve: use of undeclared crate or module `bindings`
  --> src/lib.rs:21:6
   |
21 | impl bindings::exports::wasi::http::incoming_handler::Guest for T {
   |      ^^^^^^^^ use of undeclared crate or module `bindings`

```
TODO: fix the errors :smile:
TODO: say more

TODO: Install Wasmtime 14.0.0 with the "serve" feature enabled:

```
$ cargo install --locked --version=14.0.0 wasmtime-cli --features=serve
```

```
$ wasmtime serve ...
```
TODO: ...

TODO: wasmtime serve --help for more options

TODO: make a api_proxy_streaming.rs version
