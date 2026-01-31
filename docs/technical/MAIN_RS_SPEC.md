# Main.rs Specification

Complete reconstruction guide for `src/main.rs`.

> **File Location**: `src/main.rs`  
> **Line Budget**: 270 lines  
> **Purpose**: Application entry point, TTY detection, panic handling, and event loop orchestration

---

## Related Documentation

- [Event Loop Contract](contracts/event_loop.md)
- [Mode Contract](contracts/mode.md)
- [Error Contract](contracts/errors.md)
- [Architecture Reference](ARCHITECTURE.md)

---

## Complete Implementation

```rust
//! bcpuae - A modal text editor in Rust
//! Main application entry point

use std::io::{stdin, stdout, BufRead, Write};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::{error, info, warn};

mod core;
mod operations;
mod system;
mod ui;

use crate::core::Result as EditorResult;
use crate::operations::commands::{Action, CommandMapper, EditorMode};
use crate::operations::file_io;
use crate::system::notifications::{NotificationManager, NotifyLevel};
use crate::system::{Event, EventHandler};
use crate::ui::state::EditorState;
use crate::ui::Renderer;

#[tokio::main]
async fn main() -> Result<()> {
    // Set up panic hook to restore terminal
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Renderer::restore_terminal();
        original_hook(info);
    }));

    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Starting bcpuae");

    // Check if we're in a TTY
    let is_tty = crossterm::tty::IsTty::is_tty(&stdout());
    info!("Running in TTY: {}", is_tty);

    if !is_tty {
        warn!("Not running in a TTY, falling back to simple mode");
        return run_simple_mode().await;
    }

    // Setup terminal
    let mut terminal = match Renderer::setup_terminal() {
        Ok(t) => {
            info!("Terminal setup successful");
            t
        }
        Err(e) => {
            error!("Failed to setup terminal: {}", e);
            eprintln!("Error: Failed to setup terminal: {}", e);
            eprintln!("Falling back to simple mode...");
            return run_simple_mode().await;
        }
    };

    // Initialize notification system
    let mut notification_manager = NotificationManager::init();
    NotificationManager::send("Editor started", NotifyLevel::Info);

    let renderer = Renderer::new();
    let mut event_handler = EventHandler::new();

    // Initialize editor state
    let mut state = EditorState::new();

    info!("Entering main event loop");

    // Main event loop
    let result = run_event_loop(
        &mut state,
        &mut event_handler,
        &renderer,
        &mut terminal,
        &mut notification_manager,
    )
    .await;

    info!("Exiting main event loop");

    // Cleanup
    if let Err(e) = Renderer::restore_terminal() {
        warn!("Failed to restore terminal: {}", e);
    }

    if let Err(e) = result {
        error!("Editor error: {}", e);
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    info!("bcpuae exited successfully");
    Ok(())
}

async fn run_simple_mode() -> Result<()> {
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

        if line.eq_ignore_ascii_case("quit")
            || line.eq_ignore_ascii_case("exit")
            || line.eq_ignore_ascii_case("q")
        {
            println!("Goodbye!");
            break;
        }

        if line.eq_ignore_ascii_case("save") || line.eq_ignore_ascii_case("w") {
            println!("Save not available in simple mode");
            print!("> ");
            stdout().flush()?;
            continue;
        }

        if line.eq_ignore_ascii_case("help") || line.eq_ignore_ascii_case("h") {
            println!("Commands: quit, help");
            print!("> ");
            stdout().flush()?;
            continue;
        }

        // Insert the line into the buffer
        for ch in line.chars() {
            let _ = state
                .document
                .buffer_mut()
                .insert_char(&state.cursor, ch);
            state.cursor.x += 1;
        }
        let _ = state.document.buffer_mut().insert_newline(&state.cursor);
        state.cursor.y += 1;
        state.cursor.x = 0;

        println!("Added: {}", line);
        print!("> ");
        stdout().flush()?;
    }

    println!();
    println!("Document contains {} lines:", state.document.buffer().line_count());
    for (i, line) in state.document.buffer().lines().iter().enumerate() {
        println!("  {}: {}", i + 1, line);
    }

    Ok(())
}

async fn run_event_loop(
    state: &mut EditorState,
    event_handler: &mut EventHandler,
    renderer: &Renderer,
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    notification_manager: &mut NotificationManager,
) -> Result<()> {
    loop {
        // Update notifications (expire old ones)
        notification_manager.update();

        // Draw UI
        if let Err(e) = renderer.draw(terminal, state, notification_manager.active()) {
            error!("Render error: {}", e);
            // Continue on render error
        }

        // Wait for event
        match event_handler.next().await {
            Some(event) => {
                match event {
                    Event::Key(key) => {
                        if let Err(e) = handle_key_event(key, state, notification_manager).await {
                            NotificationManager::send(format!("Error: {}", e), NotifyLevel::Fatal);
                        }
                    }
                    Event::Tick => {
                        // Periodic refresh happens via redraw
                    }
                }
            }
            None => {
                warn!("Event channel closed, exiting");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_key_event(
    key: KeyEvent,
    state: &mut EditorState,
    _notification_manager: &mut NotificationManager,
) -> EditorResult<()> {
    let action = CommandMapper::map(key, state.mode);

    match action {
        Action::Quit => {
            info!("Quit action received");
            std::process::exit(0);
        }
        Action::Save => {
            if let Some(path) = state.document.file_path() {
                let content = state.document.buffer().to_string();
                file_io::write_file(path, &content).await?;
                state.document.mark_clean();
                NotificationManager::send("File saved", NotifyLevel::Success);
            } else {
                NotificationManager::send("No file name (use save_as)", NotifyLevel::Warning);
            }
        }
        Action::EnterMode(mode) => {
            state.set_mode(mode);
        }
        Action::MoveLeft => state.move_cursor(-1, 0),
        Action::MoveRight => state.move_cursor(1, 0),
        Action::MoveUp => state.move_cursor(0, -1),
        Action::MoveDown => state.move_cursor(0, 1),
        Action::MoveStartOfLine => state.move_start_of_line(),
        Action::MoveEndOfLine => state.move_end_of_line(),
        Action::InsertChar(ch) => {
            if state.mode == EditorMode::Insert {
                let new_loc = state.document.buffer_mut().insert_char(&state.cursor, ch)?;
                state.cursor = new_loc;
                state.document.mark_dirty();
            }
        }
        Action::DeleteChar => {
            if state.mode == EditorMode::Insert {
                let new_loc = state.document.buffer_mut().delete_char(&state.cursor)?;
                state.cursor = new_loc;
                state.document.mark_dirty();
            }
        }
        Action::InsertNewline => {
            if state.mode == EditorMode::Insert {
                let new_loc = state.document.buffer_mut().insert_newline(&state.cursor)?;
                state.cursor = new_loc;
                state.document.mark_dirty();
            }
        }
        Action::NoOp => {}
    }

    // BUG: Uses hardcoded rect instead of actual layout
    state.scroll_to_cursor(&ratatui::layout::Rect::new(0, 0, 80, 24));

    Ok(())
}
```

