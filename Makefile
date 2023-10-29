run-server:
	cd server && go run ./srv/...

build-wasm:
	cargo build --target wasm32-wasi --release
	mv target/wasm32-wasi/release/http-wasm-rust.wasm ./header.wasm
