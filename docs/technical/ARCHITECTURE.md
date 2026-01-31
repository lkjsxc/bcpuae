# Architecture Reference

Complete technical reference for bcpuae.

> **Purpose**: Machine-parseable reconstruction reference. See also [System Contracts](contracts/README.md) for invariants.

---

## Module Graph

```
                    ┌─────────────────────────┐
                    │       main.rs           │
                    │  (entry, event loop)    │
                    └───────────┬─────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        ▼                       ▼                       ▼
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│     core      │      │  operations   │      │     ui        │
│   (data)      │◀────▶│   (logic)     │◀────▶│  (render)     │
│               │      │               │      │               │
│ • Buffer      │      │ • CommandMap  │      │ • Renderer    │
│ • Document    │      │ • file_io     │      │ • EditorState │
│ • Location    │      │               │      │ • Layout      │
│ • EditorError │      │               │      │ • Theme       │
└───────┬───────┘      └───────────────┘      └───────────────┘
        │
        ▼
┌───────────────┐
│    system     │
│ (infrastructure)
│               │
│ • EventHandler│
│ • Notification│
│   Manager     │
└───────────────┘
```

---

## Type Inventory

### Core Types (`src/core/`)

See [Buffer Contract](../technical/contracts/buffer.md), [Error Contract](../technical/contracts/errors.md).

```rust
// error.rs
pub enum EditorError {
    IO(#[from] std::io::Error),
    Bounds(String),
    InvalidUtf8,
    Channel(String),
}
pub type Result<T> = std::result::Result<T, EditorError>;

// location.rs
pub struct Location {
    pub g_index: usize,  // Unused in current impl (reserved)
    pub x: usize,        // Column (0-indexed)
    pub y: usize,        // Line (0-indexed)
}

// buffer.rs
pub struct Buffer {
    lines: Vec<String>,  // No trailing newlines stored
}

// document.rs
pub struct Document {
    buffer: Buffer,
    file_path: Option<PathBuf>,
    dirty: bool,
}
```

### System Types (`src/system/`)

```rust
// events.rs
pub enum Event {
    Key(KeyEvent),  // From crossterm
    Tick,           // Every 250ms
}
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
}

// notifications.rs
pub enum NotifyLevel { Info, Success, Warning, Fatal }
pub struct Notification {
    pub level: NotifyLevel,
    pub message: String,
    pub timestamp: Instant,
    pub ttl: Duration,
}
pub struct NotificationManager {
    rx: mpsc::UnboundedReceiver<Notification>,
    active: Vec<Notification>,
    history: Vec<Notification>,
}
```

### UI Types (`src/ui/`)

```rust
// theme.rs
pub struct Theme {
    pub bg: Color,           // Black
    pub fg: Color,           // White
    pub accent: Color,       // Cyan
    pub status_bg: Color,    // DarkGray
    pub status_fg: Color,    // White
    pub line_number_fg: Color, // Gray
    pub notification_colors: HashMap<NotifyLevel, Color>,
}

// layout.rs
pub struct Layout {
    pub editor: Rect,        // Main text area
    pub notifications: Rect, // 3 rows
    pub status_bar: Rect,    // 1 row
}

// state.rs
pub struct EditorState {
    pub document: Document,
    pub cursor: Location,
    pub mode: EditorMode,
    pub scroll_offset: (u16, u16),  // (x_scroll, y_scroll)
}

// renderer.rs
pub struct Renderer {
    theme: Theme,
}
```

### Operations Types (`src/operations/`)

See [Mode Contract](../technical/contracts/mode.md).

```rust
// commands.rs
pub enum EditorMode { Normal, Insert, Command }
pub enum Action {
    MoveLeft, MoveRight, MoveUp, MoveDown,
    MoveStartOfLine, MoveEndOfLine,
    InsertChar(char), DeleteChar, InsertNewline,
    EnterMode(EditorMode), Save, Quit, NoOp,
}
pub struct CommandMapper;
```

