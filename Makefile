all: format build test examples

build:
	cargo build --features default

test:
	cargo test --features default -- --nocapture 

dev: format
	cargo build --features dev
	cargo test --features dev -- --nocapture 
	cargo test --features="nightly dev" -- --nocapture

bench: format
	cargo bench --features dev -- --nocapture

format:
	cargo fmt -- --write-mode overwrite
	rustfmt --write-mode overwrite examples/*.rs
	rustfmt --write-mode overwrite benches/*.rs

clean:
	cargo clean

examples:
	cargo build --example example_embed
	cargo build --example example_hyper

.PHONY: all examples
