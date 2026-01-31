# Memory Optimization Guide

This document describes the memory optimizations applied to bcpuae.

## Optimizations Applied

### 1. Theme - Fixed-Size Arrays (68 lines)
**Before**: Two HashMaps with heap allocations
```rust
notification_colors: HashMap<NotifyLevel, Color>,  // 4 entries
mode_colors: HashMap<String, Color>,              // 3 entries
```

**After**: Fixed-size arrays on stack
```rust
notification_colors: [Color; 4],
mode_colors: [Color; 3],
```

**Savings**: ~100+ bytes per Theme instance, no heap allocations

### 2. Notifications - Bounded Buffers (172 lines)
**Before**: 
- VecDeque with 100 capacity history
- Unbounded active notifications
- OnceLock for global sender

**After**:
- Vec with 32 capacity history (MAX_HISTORY)
- Max 8 active notifications (MAX_ACTIVE)
- Simple static Option for sender

**Savings**: ~500+ bytes per NotificationManager, prevents unbounded growth

### 3. Buffer - Efficient Line Storage
Lines stored as `Vec<String>` with capacity hints:
- New buffer: 1 line minimum
- From string: exact line count

### 4. Static Binary
Using musl target with LTO:
- Final binary: ~765KB (scratch image)
- No dynamic library dependencies
- Minimal runtime memory footprint

## Memory Usage by Component

| Component | Approximate Size |
|-----------|-----------------|
| Empty Buffer | ~24 bytes + 1 line |
| Per Line | ~24 bytes + content |
| Theme | ~120 bytes (stack only) |
| EditorState | ~200 bytes + buffer |
| NotificationManager | ~1KB max (bounded) |
| IdleTracker | ~32 bytes |

## Build Sizes

| Build Type | Binary Size | Docker Image |
|------------|-------------|--------------|
| Debug | ~15MB | N/A |
| Release | ~5MB | ~100MB (Debian base) |
| Static | ~765KB | ~765KB (scratch base) |

## Runtime Memory Tips

1. **Large Files**: Buffer grows with content, but lines are efficiently stored
2. **Long Sessions**: Notification history capped at 32 entries
3. **Multiple Notifications**: Max 8 active, older ones expired immediately
4. **Sparkle Effect**: Minimal overhead - just color calculations

## Verification

```bash
# Check binary size
ls -lh target/release/bcpuae

# Check for dynamic dependencies (should show none for static)
ldd target/x86_64-unknown-linux-musl/release-static/bcpuae

# Check Docker image size
docker images bcpuae:static
```
