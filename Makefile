all: format build test examples

CARGO=RUST_BACKTRACE=1 cargo

build:
	$(CARGO) build --features default

test:
	$(CARGO) test --features default -- --nocapture 

dev: format
	$(CARGO) build --features dev
	$(CARGO) test --features dev -- --nocapture 

format:
	$(CARGO) fmt -- --write-mode overwrite

clean:
	$(CARGO) clean

examples:
	mkdir -p bin
	$(CARGO) build --example example_embed
	$(CARGO) build --example example_hyper
	cp `find target/debug/examples -type f -perm /111` bin

.PHONY: all examples
