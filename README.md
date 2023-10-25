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
error: failed to create a target world for package `hello-wasi-http` (/home/dev/wasm/hello-wasi-http/Cargo.toml)

Caused by:
    0: failed to merge local target `/home/dev/wasm/hello-wasi-http/wit`
    1: package not found
            --> /home/dev/wasm/hello-wasi-http/wit/command-extended.wit:4:10
             |
           4 |   import wasi:clocks/wall-clock@0.2.0-rc-2023-10-18;
             |          ^----------
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
