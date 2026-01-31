//! Async file operations

use std::path::Path;

use crate::core::Result;

/// Write content to a file asynchronously
pub async fn write_file(path: &Path, content: &str) -> Result<()> {
    tokio::fs::write(path, content).await?;
    Ok(())
}

/// Read content from a file asynchronously
pub async fn read_file(path: &Path) -> Result<String> {
    let content = tokio::fs::read_to_string(path).await?;
    Ok(content)
}
