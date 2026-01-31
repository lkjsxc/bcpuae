# Phase 5: Integration

Main event loop and application glue.

## Files to Create

### `src/operations/file_io.rs` (~20 lines)
- [ ] Implement `read_file()` async
- [ ] Implement `write_file()` async

### `src/main.rs` (~270 lines)
- [ ] Panic hook for terminal restore
- [ ] TTY detection
- [ ] Simple mode fallback
- [ ] Terminal setup
- [ ] Notification initialization
- [ ] Event loop
- [ ] Key event handling
- [ ] Action execution
- [ ] Cleanup on exit

## Main Structure

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // Panic hook
    // Tracing init
    // TTY check → simple mode or TUI
    // Setup terminal
    // Init notification manager
    // Init event handler
    // Init editor state
    // Run event loop
    // Cleanup
}
```

## Event Loop

```rust
async fn run_event_loop(...) {
    loop {
        notification_manager.update();
        renderer.draw(...)?;
        match event_handler.next().await {
            Some(Event::Key(k)) => handle_key_event(k, ...).await?,
            Some(Event::Tick) => {},
            None => break,
        }
    }
}
```

## Known Issues to Fix

### Hardcoded Rect Bug

Current code has:
```rust
state.scroll_to_cursor(&Rect::new(0, 0, 80, 24));
```

Should use actual layout:
```rust
let layout = Layout::calculate(terminal.size()?);
state.scroll_to_cursor(&layout.editor);
```

## Acceptance Criteria

- [ ] Application starts in TTY mode
- [ ] Simple mode works without TTY
- [ ] All modes functional
- [ ] Save and quit work
- [ ] Terminal restores on exit
- [ ] Terminal restores on panic
- [ ] All files ≤200 lines
