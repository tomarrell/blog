
.PHONY: watch

watch: ## Run the server locally and watch for file changes
	RUST_LOG=blog=trace,actix_web=info cargo watch -x run

watch-test: ## Watch for file changes and run the tests
	cargo watch -x test

clean: ## Clean
	cargo clean

build: ## Build a release binary
	cargo build --release

start: build ## Start the server, with correctly routed logging
	RUST_LOG=blog=trace,actix_web=info ./target/release/blog >> log.txt 2>> log.txt

monitor: ## Watch the log file
	 tail -f log.txt | cat

sitemap: ## Generate a sitemap
	sitemap-generator http://blog.tomarrell.com

# Requires installation of https://github.com/sharkdp/bat
# Comes with nice syntax highlighting
monitor-bat: ## Monitor the logs with bat
	 tail -f log.txt | bat --paging=never -l bash -p

## Help display.
## Pulls comments from beside commands and prints a nicely formatted
## display with the commands and their usage information.

.DEFAULT_GOAL := help

help: ## Prints this help
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