---

## Module Declarations

```rust
mod core;
mod operations;
mod system;
mod ui;
```

Rust automatically resolves:
- `core.rs` OR `core/mod.rs` → Uses `core/mod.rs`
- `operations.rs` OR `operations/mod.rs` → Uses `operations/mod.rs`
- `system.rs` OR `system/mod.rs` → Uses `system/mod.rs`
- `ui.rs` OR `ui/mod.rs` → Uses `ui/mod.rs`

---

## Type Aliases

```rust
use crate::core::Result as EditorResult;
```

Creates distinct alias to avoid confusion with `anyhow::Result`:
- `Result<T>` (from anyhow) = `std::result::Result<T, anyhow::Error>`
- `EditorResult<T>` = `std::result::Result<T, EditorError>`

---

## Panic Hook (Lines 26-30)

```rust
let original_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |info| {
    let _ = Renderer::restore_terminal();
    original_hook(info);
}));
```

**Purpose**: Ensure terminal is restored to canonical mode even if panic occurs.

**Mechanism**:
1. Save original panic hook
2. Set custom hook that:
   - Calls `Renderer::restore_terminal()` (ignore result with `let _ =`)
   - Chains to original hook for default panic behavior

**Without This**: Terminal stays in raw mode, alternate screen on panic, user sees garbled display.

---

## TTY Detection (Lines 37-56)

```rust
let is_tty = crossterm::tty::IsTty::is_tty(&stdout());
```

**Path A: TTY Detected**
- Attempt terminal setup via `Renderer::setup_terminal()`
- If setup fails, fall back to simple mode
- If setup succeeds, run full TUI

**Path B: No TTY** (piped input, CI environment, etc.)
- Skip terminal manipulation entirely
- Run `run_simple_mode()` - line-based stdin/stdout interface

**Why**: Running TUI without TTY causes immediate error or garbled output.

---

## Simple Mode (Lines 98-179)

