run-example:
	@cargo build --release
	@gcc c-example/main.c -pthread -o target/c-example target/release/libckb_ffi.a -ldl
	@./target/c-example

clean:
	@cargo clean

ci: run-example ci-rust

ci-rust:
	set -eu; \
	export RUSTFLAGS='-F warnings'; \
	cargo fmt --all -- --check; \
	cargo clippy --all --all-targets --all-features; \
	cargo test --all --verbose; \
