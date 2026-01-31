//! bcpuae - A modal text editor in Rust
//! Main application entry point

use std::io::stdout;

use anyhow::Result;
use tracing::{error, info, warn};
use tracing_subscriber::EnvFilter;

mod app;
mod core;
mod operations;
mod system;
mod ui;

use crate::app::run_event_loop;
use crate::app::run_simple_mode;
use crate::system::notifications::{NotificationManager, NotifyLevel};
use crate::system::EventHandler;
use crate::ui::state::EditorState;
use crate::ui::Renderer;

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic_hook();
    
    // Initialize tracing with env-filter for configurable log levels
    // Default to ERROR only - set RUST_LOG=info to see all logs
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("error")),
        )
        .init();
    info!("Starting bcpuae");

    let is_tty = crossterm::tty::IsTty::is_tty(&stdout());
    info!("Running in TTY: {}", is_tty);

    if !is_tty {
        warn!("Not running in a TTY, falling back to simple mode");
        return run_simple_mode().await;
    }

    match run_tui_mode().await {
        Ok(()) => {
            info!("bcpuae exited successfully");
            Ok(())
        }
        Err(e) => {
            error!("Editor error: {}", e);
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn setup_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Renderer::restore_terminal();
        original_hook(info);
    }));
}

async fn run_tui_mode() -> Result<()> {
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

    let mut notification_manager = NotificationManager::init();
    NotificationManager::send("Editor started", NotifyLevel::Info);

    let renderer = Renderer::new();
    let mut event_handler = EventHandler::new();
    let mut state = EditorState::new();

    let result = run_event_loop(
        &mut state,
        &mut event_handler,
        &renderer,
        &mut terminal,
        &mut notification_manager,
    )
    .await;

    if let Err(e) = Renderer::restore_terminal() {
        warn!("Failed to restore terminal: {}", e);
    }

    result
}
