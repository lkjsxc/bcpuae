# Quick Reference

Essential commands and reference for bcpuae.

## Docker Commands

### Build and Run

```bash
# Build image
docker build -t bcpuae .

# Run with volume mount (Linux/macOS)
docker run -it --rm \
  -v "$(pwd)/documents:/root/documents" \
  bcpuae

# Run with volume mount (Windows PowerShell)
docker run -it --rm `
  -v "${PWD}/documents:/root/documents" `
  bcpuae

# Run with full options
docker run -it --rm \
  --name bcpuae \
  -v "$(pwd)/documents:/root/documents" \
  -w /root/documents \
  -e TERM=xterm-256color \
  -e RUST_LOG=info \
  bcpuae
```

### Docker Compose

```bash
docker-compose build          # Build
docker-compose run --rm editor # Run editor
docker-compose run --rm dev   # Run dev shell
docker-compose run --rm builder # Build release
```

## Make Commands

```bash
make build      # Build Docker image
make run        # Run with volume mount
make run-simple # Run without volume
make dev        # Enter dev shell
make check      # Cargo check
make lint       # Cargo clippy
make fmt        # Cargo fmt
make validate   # Check line counts
make ci         # Run all CI checks
```

## Key Bindings

### Normal Mode

| Key | Action |
|-----|--------|
| `i` | Enter Insert mode |
| `a` | Enter Insert mode (after cursor) |
| `h` | Move cursor left |
| `j` | Move cursor down |
| `k` | Move cursor up |
| `l` | Move cursor right |
| `0` | Move to start of line |
| `$` | Move to end of line |
| `gg` | Go to first line |
| `G` | Go to last line |
| `Ctrl+S` | Save file |
| `Ctrl+Q` | Quit editor |

### Insert Mode

| Key | Action |
|-----|--------|
| `Esc` | Return to Normal mode |
| `Backspace` | Delete character before cursor |
| `Enter` | Insert new line |
| Any character | Insert at cursor position |

### Command Mode

Enter with `:` in Normal mode.

| Command | Action |
|---------|--------|
| `:w` | Save |
| `:w <file>` | Save as |
| `:q` | Quit (if saved) |
| `:q!` | Force quit |
| `:wq` or `:x` | Save and quit |
| `:<n>` | Go to line n |

## Project Structure

```
bcpuae/
├── Cargo.toml              # Rust project configuration
├── Dockerfile              # Multi-stage Docker build
├── docker-compose.yml      # Docker Compose service definition
├── README.md               # Project overview
├── AGENTS.md               # LLM reconstruction protocol
├── src/
│   ├── main.rs             # Application entry point
│   ├── core/               # Data model
│   │   ├── buffer.rs       # Text storage
│   │   ├── document.rs     # File management
│   │   ├── error.rs        # Error types
│   │   ├── location.rs     # Cursor position
│   │   └── mod.rs
│   ├── system/             # Async infrastructure
│   │   ├── events.rs       # Key event handling
│   │   ├── notifications.rs # Message bus
│   │   └── mod.rs
│   ├── ui/                 # Terminal UI
│   │   ├── layout.rs       # Screen regions
│   │   ├── renderer.rs     # Draw loop
│   │   ├── state.rs        # Editor state
│   │   ├── theme.rs        # Colors
│   │   └── mod.rs
│   └── operations/         # Business logic
│       ├── commands.rs     # Key→Action mapping
│       ├── file_io.rs      # Async file I/O
│       └── mod.rs
└── docs/                   # Documentation
    ├── policy/             # Operating contracts
    ├── design/             # Feature specifications
    ├── technical/          # Architecture reference
    ├── implementation/     # Reconstruction guides
    └── tmp/                # Agent prompts
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TERM` | `xterm-256color` | Terminal type |
| `RUST_LOG` | `info` | Log level (error, warn, info, debug, trace) |

## Troubleshooting

### Terminal not clearing properly

```bash
# Reset terminal if editor crashes
reset
# or
stty sane
```

### Docker: "the input device is not a TTY"

Ensure you're using `-it` flags:
- `-i` (interactive): Keep STDIN open
- `-t` (tty): Allocate a pseudo-TTY

### File permissions in Docker

The container runs as root by default. To preserve file ownership:

```bash
docker run -it --rm \
  -v "$(pwd)/documents:/root/documents" \
  -u $(id -u):$(id -g) \
  bcpuae
```

## Architecture Quick Reference

### Module Dependencies

```
main.rs
├── core (data)
├── operations (logic)
├── system (infrastructure)
└── ui (render)
    └── uses core
```

### Event Flow

```
Key Press → crossterm → EventHandler → MPSC Channel
                                           ↓
Action ← CommandMapper ← main loop ← Event::Key
   ↓
Buffer/Document update
   ↓
Renderer.draw()
```

### Notification Levels

| Level | Color | Duration |
|-------|-------|----------|
| Info | Blue | 5s |
| Success | Green | 3s |
| Warning | Yellow | 7s |
| Fatal | Red | 10s |

## Documentation Index

| Document | Purpose |
|----------|---------|
| [Policy](policy/INSTRUCT.md) | Operating contract, invariants |
| [Design](design/README.md) | Features, UX, modal editing |
| [Technical](technical/README.md) | Architecture, contracts |
| [Architecture](technical/ARCHITECTURE.md) | Complete type inventory |
| [Main.rs Spec](technical/MAIN_RS_SPEC.md) | Entry point reconstruction |
| [Implementation](implementation/README.md) | Reconstruction guides |
| [TODOs](implementation/todo/README.md) | Implementation backlog |
