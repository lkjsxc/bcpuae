//! System infrastructure (events, notifications)

pub mod events;
pub mod notifications;

pub use events::{Event, EventHandler};