**Use Cases**:
- Docker without `-t` flag
- CI/CD pipelines
- Scripting/Automation
- Debugging

**Interface**:
```
> Hello world
Added: Hello world
> This is a test
Added: This is a test
> quit
Goodbye!

Document contains 2 lines:
  1: Hello world
  2: This is a test
```

**Commands**:
| Input | Action |
|-------|--------|
| `quit`, `exit`, `q` | Exit simple mode |
| `save`, `w` | Print "Save not available" |
| `help`, `h` | Print command list |
| Any other text | Add as line to buffer |

**Line Processing**:
1. Read line from stdin
2. Check for commands (case-insensitive)
3. If not command:
   - Insert each character at cursor
   - Insert newline at end
   - Move cursor to next line
   - Print confirmation

---

## Main Event Loop (Lines 183-220)

See [Event Loop Contract](contracts/event_loop.md).

```rust
async fn run_event_loop(...) -> Result<()> {
    loop {
        notification_manager.update();     // Expire old notifications
        renderer.draw(...)?;                // Draw UI
        
        match event_handler.next().await {  // Wait for event
            Some(Event::Key(key)) => {
                handle_key_event(key, ...).await?;
            }
            Some(Event::Tick) => {}        // Just triggers redraw
            None => break,                  // Channel closed, exit
        }
    }
    Ok(())
}
```

**Update → Draw → Wait Cycle**:
1. `update()` - Process new notifications, expire old ones
2. `draw()` - Render editor, notifications, status bar
3. `next().await` - Block until key pressed or tick (250ms)

**Error Handling**:
- Render errors: Logged with `error!` macro, loop continues
- Key handling errors: Sent as Fatal notification, loop continues
- Channel closed (None): Break loop, exit cleanly

---

## Key Event Handler (Lines 224-282)

See [Mode Contract](contracts/mode.md).

**Structure**:
```rust
async fn handle_key_event(key, state, _nm) -> EditorResult<()> {
    let action = CommandMapper::map(key, state.mode);
    
    match action {
        Action::Quit => std::process::exit(0),
        Action::Save => { /* async file write */ },
        Action::EnterMode(mode) => state.set_mode(mode),
        Action::MoveX => state.move_cursor(...),
        Action::InsertChar(ch) => { /* buffer insert */ },
        // ... etc
    }
    
    state.scroll_to_cursor(&Rect::new(0, 0, 80, 24));  // BUG: hardcoded
    Ok(())
}
```

### Action Handling Details

**Save Action**:
```rust
Action::Save => {
    if let Some(path) = state.document.file_path() {
        let content = state.document.buffer().to_string();
        file_io::write_file(path, &content).await?;  // Async write
        state.document.mark_clean();
        NotificationManager::send("File saved", NotifyLevel::Success);
    } else {
        NotificationManager::send("No file name...", NotifyLevel::Warning);
    }
}
```

**InsertChar Action**:
```rust
Action::InsertChar(ch) => {
    if state.mode == EditorMode::Insert {  // Only in Insert mode
        let new_loc = state.document.buffer_mut().insert_char(&state.cursor, ch)?;
        state.cursor = new_loc;  // Update cursor to new position
        state.document.mark_dirty();  // Mark for save prompt
    }
}
```

**Mode Check**: InsertChar, DeleteChar, InsertNewline only work in Insert mode.

### Known Bug - Hardcoded Rect (Line 276)

```rust
state.scroll_to_cursor(&ratatui::layout::Rect::new(0, 0, 80, 24));
```

**Problem**: Uses fixed 80x24 terminal size instead of actual layout dimensions.

**Fix for Reconstruction**:
```rust
// Pass actual editor area to scroll_to_cursor
let layout = Layout::calculate(terminal.size()?);
state.scroll_to_cursor(&layout.editor);
```

Or modify scroll_to_cursor to accept layout:
```rust
state.scroll_to_cursor(&Layout::calculate(terminal.size()?));
```

---

## Reconstruction Improvements

### 1. Command Execution Support

Add to handle_key_event:
```rust
Action::ExecuteCommand(cmd) => {
    execute_command(&cmd, state).await;
}
Action::ForceQuit => {
    std::process::exit(0);
}
Action::QuitWithSaveCheck => {
    if state.document.is_dirty() {
        NotificationManager::send(
            "No write since last change (add ! to override)",
            NotifyLevel::Warning
        );
    } else {
        std::process::exit(0);
    }
}
```

