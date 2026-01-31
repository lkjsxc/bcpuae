# bcpuae Build Makefile

.PHONY: all build build-static docker docker-static test clean

# Default target
all: build

# Standard build
build:
	cargo build --release

# Static build using musl target
build-static:
	@echo "Building static binary..."
	@if ! rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then \
		rustup target add x86_64-unknown-linux-musl; \
	fi
	RUSTFLAGS="-C target-feature=+crt-static" \
		cargo build --target x86_64-unknown-linux-musl --profile release-static
	@echo "Static binary: target/x86_64-unknown-linux-musl/release-static/bcpuae"
	@ls -lh target/x86_64-unknown-linux-musl/release-static/bcpuae

# Docker build (dynamic linking)
docker:
	docker build -t bcpuae:latest .

# Docker build (static)
docker-static:
	docker build -f Dockerfile.static -t bcpuae:static .

# Run tests
test:
	cargo test

# Format code
fmt:
	cargo fmt

# Run clippy
clippy:
	cargo clippy

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Install locally (requires cargo)
install: build
	cargo install --path .

# Install static binary
install-static: build-static
	@echo "Installing static binary to ~/.cargo/bin/"
	cp target/x86_64-unknown-linux-musl/release-static/bcpuae ~/.cargo/bin/
