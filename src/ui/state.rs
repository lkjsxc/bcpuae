//! Editor state management

use ratatui::layout::Rect;

use crate::core::{Document, Location};
use crate::operations::commands::EditorMode;
use crate::ui::idle::IdleTracker;

/// Complete editor state including document, cursor, and mode
#[derive(Debug)]
pub struct EditorState {
    /// The document being edited
    pub document: Document,
    /// Current cursor position
    pub cursor: Location,
    /// Current editing mode
    pub mode: EditorMode,
    /// Scroll offset (x_scroll, y_scroll)
    pub scroll_offset: (u16, u16),
    /// Idle tracker for sparkle effects
    pub idle_tracker: IdleTracker,
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

impl EditorState {
    /// Create a new editor state with empty document
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: Location::new(0, 0),
            mode: EditorMode::Normal,
            scroll_offset: (0, 0),
            idle_tracker: IdleTracker::new(),
        }
    }

    /// Open a file and create state from it
    pub async fn open(path: &std::path::Path) -> crate::core::Result<Self> {
        let document = Document::open(path).await?;
        Ok(Self {
            document,
            cursor: Location::new(0, 0),
            mode: EditorMode::Normal,
            scroll_offset: (0, 0),
            idle_tracker: IdleTracker::new(),
        })
    }

    /// Set the current mode
    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
        self.idle_tracker.record_activity();
    }

    /// Move cursor by relative amount
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        let line_count = self.document.buffer().line_count();

        // Calculate new Y, clamped to valid range
        let new_y = if dy < 0 {
            self.cursor.y.saturating_sub(dy.unsigned_abs() as usize)
        } else {
            self.cursor.y.saturating_add(dy as usize)
        };
        let new_y = new_y.min(line_count.saturating_sub(1));

        // Calculate new X, clamped to line length
        let line_len = self.document.buffer().line_len(new_y);
        let new_x = if dx < 0 {
            self.cursor.x.saturating_sub(dx.unsigned_abs() as usize)
        } else {
            self.cursor.x.saturating_add(dx as usize)
        };
        let new_x = new_x.min(line_len);

        self.cursor.x = new_x;
        self.cursor.y = new_y;
        self.idle_tracker.record_activity();
    }

    /// Move cursor to start of current line
    pub fn move_start_of_line(&mut self) {
        self.cursor.x = 0;
        self.idle_tracker.record_activity();
    }

    /// Move cursor to end of current line
    pub fn move_end_of_line(&mut self) {
        self.cursor.x = self.document.buffer().line_len(self.cursor.y);
        self.idle_tracker.record_activity();
    }

    /// Move cursor to first line
    pub fn move_first_line(&mut self) {
        self.cursor.y = 0;
        self.cursor.x = 0;
        self.idle_tracker.record_activity();
    }

    /// Move cursor to last line
    pub fn move_last_line(&mut self) {
        let last = self.document.buffer().line_count().saturating_sub(1);
        self.cursor.y = last;
        self.cursor.x = self.document.buffer().line_len(last);
        self.idle_tracker.record_activity();
    }

    /// Scroll viewport to keep cursor visible
    pub fn scroll_to_cursor(&mut self, area: &Rect) {
        let height = area.height as usize;
        let scroll_y = self.scroll_offset.1 as usize;

        if self.cursor.y < scroll_y {
            // Cursor above viewport: align viewport top to cursor
            self.scroll_offset.1 = self.cursor.y as u16;
        } else if self.cursor.y >= scroll_y.saturating_add(height) {
            // Cursor below viewport: align viewport bottom to cursor
            self.scroll_offset.1 = self.cursor.y.saturating_sub(height.saturating_sub(1)) as u16;
        }
    }

    /// Get the start line for rendering based on scroll offset
    pub fn start_line(&self) -> usize {
        self.scroll_offset.1 as usize
    }

    /// Update idle state (call periodically)
    pub fn update_idle(&mut self) {
        self.idle_tracker.update();
    }
}

#[cfg(test)]
mod tests;
