# Commands

Ex-style commands entered via Command mode (`:` in Normal mode).

## Command Syntax

```
:<command>[!] [arguments]
```

- `!` — Force flag (ignore warnings)
- Arguments are space-separated

## File Commands

| Command | Description |
|---------|-------------|
| `:w` | Write (save) current file |
| `:w <filename>` | Save as filename |
| `:q` | Quit (fails if unsaved changes) |
| `:q!` | Force quit (discard changes) |
| `:wq` or `:x` | Save and quit |
| `:e <filename>` | Edit file |

## Navigation Commands

| Command | Description |
|---------|-------------|
| `:n` or `:n <number>` | Go to line number |
| `:<number>` | Go to line (shorthand) |

## Error Handling

- Unknown commands show warning notification
- Failed operations show fatal notification with error details
- `:q` with unsaved changes shows warning with hint (`:q!` to force)
