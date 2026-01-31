# Event Loop Contract

Invariants for the main application loop.

## Structure

```rust
loop {
    notification_manager.update();  // 1. Process expired notifications
    renderer.draw(...)?;            // 2. Draw UI
    event = event_handler.next();   // 3. Wait for event
    handle_event(event)?;           // 4. Process event
}
```

## Invariants

### INV-LOOP-1: Never Block UI

The draw phase must complete quickly (<16ms for 60fps). No I/O in draw.

### INV-LOOP-2: Async Event Polling

Event waiting is async (`await`). The loop yields control while waiting.

### INV-LOOP-3: Graceful Degradation

Errors in event handling don't crash the editor:
- Log error
- Notify user
- Continue loop

### INV-LOOP-4: Clean Shutdown

On exit:
1. Restore terminal to canonical mode
2. Leave alternate screen
3. Flush any pending output
4. Exit process

## Event Types

### Key Event

```rust
Event::Key(key) => {
    let action = CommandMapper::map(key, state.mode);
    execute_action(action)?;
}
```

### Tick Event

```rust
Event::Tick => {
    // Trigger redraw (for animations/blink)
}
```

### Channel Closed

```rust
None => {
    // Event handler shutdown, exit cleanly
    break;
}
```

## Timing

- Tick interval: 250ms
- Redraw: Every loop iteration
- Event wait: Asynchronous (blocks until event)
