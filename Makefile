all: format build test examples

build:
	cargo build --features default

test:
	cargo test --features default -- --nocapture 

dev: format
	cargo build --features dev
	cargo test --features="nightly dev" -- --nocapture

bench: format
	cargo bench --features dev -- --nocapture

format:
	@cargo fmt -- --write-mode diff > /dev/null || cargo fmt -- --write-mode overwrite || exit 0
	@rustfmt --write-mode diff examples/*.rs > /dev/null || rustfmt --write-mode overwrite examples/*.rs || exit 0
	@rustfmt --write-mode diff benches/*.rs > /dev/null || rustfmt --write-mode overwrite benches/*.rs || exit 0

clean:
	cargo clean

examples:
	cargo build --example example_embed
	cargo build --example example_hyper

.PHONY: all examples
