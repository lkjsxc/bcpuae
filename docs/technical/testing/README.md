# Testing

Test strategy and commands.

## Philosophy

- Docker-first: Tests run in container
- Unit tests: Per-module in `#[cfg(test)]` blocks
- Integration tests: Separate `tests/` directory
- Manual validation: Checklist-based

## Running Tests

```bash
# Unit tests
cargo test

# With logging
RUST_LOG=debug cargo test

# Docker
docker-compose run --rm test
```

## Test Categories

### Unit Tests

- Buffer operations
- Document state
- Command mapping
- Utility functions

### Integration Tests

- File I/O
- Event handling
- Terminal interaction (mocked)

### Manual Validation

See [Validation Checklist](#validation-checklist)

## Validation Checklist

### Build
- [ ] `cargo build` succeeds
- [ ] All files ≤200 lines
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` produces no changes

### Functional
- [ ] TTY mode works
- [ ] Non-TTY simple mode works
- [ ] Normal mode navigation
- [ ] Insert mode typing
- [ ] Mode transitions
- [ ] Save file
- [ ] Quit

### Edge Cases
- [ ] Empty file
- [ ] Very long lines
- [ ] Unicode input
- [ ] Resize terminal
