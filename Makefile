all: format build test examples

build:
	cargo build --features default

test:
	cargo test --features default -- --nocapture 

dev: format
	cargo build --features dev
	cargo test --features dev -- --nocapture 

format:
	cargo fmt -- --write-mode overwrite

clean:
	cargo clean

examples:
	mkdir -p bin
	cargo build --example example_embed
	cargo build --example example_hyper
	cp `find target/debug/examples -type f -perm /111` bin

.PHONY: all examples
