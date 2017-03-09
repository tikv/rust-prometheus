ENABLE_FEATURES ?= default

all: format build test examples

build:
	cargo build --features="${ENABLE_FEATURES}"

test:
	cargo test --features="${ENABLE_FEATURES}" -- --nocapture

dev: format
	cargo test --features="${ENABLE_FEATURES} dev" -- --nocapture

bench: format
	cargo bench --features=${ENABLE_FEATURES} -- --nocapture

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
	cargo build --features="process" --example example_process_collector

gen_proto:
	@ which protoc >/dev/null || { echo "Please install protoc first"; exit 1; }
	@ which protoc-gen-rust >/dev/null || { echo "Please install protobuf rust plugin, cargo install protobuf"; exit 1; }
	protoc --rust_out proto proto/metrics.proto

.PHONY: all examples
