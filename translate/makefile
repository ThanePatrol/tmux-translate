ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: run-google, help, test


help: ## Display this screen
	@echo "Usage: make [target]"
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s \033[0m %s\n", $$1, $$2}'

run-google: ## Uses google translate api
	cargo build
	./target/debug/translate  --api-key $(GOOGLE_KEY)  --text "Hello how are you"

test: ## Runs unit tests sequentially
	cargo test -- --test-threads 1

install:
	cargo build --release
	cp ./target/release/translate ~/.local/bin

