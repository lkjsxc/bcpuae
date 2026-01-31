//! Document management with file path and dirty state

use std::path::{Path, PathBuf};

use crate::core::Buffer;

/// A document wraps a buffer with file path and dirty state tracking
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Document {
    buffer: Buffer,
    file_path: Option<PathBuf>,
    dirty: bool,
}

impl Document {
    /// Create a new empty document
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            file_path: None,
            dirty: false,
        }
    }

    /// Create a document from a buffer with optional path
    pub fn with_buffer(buffer: Buffer, path: Option<PathBuf>) -> Self {
        Self {
            buffer,
            file_path: path,
            dirty: false,
        }
    }

    /// Open a document from a file path
    pub async fn open(path: &Path) -> crate::core::Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;
        let buffer = Buffer::from_str(&content);
        Ok(Self {
            buffer,
            file_path: Some(path.to_path_buf()),
            dirty: false,
        })
    }

    /// Get a reference to the buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get a mutable reference to the buffer
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Get the file path if set
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Set the file path
    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path);
    }

    /// Check if the document has unsaved changes
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Mark the document as dirty (modified)
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark the document as clean (saved)
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    /// Get the filename for display (or "[No Name]")
    pub fn display_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "[No Name]".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_document() {
        let doc = Document::new();
        assert!(!doc.is_dirty());
        assert_eq!(doc.file_path(), None);
        assert_eq!(doc.display_name(), "[No Name]");
    }

    #[test]
    fn test_dirty_state() {
        let mut doc = Document::new();
        assert!(!doc.is_dirty());

        doc.mark_dirty();
        assert!(doc.is_dirty());

        doc.mark_clean();
        assert!(!doc.is_dirty());
    }

    #[test]
    fn test_display_name_with_path() {
        let path = PathBuf::from("/home/user/test.txt");
        let doc = Document::with_buffer(Buffer::new(), Some(path));
        assert_eq!(doc.display_name(), "test.txt");
    }
}
