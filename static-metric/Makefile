ENABLE_FEATURES ?= default

all: format build test examples

build:
	cargo build --features="${ENABLE_FEATURES}"

test:
	cargo test --features="${ENABLE_FEATURES}" -- --nocapture

format:
	@cargo fmt --all -- --write-mode diff >/dev/null || cargo fmt --all

bench: format
	RUSTFLAGS="--cfg bench" cargo bench --features="${ENABLE_FEATURES}" -- --nocapture

clean:
	cargo clean

examples:
	cargo build --example simple
	cargo build --example with_lazy_static
	cargo build --example advanced

.PHONY: all
