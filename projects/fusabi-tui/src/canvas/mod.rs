//! Canvas module - Graph visualization and custom rendering
//!
//! This module provides canvas-based rendering for graph visualizations
//! and custom graphics using Ratatui's canvas primitives.
//!
//! ## Features
//!
//! - **Graph Visualization**: Directed graphs with nodes and edges
//! - **Node Rendering**: Colored rectangles with selection highlighting
//! - **Edge Rendering**: Lines with directional arrows and labels
//! - **Automatic Bounds**: Smart calculation of canvas coordinate space
//! - **Custom Styling**: Full control over colors and appearance
//!
//! ## Example
//!
//! ```rust
//! use fusabi_tui::canvas::graph::{GraphCanvas, GraphData, GraphNode, GraphEdge};
//! use ratatui::style::Color;
//!
//! // Create a simple graph
//! let mut graph = GraphData::new();
//! graph.add_node(GraphNode::new("source", 10.0, 20.0, "Source"));
//! graph.add_node(GraphNode::new("sink", 50.0, 20.0, "Sink"));
//! graph.add_edge(GraphEdge::new("source", "sink").label("data flow"));
//!
//! // Render the graph
//! let widget = GraphCanvas::new(&graph);
//! // frame.render_widget(widget, area);
//! ```
//!
//! ## Module Structure
//!
//! - [`graph`] - Main graph widget and data structures
//! - [`node`] - Node rendering utilities
//! - [`edge`] - Edge rendering with arrows
//! - [`bounds`] - Bounds calculation for coordinate systems

pub mod bounds;
pub mod edge;
pub mod graph;
pub mod node;

// Re-export main types for convenience
pub use graph::{GraphCanvas, GraphData, GraphEdge, GraphNode};
