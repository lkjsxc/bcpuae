//! Error types for the editor

use thiserror::Error;

/// Main error type for editor operations
#[derive(Error, Debug)]
pub enum EditorError {
    /// IO error from std::io
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    /// Out of bounds access
    #[error("Out of bounds: {0}")]
    Bounds(String),

    /// Invalid UTF-8 sequence
    #[error("Invalid UTF-8 sequence")]
    InvalidUtf8,

    /// Channel communication error
    #[error("Channel error: {0}")]
    Channel(String),
}

/// Result type alias for editor operations
pub type Result<T> = std::result::Result<T, EditorError>;
