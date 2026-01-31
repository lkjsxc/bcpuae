# Security

Threat model and validation rules.

## Threat Model

### Assets

1. User files (being edited)
2. Terminal state
3. Editor process integrity

### Threats

| Threat | Severity | Mitigation |
|--------|----------|------------|
| Path traversal in save | High | Validate paths, restrict to allowed directories |
| Terminal escape injection | Medium | Filter/sanitize control characters |
| Resource exhaustion | Low | Limit buffer size, line count |
| File overwrite | Low | Confirmation for existing files |

## Validation Rules

### File Paths

- Reject paths containing `..`
- Reject absolute paths outside working directory (in container)
- Sanitize filename characters

### Input

- Validate all key input through crossterm
- Drop invalid UTF-8 sequences
- Limit input rate (if needed)

### Terminal

- Always restore raw mode on exit
- Handle panic with terminal restore hook
- Validate terminal capability before TUI mode

## Container Security

Dockerfile should:
- Run as non-root (if possible)
- Minimal base image
- No unnecessary capabilities
