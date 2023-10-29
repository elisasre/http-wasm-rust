# http-wasm-guest-rust

## guest os

add target

```bash
rustup target add wasm32-wasi
```

build
```bash
make build-wasm
```

## server

Server expects that there is a `header.wasm` file in the root directory of this repository.

```bash
make run-server
```

## testing

after wasm is compiled and server is running, it can be tested:

```bash
% curl localhost:8090/hello
X-Foo: Hello, World!
User-Agent: curl/8.1.2
Accept: */*
```
