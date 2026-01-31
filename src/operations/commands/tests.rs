//! Tests for CommandMapper

use crossterm::event::{KeyCode, KeyEvent};

use super::*;

#[test]
fn test_map_normal_navigation() {
    let key = KeyEvent::from(KeyCode::Char('h'));
    assert_eq!(
        CommandMapper::map(key, &EditorMode::Normal),
        Action::MoveLeft
    );

    let key = KeyEvent::from(KeyCode::Char('j'));
    assert_eq!(
        CommandMapper::map(key, &EditorMode::Normal),
        Action::MoveDown
    );
}

#[test]
fn test_map_insert_char() {
    let key = KeyEvent::from(KeyCode::Char('a'));
    assert_eq!(
        CommandMapper::map(key, &EditorMode::Insert),
        Action::InsertChar('a')
    );
}
