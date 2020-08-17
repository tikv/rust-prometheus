.PHONY: all build test format bench clean examples

ENABLE_FEATURES ?= default

all: format build test examples

build:
	cargo build --features="${ENABLE_FEATURES}"

test:
	cargo test --features="${ENABLE_FEATURES}" -- --nocapture

format:
	@cargo fmt --all -- --check >/dev/null || cargo fmt --all

bench: format
	RUSTFLAGS="--cfg bench" cargo bench --features="${ENABLE_FEATURES}" -- --nocapture

clean:
	cargo clean

examples:
	cargo build --example advanced
	cargo build --example local
	cargo build --example make_auto_flush_static_counter
	cargo build --example make_auto_flush_static_metric_histogram
	cargo build --example metric_enum
	cargo build --example register_integration
	cargo build --example simple
	cargo build --example with_lazy_static
