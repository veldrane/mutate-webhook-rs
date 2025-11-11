# Build release binary
build:
	cargo build --release

# Build static binary

static:
	cargo build --release --target=x86_64-unknown-linux-musl

# Install to /usr/local/bin (can be overridden by PREFIX)
PREFIX ?= /usr/local

install: build
	install -Dm755 target/release/mutate-webhook-rs $(PREFIX)/bin/mutate-webhook-rs

# Install static binary to /usr/local/bin (can be overridden by PREFIX)
install-static: static
	install -Dm755 target/x86_64-unknown-linux-musl/release/mutate-webhook-rs $(PREFIX)/bin/mutate-webhook-rs

# Remove build artifacts
clean:
	cargo clean

# Run tests
test:
	cargo test

# Display help
help:
	@echo "Available targets:"
	@echo "  build    		- Build the project in release mode"
	@echo "  static   		- Build static binary in release mode"
	@echo "  install  		- Install the binary to $(PREFIX)/bin (default /usr/local/bin)"
	@echo "  install-static	- Install the static binary to $(PREFIX)/bin (default /usr/local/bin)"
	@echo "  clean    		- Remove build artifacts"
	@echo "  test     		- Run tests"
	@echo "  help     		- Show this help message"

.PHONY: build install clean test help