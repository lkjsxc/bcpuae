//! Cursor location representation

/// Represents a cursor position in the buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Location {
    /// Global character index (reserved for future use)
    pub g_index: usize,
    /// Column position (0-indexed, character-based)
    pub x: usize,
    /// Line position (0-indexed)
    pub y: usize,
}

impl Location {
    /// Create a new location at the specified position
    pub fn new(x: usize, y: usize) -> Self {
        Self { g_index: 0, x, y }
    }
}
