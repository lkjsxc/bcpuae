//! Tests for EditorState

use super::*;

#[test]
fn test_new_state() {
    let state = EditorState::new();
    assert_eq!(state.cursor.x, 0);
    assert_eq!(state.cursor.y, 0);
    assert!(matches!(state.mode, EditorMode::Normal));
}

#[test]
fn test_set_mode() {
    let mut state = EditorState::new();
    state.set_mode(EditorMode::Insert);
    assert!(matches!(state.mode, EditorMode::Insert));
}

#[test]
fn test_move_cursor() {
    let mut state = EditorState::new();
    state.document =
        Document::with_buffer(crate::core::Buffer::from_str("hello\nworld"), None);

    // Move right
    state.move_cursor(1, 0);
    assert_eq!(state.cursor.x, 1);

    // Move down
    state.move_cursor(0, 1);
    assert_eq!(state.cursor.y, 1);
    // X should clamp to line length (5 for "world")
    assert_eq!(state.cursor.x, 1);
}

#[test]
fn test_move_start_end_of_line() {
    let mut state = EditorState::new();
    state.document = Document::with_buffer(crate::core::Buffer::from_str("hello"), None);
    state.cursor.x = 3;

    state.move_end_of_line();
    assert_eq!(state.cursor.x, 5);

    state.move_start_of_line();
    assert_eq!(state.cursor.x, 0);
}

#[test]
fn test_scroll_to_cursor() {
    let mut state = EditorState::new();
    // Create a document with many lines
    let lines: Vec<String> = (0..100).map(|i| format!("line {}", i)).collect();
    state.document = Document::with_buffer(
        crate::core::Buffer::from_str(&lines.join("\n")),
        None,
    );

    // Cursor at line 50, viewport height 20
    state.cursor.y = 50;
    let area = ratatui::layout::Rect::new(0, 0, 80, 20);
    
    state.scroll_to_cursor(&area);
    
    // Scroll offset should be adjusted so cursor is visible
    let start_line = state.start_line();
    assert!(state.cursor.y >= start_line);
    assert!(state.cursor.y < start_line + 20);
}
