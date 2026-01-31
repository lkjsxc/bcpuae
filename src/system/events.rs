//! Async event handling with crossterm

use crossterm::event::{Event as CrosstermEvent, KeyEvent};
use futures::{FutureExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
    /// Key press event
    Key(KeyEvent),
    /// Timer tick for periodic updates
    Tick,
}

/// Async event handler using MPSC channel
#[derive(Debug)]
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    /// Create a new event handler and spawn the background task
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut tick = tokio::time::interval(Duration::from_millis(250));

            loop {
                let tick_delay = tick.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick_delay => {
                        if tx.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    result = crossterm_event => {
                        match result {
                            Some(Ok(CrosstermEvent::Key(key))) => {
                                if tx.send(Event::Key(key)).is_err() {
                                    break;
                                }
                            }
                            Some(Err(e)) => {
                                tracing::error!("Crossterm error: {}", e);
                            }
                            None => break,
                            _ => {}
                        }
                    }
                }
            }
        });

        Self { rx }
    }

    /// Get the next event
    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