Add function:
```rust
async fn execute_command(cmd: &str, state: &mut EditorState) {
    let cmd = cmd.trim();
    
    match cmd {
        "q" | "quit" => {
            if state.document.is_dirty() {
                NotificationManager::send(
                    "No write since last change (add ! to override)",
                    NotifyLevel::Warning
                );
            } else {
                std::process::exit(0);
            }
        }
        "q!" | "quit!" => std::process::exit(0),
        "w" | "write" => {
            // Trigger save
        }
        "wq" | "x" => {
            // Save and quit
        }
        cmd if cmd.starts_with("w ") => {
            let filename = cmd[2..].trim();
            // Save as filename
        }
        cmd if cmd.starts_with("e ") => {
            let filename = cmd[2..].trim();
            // Open file
        }
        _ => {
            if let Ok(line_num) = cmd.parse::<usize>() {
                state.cursor.y = line_num.saturating_sub(1);
            } else {
                NotificationManager::send(
                    format!("Unknown command: {}", cmd),
                    NotifyLevel::Warning
                );
            }
        }
    }
}
```

### 2. CLI Argument Support

Add to main() before TTY check:
```rust
let args: Vec<String> = std::env::args().collect();
let file_arg = args.get(1).map(|s| s.as_str());

// Initialize state with file if provided
let mut state = if let Some(path) = file_arg {
    match EditorState::open(std::path::Path::new(path)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            EditorState::new()
        }
    }
} else {
    EditorState::new()
};
```

### 3. Proper Scroll Rect

Modify run_event_loop to pass layout:
```rust
async fn run_event_loop(...) -> Result<()> {
    loop {
        notification_manager.update();
        
        // Calculate layout once per frame
        let layout = Layout::calculate(terminal.size()?);
        
        if let Err(e) = renderer.draw(terminal, state, notification_manager.active()) {
            error!("Render error: {}", e);
        }
        
        match event_handler.next().await {
            Some(Event::Key(key)) => {
                if let Err(e) = handle_key_event(key, state, &layout, notification_manager).await {
                    NotificationManager::send(format!("Error: {}", e), NotifyLevel::Fatal);
                }
            }
            // ...
        }
    }
}
```

Then modify handle_key_event signature:
```rust
async fn handle_key_event(
    key: KeyEvent,
    state: &mut EditorState,
    layout: &Layout,
    _notification_manager: &mut NotificationManager,
) -> EditorResult<()> {
    // ...
    state.scroll_to_cursor(&layout.editor);  // Use actual editor area
    Ok(())
}
```

---

## Reconstruction Verification

After implementing main.rs, verify:
1. Running in TTY: Shows full TUI with alternate screen
2. Running without TTY (pipe): Shows simple mode banner
3. Panic during TUI: Terminal restored (test with intentional panic)
4. Ctrl+Q in Normal mode: Exits immediately
5. Ctrl+S with no file path: Shows "No file name" warning
6. Typing in Insert mode: Characters appear, cursor advances
7. Backspace at start of line: Merges with previous line
8. Enter: Creates new line, cursor moves
9. Esc: Returns to Normal mode
10. Line numbers: Visible in editor gutter

---

## Logging Output

With `RUST_LOG=info`, expect:
```
2024-01-15T10:30:00.123 INFO bcpuae: Starting bcpuae
2024-01-15T10:30:00.124 INFO bcpuae: Running in TTY: true
2024-01-15T10:30:00.125 INFO bcpuae: Terminal setup successful
2024-01-15T10:30:00.126 INFO bcpuae: Entering main event loop
2024-01-15T10:30:05.456 INFO bcpuae: Quit action received
2024-01-15T10:30:05.457 INFO bcpuae: Exiting main event loop
2024-01-15T10:30:05.458 INFO bcpuae: bcpuae exited successfully
```

---

## Error Exit Codes

| Exit Code | Condition |
|-----------|-----------|
| 0 | Normal exit (quit, end of input) |
| 1 | Error during execution (logged with error!) |

---

## Import Hierarchy

```
main.rs
├── std::io::{stdin, stdout, BufRead, Write}
├── anyhow::Result
├── crossterm::event::{KeyCode, KeyEvent, KeyModifiers}
├── tracing::{error, info, warn}
├── core (module)
│   ├── Result as EditorResult (type alias)
│   ├── Buffer, Document, Location, EditorError
├── operations (module)
│   ├── commands::{Action, CommandMapper, EditorMode}
│   └── file_io
├── system (module)
│   ├── notifications::{NotificationManager, NotifyLevel}
│   └── {Event, EventHandler}
└── ui (module)
    ├── state::EditorState
    └── Renderer
```
