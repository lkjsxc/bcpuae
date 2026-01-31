# Technical Architecture

Implementation-focused documentation for the running system.

## Start Here

- **[System Contracts](contracts/README.md)** — Cross-cutting invariants
- **[Architecture Reference](ARCHITECTURE.md)** — Complete type inventory
- **[Testing](testing/README.md)** — Test strategy and commands
- **[Security](security/README.md)** — Threat model and validation

## Module Overview

```
src/
├── main.rs           # Entry point, event loop
├── core/             # Data model
│   ├── buffer.rs     # Text storage
│   ├── document.rs   # File management
│   ├── error.rs      # Error types
│   ├── location.rs   # Cursor position
│   └── mod.rs
├── system/           # Async infrastructure
│   ├── events.rs     # Key event handling
│   ├── notifications.rs # Message bus
│   └── mod.rs
├── ui/               # Terminal UI
│   ├── layout.rs     # Screen regions
│   ├── renderer.rs   # Draw loop
│   ├── state.rs      # Editor state
│   ├── theme.rs      # Colors
│   └── mod.rs
└── operations/       # Business logic
    ├── commands.rs   # Key→Action mapping
    ├── file_io.rs    # Async file I/O
    └── mod.rs
```

## Key Technologies

- **Tokio** — Async runtime
- **Ratatui** — TUI framework
- **Crossterm** — Terminal I/O
- **Tracing** — Structured logging