---

## Algorithm Specifications

### Buffer Line Splitting

See [Buffer Contract](../technical/contracts/buffer.md).

```rust
pub fn from_str(content: &str) -> Self {
    if content.is_empty() {
        return Self::new();  // Returns vec![String::new()]
    }
    // lines() strips newlines
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    // Handle trailing newline: lines() drops it, we need empty final line
    let lines = if content.ends_with('\n') {
        let mut lines = lines;
        lines.push(String::new());
        lines
    } else {
        lines
    };
    Self { lines }
}
```

**Invariant**: Buffer always has ≥1 line. Empty file = vec![""].

### Character Insertion with UTF-8

```rust
pub fn insert_char(&mut self, loc: &Location, ch: char) -> Result<Location> {
    let line = self.lines.get_mut(loc.y).ok_or_else(|| ...)?;
    // Convert char index to byte index
    let byte_idx = line.char_indices()
        .nth(loc.x)
        .map(|(i, _)| i)
        .unwrap_or(line.len());
    line.insert(byte_idx, ch);
    Ok(Location::new(loc.x + 1, loc.y))
}
```

**Key Insight**: `char_indices()` yields (byte_index, char). We need byte index for `String::insert()`.

### Line Merge on Backspace

```rust
pub fn delete_char(&mut self, loc: &Location) -> Result<Location> {
    if loc.x == 0 {
        if loc.y == 0 {
            return Ok(*loc);  // Document start, nothing to do
        }
        // Remove current line, append to previous
        let current_line = self.lines.remove(loc.y);
        let prev_line = &mut self.lines[loc.y - 1];
        let prev_len = prev_line.chars().count();
        prev_line.push_str(&current_line);
        return Ok(Location::new(prev_len, loc.y - 1));
    }
    // ... normal character deletion ...
}
```

### Scroll Calculation

See [Rendering Contract](../technical/contracts/rendering.md).

```rust
pub fn scroll_to_cursor(&mut self, area: &Rect) {
    let height = area.height as usize;
    let scroll_y = self.scroll_offset.1 as usize;
    
    if self.cursor.y < scroll_y {
        // Cursor above viewport: align viewport top to cursor
        self.scroll_offset.1 = self.cursor.y as u16;
    } else if self.cursor.y >= scroll_y + height {
        // Cursor below viewport: align viewport bottom to cursor
        self.scroll_offset.1 = (self.cursor.y.saturating_sub(height - 1)) as u16;
    }
}
```

**Known Issue**: Called with hardcoded Rect(0,0,80,24) in main.rs instead of actual layout.

### Async Event Polling

See [Event Loop Contract](../technical/contracts/event_loop.md).

```rust
tokio::spawn(async move {
    let mut reader = crossterm::event::EventStream::new();
    let mut tick = interval(Duration::from_millis(250));

    loop {
        let tick_delay = tick.tick();
        let crossterm_event = reader.next().fuse();

        tokio::select! {
            _ = tick_delay => {
                if tx.send(Event::Tick).is_err() { break; }
            }
            result = crossterm_event => {
                match result {
                    Some(Ok(CrosstermEvent::Key(key))) => {
                        if tx.send(Event::Key(key)).is_err() { break; }
                    }
                    Some(Err(e)) => error!("Crossterm error: {}", e),
                    None => break,
                }
            }
        }
    }
});
```

**Critical**: `reader.next().fuse()` required for tokio::select! compatibility.

### Notification Expiration

```rust
pub fn update(&mut self) {
    // 1. Collect new from channel (non-blocking)
    while let Ok(notif) = self.rx.try_recv() {
        self.active.push(notif);
    }
    // 2. Partition active into (still_active, expired)
    let now = Instant::now();
    let mut still_active = Vec::new();
    for notif in self.active.drain(..) {
        if now.duration_since(notif.timestamp) > notif.ttl {
            self.history.push(notif);
        } else {
            still_active.push(notif);
        }
    }
    self.active = still_active;
}
```

