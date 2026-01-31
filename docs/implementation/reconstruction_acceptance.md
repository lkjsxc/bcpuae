# Reconstruction Acceptance Criteria

Definition of done for reconstruction.

## Build Requirements

- [ ] `cargo build` succeeds without warnings
- [ ] All source files ≤200 lines
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` produces no changes
- [ ] Docker image builds successfully

## Functional Requirements

### TTY Mode
- [ ] TUI appears with alternate screen
- [ ] Line numbers visible in gray gutter
- [ ] Status bar shows mode in color
- [ ] Cursor visible and follows typing

### Navigation
- [ ] `h/j/k/l` move cursor in Normal mode
- [ ] `0` goes to start of line
- [ ] `$` goes to end of line
- [ ] Cursor stays visible when scrolling

### Mode Transitions
- [ ] `i` enters Insert mode
- [ ] `Esc` returns to Normal mode
- [ ] `:` enters Command mode
- [ ] Status bar updates immediately on mode change

### Text Editing
- [ ] Characters insert at cursor in Insert mode
- [ ] Backspace deletes before cursor
- [ ] Enter creates new line
- [ ] Backspace at line start merges with previous line

### File Operations
- [ ] `Ctrl+S` saves file
- [ ] `:w` saves file
- [ ] Dirty indicator `[+]` appears after edit
- [ ] Indicator clears after save

### Notifications
- [ ] Save shows success notification
- [ ] Errors show fatal notification
- [ ] Notifications expire after TTL
- [ ] Color coding matches level

### Simple Mode
- [ ] Non-TTY shows simple mode banner
- [ ] Can type lines
- [ ] `quit` exits
- [ ] Lines echoed back

## Error Handling

- [ ] Save without filename shows warning
- [ ] Invalid command shows warning
- [ ] Terminal restore on panic
- [ ] No panic for invalid user input

## Edge Cases

- [ ] Empty file shows one empty line
- [ ] Trailing newline preserved on save
- [ ] Unicode characters work
- [ ] Large files (>1000 lines) perform adequately
- [ ] Terminal resize handled gracefully
