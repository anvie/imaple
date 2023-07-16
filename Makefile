



fmt:
	@@echo Formatting code...
	@@cargo fmt

test:
	cargo test

clean:
	@@echo Cleaning up...
	@@cargo clean

.PHONY: clean fmt

