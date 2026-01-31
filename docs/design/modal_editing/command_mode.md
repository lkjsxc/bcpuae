# Command Mode

Mode for entering ex-style commands.

## Purpose

- Enter complex commands
- File operations with arguments
- Navigation to specific lines

## Entry

Press `:` in Normal mode.

Command line appears at bottom:
```
:____
```

Cursor positioned after colon.

## Key Bindings

| Key | Action |
|-----|--------|
| Printable | Append to command buffer |
| `Backspace` | Delete last character |
| `Enter` | Execute command |
| `Esc` | Cancel, return to Normal |
| `Ctrl+C` | Cancel (alternative) |

## Command Buffer

- Single-line buffer
- Characters accumulate as typed
- Shown in status area with `:` prefix

## Execution Flow

```
User presses Enter
        │
        ▼
Parse command and arguments
        │
        ▼
Dispatch to handler
        │
        ├──► File command ──▶ Execute
        ├──► Nav command ───▶ Execute
        └──► Unknown ───────▶ Error notification
        │
        ▼
Return to Normal mode
```

## Error Display

- Command errors shown as notifications
- Command line cleared on error
- User returns to Normal mode after error
