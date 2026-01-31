//! Command mapping and action definitions

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Editor modes
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EditorMode {
    /// Normal mode for navigation
    #[default]
    Normal,
    /// Insert mode for text entry
    Insert,
    /// Command mode for ex commands
    Command { buffer: String },
}

impl std::fmt::Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorMode::Normal => write!(f, "NORMAL"),
            EditorMode::Insert => write!(f, "INSERT"),
            EditorMode::Command { .. } => write!(f, "COMMAND"),
        }
    }
}

/// Possible actions from user input
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Move cursor left
    MoveLeft,
    /// Move cursor right
    MoveRight,
    /// Move cursor up
    MoveUp,
    /// Move cursor down
    MoveDown,
    /// Move to start of line
    MoveStartOfLine,
    /// Move to end of line
    MoveEndOfLine,
    /// Insert a character
    InsertChar(char),
    /// Delete character before cursor
    DeleteChar,
    /// Insert newline
    InsertNewline,
    /// Enter a specific mode
    EnterMode(EditorMode),
    /// Save the file
    Save,
    /// Save the file as
    SaveAs(String),
    /// Quit the editor
    Quit,
    /// Force quit without saving
    ForceQuit,
    /// Open a file
    Open(String),
    /// Execute a command
    ExecuteCommand(String),
    /// No operation
    NoOp,
}

/// Maps key events to actions based on current mode
#[derive(Debug, Default)]
pub struct CommandMapper;

impl CommandMapper {
    /// Map a key event to an action based on the current mode
    pub fn map(key: KeyEvent, mode: &EditorMode) -> Action {
        match mode {
            EditorMode::Normal => Self::map_normal(key),
            EditorMode::Insert => Self::map_insert(key),
            EditorMode::Command { buffer } => Self::map_command(key, buffer.clone()),
        }
    }

    /// Map keys in Normal mode
    fn map_normal(key: KeyEvent) -> Action {
        match key.code {
            // Navigation
            KeyCode::Char('h') | KeyCode::Left => Action::MoveLeft,
            KeyCode::Char('j') | KeyCode::Down => Action::MoveDown,
            KeyCode::Char('k') | KeyCode::Up => Action::MoveUp,
            KeyCode::Char('l') | KeyCode::Right => Action::MoveRight,
            KeyCode::Char('0') => Action::MoveStartOfLine,
            KeyCode::Char('$') => Action::MoveEndOfLine,

            // Mode entry
            KeyCode::Char('i') => Action::EnterMode(EditorMode::Insert),
            KeyCode::Char(':') => Action::EnterMode(EditorMode::Command {
                buffer: String::new(),
            }),

            // File operations
            KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Save,
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,

            _ => Action::NoOp,
        }
    }

    /// Map keys in Insert mode
    fn map_insert(key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Esc => Action::EnterMode(EditorMode::Normal),
            KeyCode::Char(c) => Action::InsertChar(c),
            KeyCode::Backspace => Action::DeleteChar,
            KeyCode::Enter => Action::InsertNewline,
            _ => Action::NoOp,
        }
    }

    /// Map keys in Command mode
    fn map_command(key: KeyEvent, buffer: String) -> Action {
        match key.code {
            KeyCode::Esc => Action::EnterMode(EditorMode::Normal),
            KeyCode::Enter => {
                let cmd = buffer.trim().to_string();
                Action::ExecuteCommand(cmd)
            }
            KeyCode::Char(c) => {
                let mut new_buffer = buffer;
                new_buffer.push(c);
                Action::EnterMode(EditorMode::Command { buffer: new_buffer })
            }
            KeyCode::Backspace => {
                let mut new_buffer = buffer;
                new_buffer.pop();
                Action::EnterMode(EditorMode::Command { buffer: new_buffer })
            }
            _ => Action::NoOp,
        }
    }
}

#[cfg(test)]
mod tests;
