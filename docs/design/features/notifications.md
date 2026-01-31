# Notifications

System for displaying transient messages to the user.

## Levels

| Level | Color | TTL | Use Case |
|-------|-------|-----|----------|
| Info | Blue | 5s | General information |
| Success | Green | 3s | Operations completed |
| Warning | Yellow | 7s | Non-fatal issues |
| Fatal | Red | 10s | Errors that need attention |

## Display

- Fixed 3-row notification area at bottom of screen
- Newest notification at bottom
- Older notifications scroll up
- Expired notifications removed automatically

## Prefixes

```
[INFO] General message
[OK]   Operation successful
[WARN] Warning message
[ERR]  Error message
```

## API

```rust
// From anywhere in the codebase
NotificationManager::send("File saved", NotifyLevel::Success);
```

## Behavior

- Non-blocking: notifications don't interrupt editing
- Channel-based: MPSC for thread-safe sending
- Auto-expire: TTL managed automatically
- History: Recent notifications kept in memory
