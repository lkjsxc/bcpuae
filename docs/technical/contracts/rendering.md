# Rendering Contract

Constraints for the draw loop.

## Invariants

### INV-RENDER-1: No State Mutation

Rendering must not mutate editor state. It reads from:
- `EditorState` (document, cursor, mode)
- `NotificationManager` (active notifications)
- `Theme` (colors)

### INV-RENDER-2: Complete Redraw

Each frame completely redraws the terminal. No incremental updates.

### INV-RENDER-3: Cursor Positioning

After draw, cursor is positioned at:
- Editor: cursor location (visible block cursor in Normal mode, bar in Insert)
- Command line: after typed text

### INV-RENDER-4: Terminal Size Handling

If terminal is resized:
- Recalculate layout
- Ensure cursor remains visible
- Clamp scroll offset if needed

### INV-RENDER-5: Cursor Visibility

Cursor must be visible in ALL modes:
- Normal mode: Block cursor at cursor position
- Insert mode: Bar cursor at cursor position  
- Command mode: Bar cursor after command text

### INV-RENDER-6: Viewport Scrolling

Cursor must never go off-screen. When cursor moves:
- If cursor.y < scroll_offset: scroll_offset = cursor.y
- If cursor.y >= scroll_offset + viewport_height: 
  - scroll_offset = cursor.y - viewport_height + 1

### INV-RENDER-7: Inactivity Animation

After 10 seconds of no input:
- Text sparkles with rainbow colors (HSL hue rotation)
- Animation updates every 100ms
- Any keypress resets inactivity timer

## Draw Order

1. Clear screen
2. Draw editor area (line numbers + content)
3. Draw notification area
4. Draw status bar
5. Position cursor
6. Apply cursor style (block/bar)

## Line Number Gutter

Format: `{:>4} │ `
- Always 6 characters wide
- Right-aligned line number (1-indexed)
- Box-drawing separator
- Trailing space

## Viewport Scrolling

```rust
// Ensure cursor is visible
let scroll_y = state.scroll_offset.1 as usize;
let viewport_height = layout.editor.height as usize;

if state.cursor.y < scroll_y {
    // Cursor above viewport
    state.scroll_offset.1 = state.cursor.y as u16;
} else if state.cursor.y >= scroll_y.saturating_add(viewport_height) {
    // Cursor below viewport
    state.scroll_offset.1 = (state.cursor.y.saturating_sub(viewport_height - 1)) as u16;
}
```

## Cursor Styling

```rust
// Normal mode - block cursor
crossterm::cursor::SetCursorStyle::Block

// Insert mode - line cursor  
crossterm::cursor::SetCursorStyle::Line
```

## Rainbow Sparkle Effect

```rust
// Calculate sparkle color based on time since inactivity
let inactive_duration = now.duration_since(last_activity);
if inactive_duration > Duration::from_secs(10) {
    let hue = (inactive_duration.as_millis() / 50) % 360;
    let color = hsl_to_rgb(hue as f32, 1.0, 0.5);
    // Apply to text characters
}
```

## Performance

Target: <16ms per frame for 60fps
- Limit lines rendered to viewport height
- Clip content outside viewport
- No syntax highlighting (for now)

## Error Handling

Render errors:
- Log with `error!` macro
- Do not propagate (don't crash editor)
- Continue loop (next frame may succeed)
