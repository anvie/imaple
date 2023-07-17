

all: fmt build

fmt:
	@@echo Formatting code...
	@@cargo fmt

build:
	@@cargo build

test:
	cargo test

clean:
	@@echo Cleaning up...
	@@cargo clean

.PHONY: clean fmt

