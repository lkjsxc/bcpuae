//! Main event loop for TUI mode

use anyhow::Result;
use ratatui::Terminal;
use tracing::{error, info, warn};

use crate::app::commands::handle_key_event;
use crate::system::notifications::{NotificationManager, NotifyLevel};
use crate::system::{Event, EventHandler};
use crate::ui::state::EditorState;
use crate::ui::{Layout, Renderer};

/// Run the main TUI event loop
pub async fn run_event_loop(
    state: &mut EditorState,
    event_handler: &mut EventHandler,
    renderer: &Renderer,
    terminal: &mut Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    notification_manager: &mut NotificationManager,
) -> Result<()> {
    info!("Entering main event loop");

    loop {
        // Update notifications (expire old ones)
        notification_manager.update();

        // Calculate layout for this frame
        let layout = Layout::calculate(terminal.size()?);

        // Draw UI
        if let Err(e) = renderer.draw(terminal, state, notification_manager.active().to_vec()) {
            error!("Render error: {}", e);
        }

        // Wait for event
        match event_handler.next().await {
            Some(event) => handle_event(event, state, &layout, notification_manager).await,
            None => {
                warn!("Event channel closed, exiting");
                break;
            }
        }
    }

    info!("Exiting main event loop");
    Ok(())
}

async fn handle_event(
    event: Event,
    state: &mut EditorState,
    layout: &Layout,
    notification_manager: &mut NotificationManager,
) {
    match event {
        Event::Key(key) => {
            state.idle_tracker.record_activity();
            if let Err(e) = handle_key_event(key, state, layout, notification_manager).await {
                NotificationManager::send(format!("Error: {}", e), NotifyLevel::Fatal);
            }
        }
        Event::Tick => {
            // Update idle state for sparkle effect
            state.update_idle();
        }
    }
}
