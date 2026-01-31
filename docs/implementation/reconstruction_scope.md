# Reconstruction Scope

Defines what must be implemented for a complete reconstruction.

## Phase 1: Foundation

### Core Data Structures
- [ ] `Location` — Cursor position (x, y)
- [ ] `Buffer` — Text storage with line operations
- [ ] `Document` — File management + dirty state
- [ ] `EditorError` — Error types with thiserror

### Basic Operations
- [ ] Buffer: `insert_char`, `delete_char`, `insert_newline`
- [ ] Document: `open`, `save`, `mark_clean`, `mark_dirty`

## Phase 2: UI Infrastructure

### State Management
- [ ] `EditorState` — Document + cursor + mode
- [ ] `EditorMode` enum — Normal/Insert/Command
- [ ] Mode transitions

### Rendering
- [ ] `Theme` — Color definitions
- [ ] `Layout` — Screen region calculation
- [ ] `Renderer` — Draw loop with ratatui

### Layout
- [ ] Line number gutter
- [ ] Editor area
- [ ] Notification area (3 rows)
- [ ] Status bar (1 row)

## Phase 3: Input Handling

### Event System
- [ ] `Event` enum — Key, Tick
- [ ] `EventHandler` — Async event polling with crossterm
- [ ] MPSC channel for events

### Command Mapping
- [ ] `Action` enum — All possible actions
- [ ] `CommandMapper` — Key→Action mapping
- [ ] Mode-aware mapping

## Phase 4: Notifications

- [ ] `NotifyLevel` enum
- [ ] `Notification` struct
- [ ] `NotificationManager` — MPSC + TTL management

## Phase 5: Integration

### Main Event Loop
- [ ] TTY detection
- [ ] Terminal setup/restore
- [ ] Panic hook for cleanup
- [ ] Event loop: update → draw → wait

### Simple Mode
- [ ] Non-TTY fallback
- [ ] Line-based input
- [ ] Basic commands (quit, help)

## Phase 6: Commands

- [ ] Command mode (`:`)
- [ ] `:w` — Save
- [ ] `:q` — Quit
- [ ] `:q!` — Force quit
- [ ] `:wq` — Save and quit
- [ ] `:e <file>` — Open file

## Out of Scope (Future)

- Syntax highlighting
- Multiple buffers
- Search/replace
- Macros
- Plugins
- Config file
