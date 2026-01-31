# Phase 6: Commands

Ex-style command implementation.

## Files to Update

### `src/operations/commands.rs`

Add command parsing:
- [ ] `Action::ExecuteCommand(String)`
- [ ] `Action::ForceQuit`
- [ ] `Action::QuitWithSaveCheck`

### `src/main.rs`

Add command execution:
- [ ] `execute_command()` function
- [ ] Command parsing logic
- [ ] Command handlers

## Commands to Implement

| Command | Action |
|---------|--------|
| `:w` | Save current file |
| `:w <file>` | Save as |
| `:q` | Quit (if saved) |
| `:q!` | Force quit |
| `:wq` or `:x` | Save and quit |
| `:e <file>` | Open file |
| `:<n>` | Go to line n |

## Command Parsing

```rust
fn parse_command(input: &str) -> Command {
    let input = input.trim();
    
    if let Ok(line_num) = input.parse::<usize>() {
        return Command::GoToLine(line_num);
    }
    
    match input {
        "w" | "write" => Command::Save(None),
        cmd if cmd.starts_with("w ") => Command::Save(Some(cmd[2..].to_string())),
        "q" | "quit" => Command::Quit { force: false },
        "q!" | "quit!" => Command::Quit { force: true },
        "wq" | "x" => Command::SaveAndQuit,
        cmd if cmd.starts_with("e ") => Command::Edit(cmd[2..].to_string()),
        _ => Command::Unknown(input.to_string()),
    }
}
```

## Command Buffer UI

In Command mode:
- Show `:` at status bar
- Append typed characters
- `Backspace` removes last char
- `Esc` cancels
- `Enter` executes

## Acceptance Criteria

- [ ] `:` enters command mode
- [ ] Commands parse correctly
- [ ] `:w` saves
- [ ] `:q` quits (with save check)
- [ ] `:q!` force quits
- [ ] `:<n>` jumps to line
- [ ] Unknown commands show warning
- [ ] All files ≤200 lines
