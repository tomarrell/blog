
.PHONY: watch

watch:
	RUST_LOG=blog=trace,actix_web=info cargo watch -x run
