//! Terminal rendering using ratatui

use crossterm::cursor::{SetCursorStyle, Show};
use crossterm::execute;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};

use crate::operations::commands::EditorMode;
use crate::system::notifications::Notification;
use crate::ui::draw::{draw_editor, draw_notifications, draw_status_bar};
use crate::ui::layout::Layout;
use crate::ui::state::EditorState;
use crate::ui::theme::Theme;

/// Terminal renderer
#[derive(Debug)]
pub struct Renderer {
    theme: Theme,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    /// Create a new renderer with default theme
    pub fn new() -> Self {
        Self {
            theme: Theme::new(),
        }
    }

    /// Setup terminal for TUI mode
    pub fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
        use crossterm::{
            event::EnableMouseCapture,
            execute,
            terminal::{enable_raw_mode, EnterAlternateScreen},
        };
        use std::io::stdout;

        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(terminal)
    }

    /// Restore terminal to canonical mode
    pub fn restore_terminal() -> anyhow::Result<()> {
        use crossterm::{
            event::DisableMouseCapture,
            execute,
            terminal::{disable_raw_mode, LeaveAlternateScreen},
        };
        use std::io::stdout;

        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    /// Draw the complete UI
    pub fn draw(
        &self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
        state: &EditorState,
        notifications: Vec<Notification>,
    ) -> anyhow::Result<()> {
        let theme = &self.theme;
        terminal.draw(|frame| {
            let layout = Layout::calculate(frame.size());

            draw_editor(frame, state, layout.editor, theme);
            draw_notifications(frame, &notifications, layout.notifications, theme);
            draw_status_bar(frame, state, layout.status_bar, theme);

            // Position cursor in editor
            if !matches!(state.mode, EditorMode::Command { .. }) {
                let start_line = state.start_line();
                let cursor_x = layout.editor.x + 6 + state.cursor.x as u16;
                let cursor_y = layout.editor.y
                    + state.cursor.y.saturating_sub(start_line) as u16;
                frame.set_cursor(cursor_x, cursor_y);
            }
        })?;

        // Apply cursor style based on mode
        self.apply_cursor_style(&state.mode)?;

        Ok(())
    }

    /// Apply cursor style based on editor mode
    fn apply_cursor_style(&self, mode: &EditorMode) -> anyhow::Result<()> {
        let style = match mode {
            EditorMode::Normal => SetCursorStyle::SteadyBlock,
            EditorMode::Insert => SetCursorStyle::SteadyBar,
            EditorMode::Command { .. } => SetCursorStyle::SteadyBar,
        };

        execute!(stdout(), style, Show)?;
        Ok(())
    }
}
