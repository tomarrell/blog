
.PHONY: watch

watch:
	RUST_LOG=blog=trace,actix_web=info cargo watch -x run

watch-test:
	cargo watch -x test

clean:
	cargo clean

build:
	cargo build --release

start: build
	RUST_LOG=blog=trace,actix_web=info ./target/release/blog >> log.txt 2>> log.txt