### Cursor Position Calculation

```rust
// Line number gutter width: " 123 │ " = 6 chars
//   "{:>4}" = 4 chars for line number (right-aligned)
//   " "     = 1 space
//   "│"     = 1 box-drawing char
//   " "     = 1 space
let cursor_x = layout.editor.x + 6 + state.cursor.x as u16;
let cursor_y = layout.editor.y + 
               (state.cursor.y.saturating_sub(start_line)) as u16;
frame.set_cursor(cursor_x, cursor_y);
```

---

## Dependency Graph

### Cargo.toml

```toml
tokio = { version = "1.35", features = ["full"] }
  └─ Used in: events.rs (interval, spawn, select!)
     Used in: file_io.rs (tokio::fs)
     Used in: notifications.rs (mpsc)
     Used in: main.rs (#[tokio::main])

ratatui = "0.24"
  └─ Used in: layout.rs (Rect, Layout, Constraint)
     Used in: renderer.rs (Terminal, Frame, Paragraph, Block, etc.)
     Used in: state.rs (Rect)

crossterm = { version = "0.27", features = ["event-stream"] }
  └─ Used in: events.rs (EventStream)
     Used in: renderer.rs (EnterAlternateScreen, raw_mode, etc.)
     Used in: main.rs (IsTty)

anyhow = "1.0"
  └─ Used in: main.rs (Result type)
     Used in: renderer.rs (Result type in setup/restore)

thiserror = "1.0"
  └─ Used in: error.rs (#[derive(Error)])

tracing = "0.1"
  └─ Used in: main.rs (info!, warn!, error!)
     Used in: events.rs (debug!, warn!, error!)

tracing-subscriber = { version = "0.3", features = ["env-filter"] }
  └─ Used in: main.rs (tracing_subscriber::fmt::init())

futures = "0.3"
  └─ Used in: events.rs (FutureExt, StreamExt for fuse())
```

---

## Control Flow

### Application Startup

```
main()
│
├─► std::panic::set_hook()          // Ensure terminal restore on panic
│
├─► tracing_subscriber::fmt::init()  // Initialize logging
│
├─► crossterm::tty::IsTty::is_tty(&stdout())
│   │
│   ├─► false ──► run_simple_mode()  // Line-based stdin interface
│   │
│   └─► true
│       │
│       ├─► Renderer::setup_terminal()
│       │   ├─► EnterAlternateScreen
│       │   ├─► EnableMouseCapture
│       │   ├─► enable_raw_mode()
│       │   └─► Terminal::new(backend)
│       │
│       ├─► NotificationManager::init()
│       │   └─► SENDER.set(tx)       // Initialize global sender
│       │
│       ├─► EventHandler::new()
│       │   └─► tokio::spawn()       // Spawn event reading task
│       │
│       ├─► EditorState::new()       // Empty document, cursor at (0,0)
│       │
│       └─► run_event_loop()         // Main loop
│
└─► Renderer::restore_terminal()     // Cleanup on exit
    ├─► disable_raw_mode()
    ├─► DisableMouseCapture
    └─► LeaveAlternateScreen
```

### Event Loop Iteration

See [Event Loop Contract](../technical/contracts/event_loop.md).

```
run_event_loop() iteration
│
├─► notification_manager.update()
│   ├─► rx.try_recv() - drain channel
│   └─► Expire old notifications (now - timestamp > ttl)
│
├─► renderer.draw(terminal, state, active_notifications)
│   ├─► Layout::calculate(frame.size())
│   ├─► draw_editor()    // Line numbers + content
│   ├─► draw_notifications() // Status messages
│   ├─► draw_status_bar()    // Mode + filename
│   └─► frame.set_cursor()   // Position cursor
│
├─► event_handler.next().await
│   │
│   ├─► Some(Event::Tick) ──► Continue (redraw happens)
│   │
│   ├─► Some(Event::Key(key))
│   │   │
│   │   └─► handle_key_event(key, state, notification_manager)
│   │       ├─► CommandMapper::map(key, state.mode) ──► Action
│   │       ├─► match action { ... }  // Execute action
│   │       └─► state.scroll_to_cursor(...)  // BUG: hardcoded rect
│   │
│   └─► None ──► Break loop (channel closed)
│
└─► Loop
```

