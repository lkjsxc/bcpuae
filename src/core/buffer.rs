//! Text buffer with line operations

use crate::core::error::EditorError;
use crate::core::{Location, Result};

/// Text storage with line-based operations
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Buffer {
    lines: Vec<String>,
}

impl Buffer {
    /// Create a new empty buffer with one empty line
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }

    /// Create buffer from string content
    pub fn from_str(content: &str) -> Self {
        if content.is_empty() {
            return Self::new();
        }

        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        // Handle trailing newline: lines() drops it, we need empty final line
        if content.ends_with('\n') {
            lines.push(String::new());
        }

        // Ensure at least one line
        if lines.is_empty() {
            lines.push(String::new());
        }

        Self { lines }
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// Get a reference to the lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }

    /// Get a specific line
    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }

    /// Get the length of a specific line in characters
    pub fn line_len(&self, index: usize) -> usize {
        self.lines
            .get(index)
            .map(|s| s.chars().count())
            .unwrap_or(0)
    }

    /// Insert a character at the specified location
    pub fn insert_char(&mut self, loc: &Location, ch: char) -> Result<Location> {
        let line = self
            .lines
            .get_mut(loc.y)
            .ok_or_else(|| EditorError::Bounds(format!("line {} out of range", loc.y)))?;

        // Convert char index to byte index
        let byte_idx = line
            .char_indices()
            .nth(loc.x)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        line.insert(byte_idx, ch);
        Ok(Location::new(loc.x + 1, loc.y))
    }

    /// Delete a character before the specified location
    pub fn delete_char(&mut self, loc: &Location) -> Result<Location> {
        if loc.x == 0 {
            if loc.y == 0 {
                // Document start, nothing to do
                return Ok(*loc);
            }

            // Merge current line with previous
            let current_line = self.lines.remove(loc.y);

            let prev_line = &mut self.lines[loc.y.saturating_sub(1)];
            let prev_len = prev_line.chars().count();
            prev_line.push_str(&current_line);

            return Ok(Location::new(prev_len, loc.y.saturating_sub(1)));
        }

        // Normal character deletion
        let line = self
            .lines
            .get_mut(loc.y)
            .ok_or_else(|| EditorError::Bounds(format!("line {} out of range", loc.y)))?;

        let byte_idx = line
            .char_indices()
            .nth(loc.x.saturating_sub(1))
            .map(|(i, _)| i)
            .unwrap_or(0);

        line.remove(byte_idx);
        Ok(Location::new(loc.x.saturating_sub(1), loc.y))
    }

    /// Insert a newline at the specified location
    pub fn insert_newline(&mut self, loc: &Location) -> Result<Location> {
        let line = self
            .lines
            .get_mut(loc.y)
            .ok_or_else(|| EditorError::Bounds(format!("line {} out of range", loc.y)))?;

        // Convert char index to byte index for splitting
        let byte_idx = line
            .char_indices()
            .nth(loc.x)
            .map(|(i, _)| i)
            .unwrap_or(line.len());

        let remainder: String = line.split_off(byte_idx);
        self.lines.insert(loc.y + 1, remainder);

        Ok(Location::new(0, loc.y + 1))
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lines.join("\n"))
    }
}

#[cfg(test)]
mod tests;
