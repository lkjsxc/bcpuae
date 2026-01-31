# Phase 3: Input Handling

Event handling and command mapping.

## Files to Create

### `src/system/events.rs` (~84 lines)
- [ ] Define `Event` enum (Key, Tick)
- [ ] Define `EventHandler` struct
- [ ] Implement `new()` with spawn
- [ ] Implement `next()` async method
- [ ] Set up tokio::select! with crossterm

### `src/operations/commands.rs` (~102 lines)
- [ ] Define `EditorMode` enum
- [ ] Define `Action` enum
- [ ] Implement `Display` for `EditorMode`
- [ ] Define `CommandMapper`
- [ ] Implement `map()` for key→action

### `src/system/mod.rs` (~10 lines)
- [ ] Declare modules
- [ ] Re-export public types

### `src/operations/mod.rs` (~10 lines)
- [ ] Declare modules
- [ ] Re-export public types

## Key Mapping

### Normal Mode

| Key | Action |
|-----|--------|
| `h` | MoveLeft |
| `j` | MoveDown |
| `k` | MoveUp |
| `l` | MoveRight |
| `0` | MoveStartOfLine |
| `$` | MoveEndOfLine |
| `i` | EnterMode(Insert) |
| `:` | EnterMode(Command) |
| `Ctrl+S` | Save |
| `Ctrl+Q` | Quit |

### Insert Mode

| Key | Action |
|-----|--------|
| `Esc` | EnterMode(Normal) |
| `Backspace` | DeleteChar |
| `Enter` | InsertNewline |
| `Char(c)` | InsertChar(c) |

## Acceptance Criteria

- [ ] Keys map to correct actions in each mode
- [ ] Event stream produces events
- [ ] Tick event fires every 250ms
- [ ] All files ≤200 lines
