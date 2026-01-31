//! Command handling for key events

use crossterm::event::KeyEvent;
use tracing::info;

use crate::core::Result as EditorResult;
use crate::operations::commands::{Action, CommandMapper, EditorMode};
use crate::operations::file_io;
use crate::system::notifications::{NotificationManager, NotifyLevel};
use crate::ui::state::EditorState;
use crate::ui::Layout;

/// Handle a key event and execute the corresponding action
pub async fn handle_key_event(
    key: KeyEvent,
    state: &mut EditorState,
    layout: &Layout,
    _nm: &mut NotificationManager,
) -> EditorResult<()> {
    let action = CommandMapper::map(key, &state.mode);

    match action {
        Action::Quit => handle_quit(state),
        Action::ForceQuit => handle_force_quit(),
        Action::Save => {
            if let Err(e) = handle_save(state).await {
                NotificationManager::send(format!("Save failed: {}", e), NotifyLevel::Fatal);
            }
        }
        Action::SaveAs(filename) => {
            if let Err(e) = handle_save_as(state, &filename).await {
                NotificationManager::send(format!("Save failed: {}", e), NotifyLevel::Fatal);
            }
        }
        Action::Open(filename) => handle_open(state, &filename).await,
        Action::ExecuteCommand(cmd) => {
            execute_command(&cmd, state).await;
            state.set_mode(EditorMode::Normal);
        }
        Action::EnterMode(mode) => state.set_mode(mode),
        Action::MoveLeft => state.move_cursor(-1, 0),
        Action::MoveRight => state.move_cursor(1, 0),
        Action::MoveUp => state.move_cursor(0, -1),
        Action::MoveDown => state.move_cursor(0, 1),
        Action::MoveStartOfLine => state.move_start_of_line(),
        Action::MoveEndOfLine => state.move_end_of_line(),
        Action::InsertChar(ch) => handle_insert_char(state, ch),
        Action::DeleteChar => handle_delete_char(state),
        Action::InsertNewline => handle_insert_newline(state),
        Action::NoOp => {}
    }

    state.scroll_to_cursor(&layout.editor);
    Ok(())
}

fn handle_quit(state: &EditorState) {
    info!("Quit action received");
    if state.document.is_dirty() {
        NotificationManager::send(
            "No write since last change (add ! to override)",
            NotifyLevel::Warning,
        );
    } else {
        std::process::exit(0);
    }
}

fn handle_force_quit() {
    info!("Force quit action received");
    std::process::exit(0);
}

async fn handle_save(state: &mut EditorState) -> EditorResult<()> {
    if let Some(path) = state.document.file_path() {
        let content = state.document.buffer().to_string();
        file_io::write_file(path, &content).await?;
        state.document.mark_clean();
        NotificationManager::send("File saved", NotifyLevel::Success);
    } else {
        NotificationManager::send("No file name (use :w <filename>)", NotifyLevel::Warning);
    }
    Ok(())
}

async fn handle_save_as(state: &mut EditorState, filename: &str) -> EditorResult<()> {
    let path = std::path::PathBuf::from(filename);
    let content = state.document.buffer().to_string();
    file_io::write_file(&path, &content).await?;
    state.document.set_file_path(path);
    state.document.mark_clean();
    NotificationManager::send("File saved", NotifyLevel::Success);
    Ok(())
}

async fn handle_open(state: &mut EditorState, filename: &str) {
    let path = std::path::PathBuf::from(filename);
    match EditorState::open(&path).await {
        Ok(new_state) => {
            *state = new_state;
            NotificationManager::send("File opened", NotifyLevel::Success);
        }
        Err(e) => {
            NotificationManager::send(format!("Error opening file: {}", e), NotifyLevel::Fatal);
        }
    }
}

fn handle_insert_char(state: &mut EditorState, ch: char) {
    if state.mode == EditorMode::Insert {
        if let Ok(new_loc) = state.document.buffer_mut().insert_char(&state.cursor, ch) {
            state.cursor = new_loc;
            state.document.mark_dirty();
        }
    }
}

fn handle_delete_char(state: &mut EditorState) {
    if state.mode == EditorMode::Insert {
        if let Ok(new_loc) = state.document.buffer_mut().delete_char(&state.cursor) {
            state.cursor = new_loc;
            state.document.mark_dirty();
        }
    }
}

fn handle_insert_newline(state: &mut EditorState) {
    if state.mode == EditorMode::Insert {
        if let Ok(new_loc) = state.document.buffer_mut().insert_newline(&state.cursor) {
            state.cursor = new_loc;
            state.document.mark_dirty();
        }
    }
}

async fn execute_command(cmd: &str, state: &mut EditorState) {
    let cmd = cmd.trim();

    match cmd {
        "q" | "quit" => handle_quit(state),
        "q!" | "quit!" => handle_force_quit(),
        "w" | "write" => {
            if let Err(e) = handle_save(state).await {
                NotificationManager::send(format!("Save failed: {}", e), NotifyLevel::Fatal);
            }
        }
        "wq" | "x" => handle_save_and_quit(state).await,
        _ if cmd.starts_with("w ") => {
            let filename = cmd[2..].trim();
            if let Err(e) = handle_save_as(state, filename).await {
                NotificationManager::send(format!("Save failed: {}", e), NotifyLevel::Fatal);
            }
        }
        _ if cmd.starts_with("e ") => {
            let filename = cmd[2..].trim();
            handle_open(state, filename).await;
        }
        _ => handle_line_number_or_unknown(cmd, state),
    }
}

async fn handle_save_and_quit(state: &mut EditorState) {
    if let Some(path) = state.document.file_path() {
        let content = state.document.buffer().to_string();
        match file_io::write_file(path, &content).await {
            Ok(()) => {
                state.document.mark_clean();
                std::process::exit(0);
            }
            Err(e) => {
                NotificationManager::send(format!("Save failed: {}", e), NotifyLevel::Fatal);
            }
        }
    } else {
        NotificationManager::send("No file name (use :w <filename>)", NotifyLevel::Warning);
    }
}

fn handle_line_number_or_unknown(cmd: &str, state: &mut EditorState) {
    if let Ok(line_num) = cmd.parse::<usize>() {
        if line_num > 0 {
            let target = line_num.saturating_sub(1);
            let max_line = state.document.buffer().line_count().saturating_sub(1);
            state.cursor.y = target.min(max_line);
            state.cursor.x = 0;
        }
    } else {
        NotificationManager::send(format!("Unknown command: {}", cmd), NotifyLevel::Warning);
    }
}
