all: format build test

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

example:
	mkdir -p bin
	cargo build --bin example_embed
	cp target/debug/example_embed bin/
	cargo build --bin example_hyper
	cp target/debug/example_hyper bin/

.PHONY: example all
