# Insert Mode

Mode for text entry and modification.

## Purpose

- Insert printable characters
- Delete characters (backspace)
- Insert newlines

## Key Bindings

| Key | Action |
|-----|--------|
| Any printable | Insert at cursor position |
| `Backspace` | Delete character before cursor |
| `Enter` | Insert newline, move cursor to new line |
| `Esc` | Return to Normal mode |
| `Ctrl+H` | Backspace (alternative) |
| `Ctrl+J` | Newline (alternative) |
| `Ctrl+[` | Escape (alternative) |

## Cursor Behavior

- Cursor is bar-shaped (when terminal supports it)
- After insertion, cursor is after inserted character
- After backspace at line start, cursor moves to end of previous line

## Backspace Behavior

```
Case 1: x > 0 (middle of line)
  Delete character before cursor
  Cursor x -= 1

Case 2: x = 0, y > 0 (start of line, not first line)
  Merge current line with previous line
  Remove current line
  Cursor y -= 1
  Cursor x = previous_line_length

Case 3: x = 0, y = 0 (start of document)
  No operation
```

## Dirty State

Every insertion or deletion marks the document as dirty:
- Dirty indicator `[+]` appears in status bar
- Save prompt appears on quit

## Error Handling

- Invalid UTF-8 input: silently drop
- Buffer bounds error: log error, continue
