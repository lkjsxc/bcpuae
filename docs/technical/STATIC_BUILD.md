# Static Build Guide

bcpuae can be built as a fully static binary with no external dependencies.

## Why Static Build?

- **No dependencies**: Single binary, no shared libraries needed
- **Portable**: Runs on any Linux distribution
- **Minimal Docker image**: Can run from `scratch` (5MB vs 100MB+)
- **Distribution**: Easy to ship as a single file

## Quick Start

### Using Make

```bash
# Build static binary
make build-static

# The binary will be at:
# target/x86_64-unknown-linux-musl/release-static/bcpuae
```

### Using Docker

```bash
# Build static Docker image
make docker-static

# Or manually:
docker build -f Dockerfile.static -t bcpuae:static .

# Run from scratch image (no OS overhead)
docker run -it --rm bcpuae:static
```

### Manual Build

```bash
# Add musl target
rustup target add x86_64-unknown-linux-musl

# Build static binary
RUSTFLAGS="-C target-feature=+crt-static" \
    cargo build --release --target x86_64-unknown-linux-musl \
    --profile release-static

# Check the binary is static
ldd target/x86_64-unknown-linux-musl/release-static/bcpuae
# Should output: "not a dynamic executable"
```

## Build Profiles

### Standard Release
- Dynamic linking to system libraries
- Larger deployment (needs glibc)
- Better performance in some cases

### Static Release (`release-static`)
- Fully static linking with musl libc
- Single binary ~5MB
- No external dependencies
- Slightly larger binary size

## Docker Images

| Image | Size | Base | Use Case |
|-------|------|------|----------|
| `bcpuae:latest` | ~100MB | Debian | Full featured |
| `bcpuae:static` | ~5MB | Scratch | Minimal/embedded |

## Verification

Check that your binary is truly static:

```bash
# Should show "not a dynamic executable"
ldd ./bcpuae

# Check file type
file ./bcpuae
# Should show: "statically linked"

# Check size
ls -lh ./bcpuae
```

## Platform Support

| Platform | Static Build Support |
|----------|---------------------|
| Linux x86_64 | ✅ Full support |
| Linux ARM64 | ✅ Via `aarch64-unknown-linux-musl` |
| macOS | ❌ Not supported (no musl) |
| Windows | ❌ Not supported |

## Troubleshooting

### Build Fails: "musl-gcc not found"

Install musl development files:

```bash
# Debian/Ubuntu
sudo apt-get install musl-tools

# Alpine
apk add musl-dev

# Arch
sudo pacman -S musl
```

### Binary Still Shows Dynamic Dependencies

Ensure the correct profile is used:

```bash
# Wrong - uses standard release profile
cargo build --release --target x86_64-unknown-linux-musl

# Correct - uses static profile (note: no --release flag)
cargo build --target x86_64-unknown-linux-musl --profile release-static
```

### Size is Larger Than Expected

The static profile includes:
- `lto = true`: Link-time optimization
- `strip = true`: Strip debug symbols
- `codegen-units = 1`: Maximum optimization

Expected size: 4-6MB
