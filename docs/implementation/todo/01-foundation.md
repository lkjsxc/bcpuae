# Phase 1: Foundation

Core data structures and basic operations.

## Files to Create

### `src/core/error.rs` (~24 lines)
- [ ] Define `EditorError` enum
- [ ] Implement `thiserror` derive
- [ ] Define `Result<T>` type alias

### `src/core/location.rs` (~56 lines)
- [ ] Define `Location` struct (x, y, g_index)
- [ ] Implement `new()`
- [ ] Implement movement helpers

### `src/core/buffer.rs` (~107 lines)
- [ ] Define `Buffer` struct (Vec<String>)
- [ ] Implement `new()`, `from_str()`
- [ ] Implement `insert_char()`
- [ ] Implement `delete_char()`
- [ ] Implement `insert_newline()`
- [ ] Implement `to_string()`, `lines()`

### `src/core/document.rs` (~105 lines)
- [ ] Define `Document` struct
- [ ] Implement `new()`, `open()`
- [ ] Implement `save()`
- [ ] Implement `mark_clean()`, `mark_dirty()`
- [ ] Implement `is_dirty()`, `buffer()`, `buffer_mut()`

### `src/core/mod.rs` (~20 lines)
- [ ] Declare modules
- [ ] Re-export public types

## Acceptance Criteria

- [ ] `cargo test` passes in core/
- [ ] Buffer always has ≥1 line
- [ ] UTF-8 handled correctly
- [ ] All files ≤200 lines

## Test Cases

```rust
#[test]
fn buffer_always_has_one_line() {
    let buf = Buffer::new();
    assert_eq!(buf.line_count(), 1);
}

#[test]
fn insert_char_works() {
    let mut buf = Buffer::new();
    let loc = Location::new(0, 0);
    let new_loc = buf.insert_char(&loc, 'a').unwrap();
    assert_eq!(new_loc.x, 1);
    assert_eq!(buf.line(0), "a");
}

#[test]
fn delete_char_merges_lines() {
    let mut buf = Buffer::from_str("line1\nline2");
    let loc = Location::new(0, 1);
    let new_loc = buf.delete_char(&loc).unwrap();
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.line(0), "line1line2");
}
```
