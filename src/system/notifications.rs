//! Notification system with MPSC channel

use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Maximum notification history to retain
const MAX_HISTORY: usize = 32;

/// Maximum active notifications
const MAX_ACTIVE: usize = 8;

/// Notification severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotifyLevel {
    /// General information (blue)
    Info,
    /// Success message (green)
    Success,
    /// Warning message (yellow)
    Warning,
    /// Error message (red)
    Fatal,
}

impl NotifyLevel {
    /// Get the TTL for this notification level
    fn ttl(&self) -> Duration {
        match self {
            NotifyLevel::Info => Duration::from_secs(5),
            NotifyLevel::Success => Duration::from_secs(3),
            NotifyLevel::Warning => Duration::from_secs(7),
            NotifyLevel::Fatal => Duration::from_secs(10),
        }
    }
}

/// A single notification
#[derive(Debug, Clone)]
pub struct Notification {
    /// Severity level
    pub level: NotifyLevel,
    /// Message content
    pub message: String,
    /// When the notification was created
    pub timestamp: Instant,
    /// Time to live
    pub ttl: Duration,
}

impl Notification {
    /// Create a new notification
    fn new(level: NotifyLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            timestamp: Instant::now(),
            ttl: level.ttl(),
        }
    }

    /// Check if this notification has expired
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.timestamp) > self.ttl
    }
}

/// Global sender for notifications
static mut SENDER: Option<mpsc::UnboundedSender<Notification>> = None;

/// Manages active and historical notifications
#[derive(Debug)]
pub struct NotificationManager {
    rx: mpsc::UnboundedReceiver<Notification>,
    active: Vec<Notification>,
    history: Vec<Notification>,
}

impl NotificationManager {
    /// Initialize the notification system and create a manager
    pub fn init() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Store sender globally for static access
        unsafe {
            SENDER = Some(tx);
        }

        Self {
            rx,
            active: Vec::with_capacity(MAX_ACTIVE),
            history: Vec::with_capacity(MAX_HISTORY),
        }
    }

    /// Send a notification from anywhere in the codebase
    pub fn send(message: impl Into<String>, level: NotifyLevel) {
        unsafe {
            if let Some(ref sender) = SENDER {
                let notif = Notification::new(level, message);
                let _ = sender.send(notif);
            }
        }
    }

    /// Update notifications: collect new ones and expire old ones
    pub fn update(&mut self) {
        // Collect new notifications from channel (limit to prevent spam)
        let mut count = 0;
        while let Ok(notif) = self.rx.try_recv() {
            if self.active.len() < MAX_ACTIVE {
                self.active.push(notif);
            }
            count += 1;
            if count >= MAX_ACTIVE {
                break;
            }
        }

        // Expire old notifications
        let now = Instant::now();
        let mut i = 0;
        while i < self.active.len() {
            if now.duration_since(self.active[i].timestamp) > self.active[i].ttl {
                let notif = self.active.remove(i);
                // Add to history with rotation
                if self.history.len() >= MAX_HISTORY {
                    self.history.remove(0);
                }
                self.history.push(notif);
            } else {
                i += 1;
            }
        }
    }

    /// Get active notifications as a slice
    pub fn active(&self) -> Vec<Notification> {
        self.active.clone()
    }

    /// Get the count of active notifications
    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Get history count
    pub fn history_count(&self) -> usize {
        self.history.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notif = Notification::new(NotifyLevel::Info, "test message");
        assert_eq!(notif.level, NotifyLevel::Info);
        assert_eq!(notif.message, "test message");
        assert!(!notif.is_expired());
    }

    #[test]
    fn test_notification_ttl() {
        let info = NotifyLevel::Info;
        assert_eq!(info.ttl(), Duration::from_secs(5));

        let success = NotifyLevel::Success;
        assert_eq!(success.ttl(), Duration::from_secs(3));
    }
}
