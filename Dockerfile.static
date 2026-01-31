# bcpuae - Static Build
# Creates a fully static binary using musl target

FROM rust:1.80-alpine AS builder

# Install musl-dev for static linking
RUN apk add --no-cache musl-dev

WORKDIR /app

# Add musl target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY docs ./docs

# Build static binary
ENV RUSTFLAGS="-C target-feature=+crt-static"
RUN cargo build --target x86_64-unknown-linux-musl \
    --profile release-static

# Runtime stage - minimal scratch image
FROM scratch

# Copy static binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release-static/bcpuae /bcpuae

# Default to TUI mode
ENTRYPOINT ["/bcpuae"]
