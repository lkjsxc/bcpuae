//! Color theme definitions

use ratatui::style::Color;

use crate::system::notifications::NotifyLevel;

/// Color theme for the editor UI
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    /// Background color
    pub bg: Color,
    /// Foreground (text) color
    pub fg: Color,
    /// Accent color for highlights
    pub accent: Color,
    /// Status bar background
    pub status_bg: Color,
    /// Status bar foreground
    pub status_fg: Color,
    /// Line number gutter color
    pub line_number_fg: Color,
    /// Notification colors: [Info, Success, Warning, Fatal]
    notification_colors: [Color; 4],
    /// Mode badge colors: [Normal, Insert, Command]
    mode_colors: [Color; 3],
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Black,
            fg: Color::White,
            accent: Color::Cyan,
            status_bg: Color::DarkGray,
            status_fg: Color::White,
            line_number_fg: Color::Gray,
            notification_colors: [Color::Blue, Color::Green, Color::Yellow, Color::Red],
            mode_colors: [Color::Cyan, Color::Green, Color::Yellow],
        }
    }
}

impl Theme {
    /// Create a new theme with default colors
    pub fn new() -> Self {
        Self::default()
    }

    /// Get color for a notification level
    pub fn notify_color(&self, level: &NotifyLevel) -> Color {
        match level {
            NotifyLevel::Info => self.notification_colors[0],
            NotifyLevel::Success => self.notification_colors[1],
            NotifyLevel::Warning => self.notification_colors[2],
            NotifyLevel::Fatal => self.notification_colors[3],
        }
    }

    /// Get color for a mode
    pub fn mode_color(&self, mode: &str) -> Color {
        match mode {
            "NORMAL" => self.mode_colors[0],
            "INSERT" => self.mode_colors[1],
            "COMMAND" => self.mode_colors[2],
            _ => self.accent,
        }
    }
}
