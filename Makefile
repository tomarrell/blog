.PHONY: push
push: ## Push the image to my personal docker registry with the git hash as the tag
	docker build -t tomarrell/personal:blog-$$(git show -s --format=%h) .
	docker push tomarrell/personal:blog-$$(git show -s --format=%h)
	echo "blog-$$(git show -s --format=%h)" | tr -d '\n'

.PHONY: watch
watch: ## Run the blog locally, restarting on changes
	watchexec -r "go run ."

## Help display.
## Pulls comments from beside commands and prints a nicely formatted
## display with the commands and their usage information.

.DEFAULT_GOAL := help

help: ## Prints this help
	@grep -h -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

