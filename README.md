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

TODO: Install Wasmtime 14.0.0 with the "serve" feature enabled:

```
$ cargo install --locked --version=14.0.0 wasmtime-cli --features=serve
```

```
$ wasmtime serve
```
TODO: ...

TODO: make a api_proxy_streaming.rs version
