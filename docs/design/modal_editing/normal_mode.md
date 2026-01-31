# Normal Mode

Default mode for navigation and command execution.

## Purpose

- Navigate through document
- Execute single-key commands
- Enter other modes (Insert, Command)
- Save and quit

## Key Bindings

### Movement

| Key | Action |
|-----|--------|
| `h` | Left |
| `j` | Down |
| `k` | Up |
| `l` | Right |
| `0` | Start of line |
| `$` | End of line |
| `gg` | First line |
| `G` | Last line |
| `w` | Next word |
| `b` | Previous word |

### Editing

| Key | Action |
|-----|--------|
| `x` | Delete character |
| `dd` | Delete line |
| `u` | Undo |
| `Ctrl+R` | Redo |

### Mode Entry

| Key | Action |
|-----|--------|
| `i` | Insert mode (before cursor) |
| `a` | Insert mode (after cursor) |
| `o` | Open line below, Insert mode |
| `:` | Command mode |

### File Operations

| Key | Action |
|-----|--------|
| `Ctrl+S` | Save |
| `Ctrl+Q` | Quit |

## Cursor Behavior

- Cursor is block-shaped (when terminal supports it)
- Cursor never leaves document bounds
- Cursor X clamps to line length when changing lines

## Error Handling

- Movement past bounds: silently clamp
- Invalid key: silently ignore (no error)
- Save without filename: show warning notification
