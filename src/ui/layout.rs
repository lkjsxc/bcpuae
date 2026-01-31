//! Screen region calculations

use ratatui::layout::{Constraint, Direction, Rect};

/// Layout regions for the editor UI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Layout {
    /// Main text editor area
    pub editor: Rect,
    /// Notification area (3 rows)
    pub notifications: Rect,
    /// Status bar area (1 row)
    pub status_bar: Rect,
}

impl Layout {
    /// Calculate layout from terminal size
    pub fn calculate(area: Rect) -> Self {
        let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Editor gets remaining space
                Constraint::Length(3), // Notifications fixed at 3
                Constraint::Length(1), // Status bar fixed at 1
            ])
            .split(area);

        Self {
            editor: chunks[0],
            notifications: chunks[1],
            status_bar: chunks[2],
        }
    }

    /// Calculate the editor viewport dimensions
    pub fn editor_viewport(&self) -> (u16, u16) {
        // Account for line number gutter (6 chars: " 123 │ ")
        let gutter_width = 6;
        let width = self.editor.width.saturating_sub(gutter_width);
        let height = self.editor.height;
        (width, height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_calculation() {
        let area = Rect::new(0, 0, 80, 24);
        let layout = Layout::calculate(area);

        // Editor should get remaining space: 24 - 3 - 1 = 20
        assert_eq!(layout.editor.height, 20);
        assert_eq!(layout.editor.width, 80);

        // Notifications fixed at 3 rows
        assert_eq!(layout.notifications.height, 3);

        // Status bar fixed at 1 row
        assert_eq!(layout.status_bar.height, 1);
    }

    #[test]
    fn test_editor_viewport() {
        let area = Rect::new(0, 0, 80, 24);
        let layout = Layout::calculate(area);
        let (width, height) = layout.editor_viewport();

        // 80 - 6 (gutter) = 74
        assert_eq!(width, 74);
        assert_eq!(height, 20);
    }
}
