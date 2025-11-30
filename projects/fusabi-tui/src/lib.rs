//! # Fusabi TUI
//!
//! A Terminal UI library providing Ratatui bindings and widgets for Fusabi scripts.
//!
//! ## Overview
//!
//! `fusabi-tui` enables Fusabi scripts to create rich terminal user interfaces by exposing
//! Ratatui's powerful widget system and layout engine. This library provides:
//!
//! - **Widgets**: High-level UI components (lists, tables, charts, gauges, etc.)
//! - **Layouts**: Flexbox-like constraint-based layout system
//! - **Canvas**: Low-level drawing primitives for custom visualizations
//! - **Bindings**: Native functions exposed to Fusabi scripts
//!
//! ## Architecture
//!
//! The library is organized into four main modules:
//!
//! - `widgets`: Ratatui widget wrappers and builders
//! - `layouts`: Layout utilities and constraint handling
//! - `canvas`: Canvas-based rendering for custom graphics
//! - `bindings`: Fusabi VM integration and native function registration
//!
//! ## Example Usage
//!
//! ```fsharp
//! // From a Fusabi script (.fsx)
//! open Fusabi.TUI
//!
//! let list = List.create ["Item 1"; "Item 2"; "Item 3"]
//! let layout = Layout.vertical [Constraint.Percentage 50; Constraint.Min 10]
//! ```
//!
//! ## Features
//!
//! - Zero-copy where possible for performance
//! - Type-safe widget construction
//! - Composable layout system
//! - Seamless Fusabi integration

pub mod widgets;
pub mod layouts;
pub mod canvas;
#[cfg(feature = "bindings")]
pub mod bindings;
pub mod formatting;

// Re-export commonly used types for convenience
pub use widgets::{Widget, WidgetBuilder};
pub use widgets::table::{TableData, ColumnDef, render_table};
pub use layouts::{Layout, Constraint};
pub use canvas::{GraphCanvas, GraphData, GraphNode, GraphEdge};
#[cfg(feature = "bindings")]
pub use bindings::FusabiTuiModule;
pub use formatting::{format_number, format_bytes, format_latency, format_duration};

/// Result type alias for this crate
pub type Result<T> = anyhow::Result<T>;

/// Version of the fusabi-tui library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!VERSION.is_empty());
    }
}
