//! Inactivity tracking and sparkle effects

use std::time::{Duration, Instant};

/// Inactivity threshold for sparkle effect (10 seconds)
const SPARKLE_THRESHOLD: Duration = Duration::from_secs(10);

/// Tracks user inactivity for visual effects
#[derive(Debug, Clone, Copy)]
pub struct IdleTracker {
    last_activity: Instant,
    is_idle: bool,
}

impl Default for IdleTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl IdleTracker {
    /// Create a new idle tracker
    pub fn new() -> Self {
        Self {
            last_activity: Instant::now(),
            is_idle: false,
        }
    }

    /// Record user activity (resets idle timer)
    pub fn record_activity(&mut self) {
        self.last_activity = Instant::now();
        self.is_idle = false;
    }

    /// Update idle state based on current time
    pub fn update(&mut self) {
        self.is_idle = self.last_activity.elapsed() > SPARKLE_THRESHOLD;
    }

    /// Check if sparkle effect should be active
    pub fn should_sparkle(&self) -> bool {
        self.is_idle
    }

    /// Get sparkle hue offset based on elapsed idle time (fast cycle)
    pub fn sparkle_offset(&self) -> u64 {
        let elapsed = self.last_activity.elapsed().as_millis() as u64;
        elapsed / 15 // Fast cycle: change hue every 15ms
    }

    /// Get rainbow color for a specific character with independent cycling
    /// Each character has its own phase offset creating a dazzling wave effect
    pub fn char_sparkle_color(&self, line_idx: usize, char_idx: usize) -> ratatui::style::Color {
        let base = self.sparkle_offset();
        // Each character has independent phase: line * 37 + char * 13 creates
        // a non-repeating pattern where adjacent characters cycle at different rates
        let char_phase = (line_idx * 37 + char_idx * 13) as u64;
        // Cycle each character through full rainbow at different speeds
        let hue = ((base + char_phase) * 3) % 360;
        rainbow_color(hue as u64)
    }

    /// Get seconds of inactivity
    pub fn idle_seconds(&self) -> u64 {
        self.last_activity.elapsed().as_secs()
    }
}

/// Convert HSL to RGB for rainbow sparkle effect
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r1 + m) * 255.0) as u8,
        ((g1 + m) * 255.0) as u8,
        ((b1 + m) * 255.0) as u8,
    )
}

/// Get rainbow color for sparkle effect at given offset
pub fn rainbow_color(offset: u64) -> ratatui::style::Color {
    let hue = (offset % 360) as f32;
    let (r, g, b) = hsl_to_rgb(hue, 0.8, 0.6);
    ratatui::style::Color::Rgb(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idle_tracker_new() {
        let tracker = IdleTracker::new();
        assert!(!tracker.should_sparkle());
    }

    #[test]
    fn test_activity_resets_idle() {
        let mut tracker = IdleTracker::new();
        // Simulate time passing
        tracker.last_activity = Instant::now() - Duration::from_secs(15);
        tracker.update();
        assert!(tracker.should_sparkle());

        // Activity resets
        tracker.record_activity();
        assert!(!tracker.should_sparkle());
    }

    #[test]
    fn test_hsl_to_rgb() {
        // Red at hue 0
        let (r, g, b) = hsl_to_rgb(0.0, 1.0, 0.5);
        assert!(r > 200);
        assert!(g < 50);
        assert!(b < 50);

        // Green at hue 120
        let (r, g, b) = hsl_to_rgb(120.0, 1.0, 0.5);
        assert!(r < 50);
        assert!(g > 200);
        assert!(b < 50);
    }
}
