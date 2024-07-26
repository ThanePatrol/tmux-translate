.PHONY: run-google

SHELL := /bin/bash
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

help: ## Display this screen
	@echo "Usage: make [target]"
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s \033[0m %s\n", $$1, $$2}'


run-google: ## Uses google translate api
	cargo build
	./target/debug/translate --name google --api-key $(GOOGLE_KEY)
