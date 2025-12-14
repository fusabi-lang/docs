//! Layout module - Constraint-based layout system
//!
//! This module provides utilities for dividing terminal space using constraints,
//! similar to CSS Flexbox. Layouts can be horizontal or vertical and support
//! various constraint types.
//!
//! ## Constraint Types
//!
//! - **Percentage**: Proportional allocation (e.g., 50%)
//! - **Ratio**: Fractional allocation (e.g., 1/3)
//! - **Length**: Fixed size in characters
//! - **Min**: Minimum size with flexibility
//! - **Max**: Maximum size with flexibility
//!
//! ## Example
//!
//! ```rust
//! use fusabi_tui::layouts::{Layout, Constraint};
//!
//! // Layout utilities will be used to manage terminal space
//! ```

use anyhow::Result;

/// Represents a layout constraint for dividing space
#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    /// Percentage of available space (0-100)
    Percentage(u16),
    /// Ratio of available space (numerator, denominator)
    Ratio(u32, u32),
    /// Fixed length in characters
    Length(u16),
    /// Minimum size
    Min(u16),
    /// Maximum size
    Max(u16),
}

/// Layout builder for dividing rectangular areas
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
    margin: u16,
}

/// Layout direction (horizontal or vertical)
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    /// Horizontal layout (left to right)
    Horizontal,
    /// Vertical layout (top to bottom)
    Vertical,
}

impl Layout {
    /// Create a new layout with the given direction
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            constraints: Vec::new(),
            margin: 0,
        }
    }

    /// Create a vertical layout
    pub fn vertical() -> Self {
        Self::new(Direction::Vertical)
    }

    /// Create a horizontal layout
    pub fn horizontal() -> Self {
        Self::new(Direction::Horizontal)
    }

    /// Set the constraints for this layout
    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    /// Set the margin for this layout
    pub fn margin(mut self, margin: u16) -> Self {
        self.margin = margin;
        self
    }
}

// Placeholder for layout computation logic
// This will be populated when extracting from hibana
