# Hello wasi-http!

This is a simple tutorial to get started with wasi-http using the
`wasmtime serve` command.

## Let's go!

First, [install `cargo component`](https://github.com/bytecodealliance/cargo-component#requirements).

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

Build it with:
```
$ cargo component build
thread 'main' panicked at /home/dev/.cargo/registry/src/index.crates.io-6f17d22bba15001f/wit-parser-0.12.1/src/resolve.rs:774:9:
assertion failed: prev.is_none()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
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
