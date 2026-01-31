//! Simple non-TTY mode for line-based editing

use std::io::{stdin, stdout, BufRead, Write};

use anyhow::Result;

use crate::ui::state::EditorState;

/// Run simple line-based mode when TTY is not available
pub async fn run_simple_mode() -> Result<()> {
    println!("╔═══════════════════════════════════════╗");
    println!("║     bcpuae - Simple Mode        ║");
    println!("╚═══════════════════════════════════════╝");
    println!();
    println!("Not running in a terminal (TTY).");
    println!();
    println!("To use the full TUI editor, run with:");
    println!("  docker run -it --rm bcpuae");
    println!();
    println!("Available commands:");
    println!("  - Type text and press Enter to add lines");
    println!("  - Press Ctrl+D or type 'quit' to exit");
    println!();

    let mut state = EditorState::new();
    let stdin = stdin();

    print!("> ");
    stdout().flush()?;

    for line in stdin.lock().lines() {
        let line = line?;

        if is_quit_command(&line) {
            println!("Goodbye!");
            break;
        }

        if is_save_command(&line) {
            println!("Save not available in simple mode");
            print_prompt()?;
            continue;
        }

        if is_help_command(&line) {
            println!("Commands: quit, help");
            print_prompt()?;
            continue;
        }

        // Insert the line into the buffer
        insert_line(&mut state, &line);

        println!("Added: {}", line);
        print_prompt()?;
    }

    print_summary(&state);

    Ok(())
}

fn is_quit_command(line: &str) -> bool {
    line.eq_ignore_ascii_case("quit")
        || line.eq_ignore_ascii_case("exit")
        || line.eq_ignore_ascii_case("q")
}

fn is_save_command(line: &str) -> bool {
    line.eq_ignore_ascii_case("save") || line.eq_ignore_ascii_case("w")
}

fn is_help_command(line: &str) -> bool {
    line.eq_ignore_ascii_case("help") || line.eq_ignore_ascii_case("h")
}

fn print_prompt() -> Result<()> {
    print!("> ");
    stdout().flush()?;
    Ok(())
}

fn insert_line(state: &mut EditorState, line: &str) {
    for ch in line.chars() {
        let _ = state.document.buffer_mut().insert_char(&state.cursor, ch);
        state.cursor.x += 1;
    }
    let _ = state.document.buffer_mut().insert_newline(&state.cursor);
    state.cursor.y += 1;
    state.cursor.x = 0;
}

fn print_summary(state: &EditorState) {
    println!();
    println!(
        "Document contains {} lines:",
        state.document.buffer().line_count()
    );
    for (i, line) in state.document.buffer().lines().iter().enumerate() {
        println!("  {}: {}", i + 1, line);
    }
}
