//! Application logic and event handling

pub mod commands;
pub mod event_loop;
pub mod simple_mode;

pub use event_loop::run_event_loop;
pub use simple_mode::run_simple_mode;
