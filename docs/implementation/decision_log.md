# Decision Log

Record of architectural decisions made during development.

## Format

```markdown
## YYYY-MM-DD: Decision Title

**Context**: Why the decision was needed

**Decision**: What was decided

**Consequences**: Impact on the codebase

**Alternatives Considered**: Other options and why rejected
```

---

## 2024-01-15: Async/Await with Tokio

**Context**: Editor needs non-blocking I/O for file operations while maintaining responsive UI.

**Decision**: Use Tokio for async runtime with MPSC channels for event passing.

**Consequences**:
- All I/O operations are async
- Event loop uses `tokio::select!` for multiplexing
- Requires `#[tokio::main]` attribute

**Alternatives Considered**:
- Synchronous I/O: Would block UI during file operations
- Custom async runtime: Too complex for this project

---

## 2024-01-15: Ratatui + Crossterm

**Context**: Need cross-platform TUI framework.

**Decision**: Use ratatui for UI components, crossterm for terminal I/O.

**Consequences**:
- Works on Linux, macOS, Windows
- High-level UI primitives (Layout, Paragraph, Block)
- Event stream support for async

**Alternatives Considered**:
- termion: Linux/macOS only
- pancurses: Requires ncurses dependency
- Custom VT100: Too low-level

---

## 2024-01-15: Three-Mode Design

**Context**: Editor needs clear separation between navigation and text entry.

**Decision**: Implement Normal/Insert/Command modes (Vim-inspired).

**Consequences**:
- Steep learning curve for non-Vim users
- Very efficient once learned
- Mode confusion possible (mitigated by clear status bar)

**Alternatives Considered**:
- Modeless (Emacs-style): More key combinations needed
- Modal with fewer modes: Less expressive

---

## 2024-01-15: Notification System with TTL

**Context**: Need transient feedback without interrupting editing flow.

**Decision**: MPSC-based notification system with automatic expiration.

**Consequences**:
- Non-blocking notifications
- Automatic cleanup
- History available for debugging

**Alternatives Considered**:
- Modal dialogs: Blocking, annoying
- Status bar only: Limited space for messages
- Log file only: Not immediate enough

---

## 2026-01-31: Docker-First Deployment

**Context**: Need consistent runtime environment and easy deployment.

**Decision**: Create Dockerfile with multi-stage build, use docker-compose for local dev.

**Consequences**:
- Reproducible builds across environments
- TTY handling requires `-it` flags
- Slightly larger distribution size

**Alternatives Considered**:
- Native binaries only: Environment-specific issues
- Snap packages: Less universal than Docker

---

## 2026-01-31: Cursor Visibility in Normal Mode

**Context**: Users need to see cursor position even in Normal mode for navigation.

**Decision**: Always show cursor block in Normal mode at cursor position.

**Consequences**:
- Requires crossterm cursor styling APIs
- Block cursor distinguishes Normal from Insert mode

**Alternatives Considered**:
- Hide cursor in Normal: Poor UX, users lose position
- Line highlight: Complex to implement

---

## 2026-01-31: Inactivity Sparkle Effect

**Context**: Visual feedback during idle periods adds personality.

**Decision**: After 10s inactivity, text sparkles with rainbow colors using HSL color cycling.

**Consequences**:
- Adds `last_activity` timestamp to state
- Requires animation frame timing
- Can be disabled via config (future)

**Alternatives Considered**:
- Screen saver: Too disruptive
- Status animation: Less visible
- No effect: Boring
