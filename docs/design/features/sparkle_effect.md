# Sparkle Effect Feature

Visual feedback after period of inactivity.

## Overview

After 10 seconds of no user input, the editor text sparkles with rainbow colors to add visual interest and indicate the editor is idle.

## User Story

As a user taking a break from editing, I want to see a pleasing animation so that I know the editor is still running and hasn't frozen.

## Technical Requirements

### Timing
- **Inactivity threshold**: 10 seconds
- **Animation frame rate**: 10 FPS (100ms updates)
- **Color cycle duration**: 3.6 seconds full rainbow (100 hue steps)

### State Tracking
```rust
pub struct EditorState {
    // ... existing fields ...
    pub last_activity: Instant,
    pub is_idle: bool,
}
```

### Color Generation
```rust
fn rainbow_color(offset: u64) -> Color {
    let hue = (offset % 360) as f32;
    hsl_to_rgb(hue, 0.8, 0.6)
}
```

### Animation Trigger
- Reset `last_activity` on any key event
- Check inactivity on each Tick event
- Toggle `is_idle` flag when threshold crossed

### Visual Effect
- Each character gets a slightly different hue offset
- Creates a flowing rainbow wave effect across text
- Line numbers remain gray (not affected)

## Configuration (Future)

```toml
[sparkle]
enabled = true
threshold_seconds = 10
speed = 1.0  # multiplier
```

## Performance Considerations

- Only compute colors when idle
- Cache color calculations
- Skip effect if terminal doesn't support true color
