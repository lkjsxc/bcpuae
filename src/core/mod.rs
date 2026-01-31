//! Core data structures for the editor

pub mod buffer;
pub mod document;
pub mod error;
pub mod location;

pub use buffer::Buffer;
pub use document::Document;
pub use error::Result;
pub use location::Location;
