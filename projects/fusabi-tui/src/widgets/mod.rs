//! Widget module - Ratatui widget wrappers and builders
//!
//! This module provides type-safe builders for Ratatui widgets, making them
//! accessible from Fusabi scripts. Each widget type has a corresponding builder
//! that handles construction and rendering.
//!
//! ## Supported Widgets
//!
//! - **List**: Scrollable list of items (planned)
//! - **Table**: Multi-column tabular data (implemented)
//! - **Gauge**: Progress indicators (planned)
//! - **Chart**: Line and bar charts (planned)
//! - **Paragraph**: Multi-line text blocks (planned)
//! - **Block**: Container with borders and titles (planned)
//!
//! ## Example
//!
//! ```rust
//! use fusabi_tui::widgets::{WidgetBuilder, Widget};
//! use fusabi_tui::widgets::table::{TableData, ColumnDef};
//!
//! // Use the table widget with generic data
//! // See table module documentation for detailed examples
//! ```

use anyhow::Result;

pub mod table;

/// Common trait for all widget types
pub trait Widget {
    /// Render the widget to a buffer
    fn render(&self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer);
}

/// Builder pattern trait for constructing widgets
pub trait WidgetBuilder {
    /// The widget type this builder produces
    type Output: Widget;

    /// Build the widget from the current configuration
    fn build(self) -> Result<Self::Output>;
}

// Re-export table types for convenience
pub use table::{ColumnDef, TableData, render_table};
