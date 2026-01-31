# Core Features

## Modal Editing

Three distinct modes with clear visual indicators:

| Mode | Purpose | Indicator |
|------|---------|-----------|
| Normal | Navigation, commands | Status bar shows "NORMAL" (cyan) |
| Insert | Text entry | Status bar shows "INSERT" (green) |
| Command | Ex commands (`:w`, `:q`) | Status bar shows `:` prompt |

## Navigation

### Normal Mode

| Key | Action |
|-----|--------|
| `h` | Move left |
| `j` | Move down |
| `k` | Move up |
| `l` | Move right |
| `0` | Start of line |
| `$` | End of line |
| `gg` | First line |
| `G` | Last line |
| `Ctrl+U` | Page up |
| `Ctrl+D` | Page down |

### Scrolling

- Cursor always stays visible within viewport
- Automatic scroll when cursor moves outside visible area
- Smooth scrolling (not jumpy)

## Text Editing

### Insert Mode

| Key | Action |
|-----|--------|
| Any printable | Insert at cursor |
| `Backspace` | Delete before cursor |
| `Enter` | Insert newline |
| `Esc` | Return to Normal mode |

### Deletion (Normal Mode)

| Key | Action |
|-----|--------|
| `x` | Delete character under cursor |
| `dd` | Delete entire line |
| `dw` | Delete word |

## File Operations

| Action | Key/Command |
|--------|-------------|
| Save | `Ctrl+S` or `:w` |
| Save and quit | `:wq` or `:x` |
| Quit | `Ctrl+Q` or `:q` |
| Force quit | `:q!` |
| Open file | `:e <filename>` |

## Visual Feedback

- **Line numbers** — Gray gutter on left
- **Status bar** — Mode, filename, dirty indicator `[+]`
- **Notifications** — Color-coded messages (Info/Warning/Success/Fatal)
- **Cursor** — Always visible and correctly positioned