### Action Execution

```
Action Handling (in handle_key_event)
│
├─► Quit ──► std::process::exit(0)
│
├─► Save ──► file_io::write_file(path, content).await
│   ├─► Success ──► state.document.mark_clean()
│   │             └─► NotificationManager::send("Saved", Success)
│   └─► Error ──► Propagated up, sent as notification
│
├─► EnterMode(m) ──► state.set_mode(m)
│
├─► MoveX ──► state.move_cursor(dx, dy)
│   ├─► Clamp Y to [0, line_count-1]
│   ├─► Get line length at new Y
│   └─► Clamp X to [0, line_length]
│
├─► InsertChar(c) ──► state.document.buffer_mut().insert_char(loc, c)
│   ├─► Convert char index to byte index
│   ├─► String::insert(byte_idx, c)
│   ├─► state.cursor = new_location
│   └─► state.document.mark_dirty()
│
└─► DeleteChar ──► state.document.buffer_mut().delete_char(loc)
    ├─► If x=0, y>0: Merge with previous line
    └─► Else: Delete char before cursor
```

---

## State Transitions

### Mode Transitions

See [Mode Contract](../technical/contracts/mode.md), [Design Spec](../../design/modal_editing/transitions.md).

```
┌─────────┐         i          ┌─────────┐
│ NORMAL  │───────────────────▶│ INSERT  │
│         │◀───────────────────│         │
└────┬────┘        Esc         └─────────┘
     │
     │ :
     ▼
┌─────────┐
│ COMMAND │
│         │◀─── Any char (appends to buffer)
└────┬────┘
     │
     │ Enter or Esc
     ▼
┌─────────┐
│ NORMAL  │
└─────────┘
```

### Dirty State Transitions

```
┌─────────────┐
│    Clean    │◀───────────────────┐
│  (dirty=false)                   │
└──────┬──────┘                    │
       │                           │
       │ Any edit:                 │ mark_clean()
       │ - InsertChar              │ (after save)
       │ - DeleteChar              │
       │ - InsertNewline           │
       ▼                           │
┌─────────────┐                    │
│    Dirty    │────────────────────┘
│  (dirty=true)│
└─────────────┘
```

---

## Rendering Detail

### Layout Geometry Calculation

```rust
// Given terminal size (e.g., 80x24)
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Min(1),        // Editor gets remaining space
        Constraint::Length(3),     // Notifications fixed at 3
        Constraint::Length(1),     // Status bar fixed at 1
    ])
    .split(area);

// Result for 80x24:
// chunks[0] = Rect { x: 0, y: 0, width: 80, height: 20 }
// chunks[1] = Rect { x: 0, y: 20, width: 80, height: 3 }
// chunks[2] = Rect { x: 0, y: 23, width: 80, height: 1 }
```

### Line Number Format

```rust
let line_num = format!("{:>4} │ ", i + 1);
// Examples:
//   1 │     (line 1)
//  12 │     (line 12)
// 123 │     (line 123)
//1000 │     (line 1000)
```

### Status Bar Format

```rust
// Current implementation:
// [MODE] [filename] [[+]]
// Example: " NORMAL  test.txt  [+] "

// Mode styling:
Span::styled(
    &mode_str, 
    Style::default()
        .bg(self.theme.accent)      // Cyan background
        .fg(Color::Black)            // Black text
        .add_modifier(Modifier::BOLD)
)

// Dirty indicator:
Span::styled(" [+] ", Style::default().fg(Color::Yellow))
```

### Notification Format

