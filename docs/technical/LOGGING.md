# Logging Configuration

bcpuae uses the `tracing` library for structured logging with configurable log levels.

## Default Behavior

By default, only **ERROR** level logs are output. This keeps the terminal clean during normal operation.

## Configuration

Set the `RUST_LOG` environment variable to control log levels:

### Log Levels (from least to most verbose)

```bash
# Errors only (default)
RUST_LOG=error

# Warnings and errors
RUST_LOG=warn

# Info, warnings, and errors
RUST_LOG=info

# Debug and above
RUST_LOG=debug

# Trace and above (everything)
RUST_LOG=trace
```

### Usage Examples

```bash
# Run with default (errors only)
./bcpuae

# Run with info logging
RUST_LOG=info ./bcpuae

# Run with debug logging (very verbose)
RUST_LOG=debug ./bcpuae

# Docker with info logging
docker run -it --rm -e RUST_LOG=info bcpuae

# Docker Compose - add to docker-compose.yml
environment:
  - RUST_LOG=info
```

### Module-Specific Logging

You can also set different levels for specific modules:

```bash
# Info for main app, debug for UI module
RUST_LOG=info,bcpuae::ui=debug

# Only show events module at trace level
RUST_LOG=error,bcpuae::system::events=trace
```

## Log Output

Logs are written to stderr. In TUI mode, they are not visible unless redirected:

```bash
# Redirect logs to a file
RUST_LOG=info ./bcpuae 2> editor.log

# View logs in another terminal
tail -f editor.log
```

## When to Use Different Levels

| Level | Use Case |
|-------|----------|
| `error` | **Default** - Only errors that affect functionality |
| `warn` | Warnings about potential issues |
| `info` | General operational information (startup, file operations) |
| `debug` | Detailed debugging information for development |
| `trace` | Very verbose - every event, keypress, etc. |
