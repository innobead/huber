PROJECT:=$(shell basename $(CURDIR))
COMMIT:=$(shell git rev-parse --short HEAD 2>/dev/null)-$(shell date "+%Y%m%d%H%M%S")
TAG:=$(shell git describe --tags --dirty 2>/dev/null)
BUILD_CACHE_DIR:=$(CURDIR)/.cache
BUILD_DIR := $(CURDIR)/.target
OUTPUT_DIR := $(CURDIR)/.output
HUBER_ARTIFACT := $(shell $(CURDIR)/hack/huber-artifact-name.sh)

.PHONY: help
help:
	@grep -E '^[a-zA-Z%_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: env
setup-dev: ## Setup development environment
	$(CURDIR)/hack/setup-dev.sh

.PHONY: buildk
build: fmt ## Build binaries
	cargo build $(CARGO_OPTS) --workspace --exclude=huber-generator

.PHONY: test
test: ## Run tests
	cargo test $(CARGO_OPTS) --workspace --exclude=huber-generator

.PHONY: fmt
fmt: ## Format & Lint codes
	rustup component add rustfmt clippy
	cargo fmt

.PHONY: release
release: ## Release binaries
	CARGO_OPTS="--release" $(MAKE) build
	mkdir -p $(BUILD_DIR) && cp $(CURDIR)/target/release/huber $(BUILD_DIR)/$(HUBER_ARTIFACT)
	$(MAKE) checksum

.PHONY: install
install: ## Install binaries
	cargo install $(CARGO_OPTS) --path ./src/app/ --bins
	mkdir -p ~/.huber/bin && cp ~/.cargo/bin/huber ~/.huber/bin && $(CURDIR)/hack/update-env.sh

.PHONY: clean
clean: ## Clean build caches
	cargo clean
	rm -rf $(BUILD_CACHE_DIR) $(BUILD_DIR) $(OUTPUT_DIR)

.PHONY: fix
fix:  ## Fix code
	cargo fix --allow-dirty || cargo fix --allow-staged

.PHONY: generate
generate: ## Generate managed package list
	@echo "! Must have GITHUB_TOKEN to automatically generate package description"
	GITHUB_TOKEN=$(GITHUB_TOKEN) cargo build --manifest-path=./src/generator/Cargo.toml
	GITHUB_KEY=$(GITHUB_KEY) $(MAKE) build && \
	(MANAGED_PKG_ROOT_DIR=$(CURDIR)/generated $(CURDIR)/target/debug/huber search | xargs -0 $(CURDIR)/hack/generate-packages.md.sh)

.PHONY: checksum
checksum: ## Generate checksum files for built executables
	$(CURDIR)/hack/generate-checksum.sh $(BUILD_DIR)

.PHONY: udep
udep: ## Check undepedencies
	cargo install cargo-udeps --locked
	cargo +nightly udeps  --workspace --exclude=huber-generator

.PHONY: build-multiarch
build-multiarch: ## Build binaries for linux multiple architectures
	PLATFORMS=linux/arm64 BUILD_TARGET=debug MAKE_TARGET="test build" $(CURDIR)/hack/build-multiarch.sh

.PHONY: release-multiarch
release-multiarch: ## Release binaries for linux multiple archite
	PLATFORMS=linux/arm64 BUILD_TARGET=release OUTPUT_DIR=$(OUTPUT_DIR) MAKE_TARGET=release $(CURDIR)/hack/build-multiarch.sh
	mkdir -p $(BUILD_DIR) && cp $(OUTPUT_DIR)/target/huber-* $(BUILD_DIR)/
	$(MAKE) checksum