```rust
// Prefixes (aligned to 6 chars):
// "[INFO] " - Blue
// "[OK]   " - Green
// "[WARN] " - Yellow
// "[ERR]  " - Red

// Each prefix is bold, message is normal weight
```

---

## Error Handling Paths

### I/O Error During Save

```
file_io::write_file(path, content).await
│
├─► tokio::fs::write fails
│   │
│   └─► Returns std::io::Error
│       │
│       └─► .map_err(EditorError::from) ──► EditorError::IO
│           │
│           └─► Propagated via ? in handle_key_event
│               │
│               └─► Caught in run_event_loop
│                   │
│                   └─► NotificationManager::send(error, Fatal)
```

### Bounds Error in Buffer

```
buffer.insert_char(loc, ch)
│
├─► loc.y >= line_count
│   │
│   └─► lines.get_mut(loc.y) returns None
│       │
│       └─► ok_or_else(|| EditorError::Bounds(...))
│           │
│           └─► Returned to caller
```

### Terminal Setup Failure

```
Renderer::setup_terminal()
│
├─► stdout().execute(EnterAlternateScreen) fails
│   │
│   └─► Returns crossterm::ErrorKind
│       │
│       └─► Converted to anyhow::Error
│           │
│           └─► Matched in main.rs
│               │
│               └─► Fallback to run_simple_mode()
```

---

## Trait Implementations

### Default

```rust
impl Default for Buffer {
    fn default() -> Self { Self::new() }  // vec![String::new()]
}

impl Default for Document {
    fn default() -> Self { Self::new() }  // empty buffer, no path, clean
}

impl Default for EditorState {
    fn default() -> Self { Self::new() }  // new doc, cursor (0,0), Normal mode
}

impl Default for Renderer {
    fn default() -> Self { Self::new() }  // default theme
}

impl Default for EventHandler {
    fn default() -> Self { Self::new() }  // spawns background task
}
```

### Display

```rust
// commands.rs
impl std::fmt::Display for EditorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorMode::Normal => write!(f, "NORMAL"),
            EditorMode::Insert => write!(f, "INSERT"),
            EditorMode::Command => write!(f, "COMMAND"),
        }
    }
}
```

### Error (via thiserror)

```rust
// error.rs
#[derive(Error, Debug)]
pub enum EditorError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Out of bounds: {0}")]
    Bounds(String),
    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8,
    #[error("Channel error: {0}")]
    Channel(String),
}
```

---

## Memory Layout

### EditorState Memory Structure

```
EditorState (stack allocated)
├── document: Document
│   ├── buffer: Buffer
│   │   └── lines: Vec<String> (heap)
│   │       ├── String[0] (heap)
│   │       ├── String[1] (heap)
│   │       └── ...
│   ├── file_path: Option<PathBuf> (heap if Some)
│   └── dirty: bool (1 byte)
├── cursor: Location
│   ├── g_index: usize (8 bytes)
│   ├── x: usize (8 bytes)
│   └── y: usize (8 bytes)
├── mode: EditorMode (1 byte, discriminant)
└── scroll_offset: (u16, u16) (4 bytes)
```

### NotificationManager Memory Structure

```
NotificationManager
├── rx: mpsc::UnboundedReceiver<Notification> (channel)
├── active: Vec<Notification> (heap)
│   └── Notification
│       ├── level: NotifyLevel
│       ├── message: String (heap)
│       ├── timestamp: Instant
│       └── ttl: Duration
└── history: Vec<Notification> (heap, grows unbounded)
```

---

## Reconstruction Validation

See [Acceptance Criteria](../implementation/reconstruction_acceptance.md) for full checklist.

### Build
- [ ] `cargo build` succeeds
- [ ] All files ≤200 lines
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` produces no changes

### Functional
- [ ] TTY mode works
- [ ] Non-TTY simple mode works
- [ ] Navigation keys work
- [ ] Edit operations work
- [ ] Save/quit work

### Edge Cases
- [ ] Empty file
- [ ] Trailing newline
- [ ] Unicode
- [ ] Large files
