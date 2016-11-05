all: format build test examples

build:
	cargo build --features default

test:
	cargo test --features="push" -- --nocapture

dev: format
	cargo test --features="push nightly dev" -- --nocapture

bench: format
	cargo bench --features dev -- --nocapture

format:
	@( cargo fmt -- --write-mode diff > /dev/null || cargo fmt -- --write-mode overwrite ) && \
	( rustfmt --write-mode diff examples/*.rs > /dev/null || rustfmt --write-mode overwrite examples/*.rs ) && \
	( rustfmt --write-mode diff benches/*.rs > /dev/null || rustfmt --write-mode overwrite benches/*.rs )

clean:
	cargo clean

examples:
	cargo build --example example_embed
	cargo build --example example_hyper
	cargo build --features="push" --example example_push

.PHONY: all examples
