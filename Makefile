run-server:
	cd server && go run ./srv/...

build-wasm:
	cargo build --target wasm32-wasi --release
	mv target/wasm32-wasi/release/http-wasm-guest-rust.wasm ./header.wasm
