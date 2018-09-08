ENABLE_FEATURES ?= default

all: format build test examples

build:
	cargo build --features="${ENABLE_FEATURES}"

test:
	cargo test --features="${ENABLE_FEATURES}" -- --nocapture

dev: format test

bench: format
	cargo bench --features=${ENABLE_FEATURES} -- --nocapture

format:
	@cargo fmt --all -- --check >/dev/null || cargo fmt --all

clean:
	cargo clean

examples:
	cargo build --all-features --examples

gen_proto:
	@ which protoc >/dev/null || { echo "Please install protoc first"; exit 1; }
	@ which protoc-gen-rust >/dev/null || { echo "Please install protobuf rust plugin, cargo install protobuf"; exit 1; }
	protoc --rust_out proto proto/metrics.proto

.PHONY: all examples
