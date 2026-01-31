# Error Handling Contract

Rules for error propagation and handling.

## Principles

1. **Recoverable errors** — Return `Result<T, EditorError>`
2. **Fatal errors** — Log and notify, attempt graceful exit
3. **Never panic for user input** — Invalid input is not exceptional

## Error Types

```rust
pub enum EditorError {
    IO(std::io::Error),
    Bounds(String),
    InvalidUtf8,
    Channel(String),
}
```

### IO Errors

- File read/write failures
- Terminal I/O errors
- Action: Return to caller, likely notify user

### Bounds Errors

- Cursor out of range
- Line index invalid
- Action: Log warning, clamp to valid range

### InvalidUtf8

- Invalid UTF-8 in input
- Action: Replace with replacement character or drop

### Channel Errors

- MPSC send/receive failures
- Action: Log error, may need graceful shutdown

## Error Propagation

```
Low-level (buffer, document)
    │
    ▼
Return Result<T, EditorError>
    │
    ▼
Mid-level (operations)
    │
    ▼
Propagate with ? or handle locally
    │
    ▼
High-level (main, event loop)
    │
    ▼
Display notification, continue or exit
```

## User Notification

| Error Type | Notification Level |
|------------|-------------------|
| IO (save) | Fatal |
| IO (read) | Warning |
| Bounds | Warning (internal error) |
| InvalidUtf8 | Warning |

## Panic Policy

Only panic for:
- Programming errors (unreachable!())
- Invariant violations that indicate corruption

Never panic for:
- Invalid user input
- File not found
- Network errors (if added)
