# Phase 4: Notifications

Message system implementation.

## Files to Create

### `src/system/notifications.rs` (~120 lines)
- [ ] Define `NotifyLevel` enum
- [ ] Define `Notification` struct
- [ ] Define `NotificationManager` struct
- [ ] Implement global SENDER static
- [ ] Implement `init()`
- [ ] Implement `send()`
- [ ] Implement `update()`
- [ ] Implement `active()`, `history()`

## API Usage

```rust
// Initialize once
let mut nm = NotificationManager::init();

// Send from anywhere
NotificationManager::send("File saved", NotifyLevel::Success);

// Update in event loop
nm.update();

// Get active for rendering
let active = nm.active();
```

## TTL Values

| Level | TTL |
|-------|-----|
| Info | 5s |
| Success | 3s |
| Warning | 7s |
| Fatal | 10s |

## Acceptance Criteria

- [ ] Notifications appear when sent
- [ ] Expire after TTL
- [ ] Color coding correct
- [ ] Non-blocking (MPSC)
- [ ] All files ≤200 lines
