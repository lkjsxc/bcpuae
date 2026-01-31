//! Tests for Buffer

use super::*;

#[test]
fn test_empty_buffer() {
    let buf = Buffer::new();
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.get_line(0), Some(""));
}

#[test]
fn test_from_str_empty() {
    let buf = Buffer::from_str("");
    assert_eq!(buf.line_count(), 1);
    assert_eq!(buf.get_line(0), Some(""));
}

#[test]
fn test_from_str_no_trailing_newline() {
    let buf = Buffer::from_str("line1\nline2");
    assert_eq!(buf.line_count(), 2);
    assert_eq!(buf.get_line(0), Some("line1"));
    assert_eq!(buf.get_line(1), Some("line2"));
}

#[test]
fn test_from_str_with_trailing_newline() {
    let buf = Buffer::from_str("line1\nline2\n");
    assert_eq!(buf.line_count(), 3);
    assert_eq!(buf.get_line(0), Some("line1"));
    assert_eq!(buf.get_line(1), Some("line2"));
    assert_eq!(buf.get_line(2), Some(""));
}

#[test]
fn test_insert_char() {
    let mut buf = Buffer::new();
    let loc = Location::new(0, 0);
    let new_loc = buf.insert_char(&loc, 'h').unwrap();
    assert_eq!(new_loc.x, 1);
    assert_eq!(buf.get_line(0), Some("h"));
}

#[test]
fn test_insert_char_unicode() {
    let mut buf = Buffer::new();
    let loc = Location::new(0, 0);
    buf.insert_char(&loc, '中').unwrap();
    assert_eq!(buf.get_line(0), Some("中"));
}

#[test]
fn test_delete_char() {
    let mut buf = Buffer::from_str("hello");
    let loc = Location::new(5, 0);
    let new_loc = buf.delete_char(&loc).unwrap();
    assert_eq!(new_loc.x, 4);
    assert_eq!(buf.get_line(0), Some("hell"));
}

#[test]
fn test_delete_char_merge_lines() {
    let mut buf = Buffer::from_str("line1\nline2");
    let loc = Location::new(0, 1);
    let new_loc = buf.delete_char(&loc).unwrap();
    assert_eq!(new_loc.y, 0);
    assert_eq!(new_loc.x, 5);
    assert_eq!(buf.get_line(0), Some("line1line2"));
    assert_eq!(buf.line_count(), 1);
}

#[test]
fn test_insert_newline() {
    let mut buf = Buffer::from_str("hello world");
    let loc = Location::new(5, 0);
    let new_loc = buf.insert_newline(&loc).unwrap();
    assert_eq!(new_loc.y, 1);
    assert_eq!(new_loc.x, 0);
    assert_eq!(buf.get_line(0), Some("hello"));
    assert_eq!(buf.get_line(1), Some(" world"));
}
