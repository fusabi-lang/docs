//! Graph widget for canvas-based visualization
//!
//! This module provides a complete graph visualization widget using
//! ratatui's canvas for rendering nodes and edges as a directed graph.

use super::bounds::calculate_bounds;
use super::edge::render_edge;
use super::node::render_node;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{
        canvas::{Canvas, Context},
        Block, Widget,
    },
};
use std::collections::HashMap;

/// A node in the graph
///
/// Represents a single node with position, size, visual properties,
/// and selection state.
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// Unique identifier for the node
    pub id: String,
    /// X coordinate of the node's top-left corner
    pub x: f64,
    /// Y coordinate of the node's top-left corner
    pub y: f64,
    /// Width of the node
    pub width: f64,
    /// Height of the node
    pub height: f64,
    /// Color of the node
    pub color: Color,
    /// Display label for the node
    pub label: String,
    /// Whether the node is currently selected
    pub selected: bool,
}

impl GraphNode {
    /// Create a new graph node
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `x` - X position
    /// * `y` - Y position
    /// * `label` - Display label
    ///
    /// # Returns
    ///
    /// A new GraphNode with default dimensions and color
    pub fn new(id: impl Into<String>, x: f64, y: f64, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            x,
            y,
            width: 8.0,
            height: 4.0,
            color: Color::Blue,
            label: label.into(),
            selected: false,
        }
    }

    /// Create a builder for the node
    pub fn builder(id: impl Into<String>) -> GraphNodeBuilder {
        GraphNodeBuilder::new(id)
    }
}

/// Builder for GraphNode with fluent API
#[derive(Debug)]
pub struct GraphNodeBuilder {
    id: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: Color,
    label: String,
    selected: bool,
}

impl GraphNodeBuilder {
    /// Create a new builder
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            label: id.clone(),
            id,
            x: 0.0,
            y: 0.0,
            width: 8.0,
            height: 4.0,
            color: Color::Blue,
            selected: false,
        }
    }

    /// Set position
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set size
    pub fn size(mut self, width: f64, height: f64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Build the node
    pub fn build(self) -> GraphNode {
        GraphNode {
            id: self.id,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            color: self.color,
            label: self.label,
            selected: self.selected,
        }
    }
}

/// An edge connecting two nodes
///
/// Represents a directed edge from one node to another with
/// optional styling and labels.
#[derive(Debug, Clone)]
pub struct GraphEdge {
    /// ID of the source node
    pub from: String,
    /// ID of the destination node
    pub to: String,
    /// Color of the edge
    pub color: Color,
    /// Optional label for the edge (e.g., weight, throughput)
    pub label: Option<String>,
}

impl GraphEdge {
    /// Create a new graph edge
    ///
    /// # Arguments
    ///
    /// * `from` - Source node ID
    /// * `to` - Destination node ID
    ///
    /// # Returns
    ///
    /// A new GraphEdge with default color (Cyan)
    pub fn new(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            color: Color::Cyan,
            label: None,
        }
    }

    /// Set the edge color
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the edge label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Complete graph data structure
///
/// Contains all nodes and edges for rendering.
#[derive(Debug, Clone, Default)]
pub struct GraphData {
    /// All nodes in the graph
    pub nodes: Vec<GraphNode>,
    /// All edges in the graph
    pub edges: Vec<GraphEdge>,
}

impl GraphData {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a graph with nodes and edges
    pub fn with_data(nodes: Vec<GraphNode>, edges: Vec<GraphEdge>) -> Self {
        Self { nodes, edges }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }

    /// Find a node by ID
    pub fn find_node(&self, id: &str) -> Option<&GraphNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Find a mutable node by ID
    pub fn find_node_mut(&mut self, id: &str) -> Option<&mut GraphNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get all edges from a specific node
    pub fn edges_from(&self, node_id: &str) -> Vec<&GraphEdge> {
        self.edges.iter().filter(|e| e.from == node_id).collect()
    }

    /// Get all edges to a specific node
    pub fn edges_to(&self, node_id: &str) -> Vec<&GraphEdge> {
        self.edges.iter().filter(|e| e.to == node_id).collect()
    }
}

/// Graph canvas widget
///
/// Renders a directed graph with nodes and edges on a canvas.
/// Supports automatic bounds calculation and optional block wrapping.
pub struct GraphCanvas<'a> {
    graph: &'a GraphData,
    block: Option<Block<'a>>,
    x_bounds: Option<(f64, f64)>,
    y_bounds: Option<(f64, f64)>,
}

impl<'a> GraphCanvas<'a> {
    /// Create a new graph canvas widget
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph data to render
    ///
    /// # Example
    ///
    /// ```
    /// use fusabi_tui::canvas::graph::{GraphCanvas, GraphData, GraphNode, GraphEdge};
    /// use ratatui::style::Color;
    ///
    /// let mut graph = GraphData::new();
    /// graph.add_node(GraphNode::new("node1", 10.0, 20.0, "Node 1"));
    /// graph.add_node(GraphNode::new("node2", 50.0, 20.0, "Node 2"));
    /// graph.add_edge(GraphEdge::new("node1", "node2"));
    ///
    /// let widget = GraphCanvas::new(&graph);
    /// ```
    pub fn new(graph: &'a GraphData) -> Self {
        Self {
            graph,
            block: None,
            x_bounds: None,
            y_bounds: None,
        }
    }

    /// Set an optional block to wrap the canvas
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Set custom X bounds (overrides auto-calculation)
    pub fn x_bounds(mut self, min: f64, max: f64) -> Self {
        self.x_bounds = Some((min, max));
        self
    }

    /// Set custom Y bounds (overrides auto-calculation)
    pub fn y_bounds(mut self, min: f64, max: f64) -> Self {
        self.y_bounds = Some((min, max));
        self
    }

    /// Calculate or use provided bounds
    fn get_bounds(&self) -> (f64, f64, f64, f64) {
        let (auto_min_x, auto_max_x, auto_min_y, auto_max_y) =
            calculate_bounds(&self.graph.nodes);

        let (min_x, max_x) = self.x_bounds.unwrap_or((auto_min_x, auto_max_x));
        let (min_y, max_y) = self.y_bounds.unwrap_or((auto_min_y, auto_max_y));

        (min_x, max_x, min_y, max_y)
    }

    /// Paint function for the canvas
    fn paint(&self, ctx: &mut Context) {
        // Build node lookup map for edge rendering
        let node_map: HashMap<String, &GraphNode> =
            self.graph.nodes.iter().map(|n| (n.id.clone(), n)).collect();

        // Render edges first (so they appear behind nodes)
        for edge in &self.graph.edges {
            render_edge(ctx, edge, &node_map);
        }

        // Render nodes
        for node in &self.graph.nodes {
            render_node(ctx, node);
        }
    }
}

impl<'a> Widget for GraphCanvas<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate bounds for the canvas
        let (min_x, max_x, min_y, max_y) = self.get_bounds();

        // Clone data needed for the paint closure
        let nodes = self.graph.nodes.clone();
        let edges = self.graph.edges.clone();

        // Create the canvas widget
        let canvas = Canvas::default()
            .block(self.block.unwrap_or_default())
            .x_bounds([min_x, max_x])
            .y_bounds([min_y, max_y])
            .paint(move |ctx| {
                // Build node lookup map for edge rendering
                let node_map: HashMap<String, &GraphNode> =
                    nodes.iter().map(|n| (n.id.clone(), n)).collect();

                // Render edges first (so they appear behind nodes)
                for edge in &edges {
                    render_edge(ctx, edge, &node_map);
                }

                // Render nodes
                for node in &nodes {
                    render_node(ctx, node);
                }
            });

        // Render the canvas
        canvas.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_new() {
        let node = GraphNode::new("node1", 10.0, 20.0, "Test Node");
        assert_eq!(node.id, "node1");
        assert_eq!(node.x, 10.0);
        assert_eq!(node.y, 20.0);
        assert_eq!(node.label, "Test Node");
        assert_eq!(node.width, 8.0);
        assert_eq!(node.height, 4.0);
        assert!(!node.selected);
    }

    #[test]
    fn test_graph_node_builder() {
        let node = GraphNode::builder("node1")
            .position(10.0, 20.0)
            .size(12.0, 6.0)
            .color(Color::Red)
            .label("Custom Node")
            .selected(true)
            .build();

        assert_eq!(node.id, "node1");
        assert_eq!(node.x, 10.0);
        assert_eq!(node.y, 20.0);
        assert_eq!(node.width, 12.0);
        assert_eq!(node.height, 6.0);
        assert_eq!(node.color, Color::Red);
        assert_eq!(node.label, "Custom Node");
        assert!(node.selected);
    }

    #[test]
    fn test_graph_edge_new() {
        let edge = GraphEdge::new("node1", "node2");
        assert_eq!(edge.from, "node1");
        assert_eq!(edge.to, "node2");
        assert_eq!(edge.color, Color::Cyan);
        assert!(edge.label.is_none());
    }

    #[test]
    fn test_graph_edge_builder() {
        let edge = GraphEdge::new("node1", "node2")
            .color(Color::Green)
            .label("100 ev/s");

        assert_eq!(edge.color, Color::Green);
        assert_eq!(edge.label, Some("100 ev/s".to_string()));
    }

    #[test]
    fn test_graph_data_new() {
        let graph = GraphData::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_graph_data_add() {
        let mut graph = GraphData::new();
        graph.add_node(GraphNode::new("node1", 10.0, 20.0, "Node 1"));
        graph.add_edge(GraphEdge::new("node1", "node2"));

        assert_eq!(graph.nodes.len(), 1);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_graph_data_find_node() {
        let mut graph = GraphData::new();
        graph.add_node(GraphNode::new("node1", 10.0, 20.0, "Node 1"));

        let found = graph.find_node("node1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "node1");

        let not_found = graph.find_node("node2");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_graph_data_edges_from() {
        let mut graph = GraphData::new();
        graph.add_edge(GraphEdge::new("node1", "node2"));
        graph.add_edge(GraphEdge::new("node1", "node3"));
        graph.add_edge(GraphEdge::new("node2", "node3"));

        let edges = graph.edges_from("node1");
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_graph_data_edges_to() {
        let mut graph = GraphData::new();
        graph.add_edge(GraphEdge::new("node1", "node3"));
        graph.add_edge(GraphEdge::new("node2", "node3"));
        graph.add_edge(GraphEdge::new("node3", "node4"));

        let edges = graph.edges_to("node3");
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn test_graph_canvas_new() {
        let graph = GraphData::new();
        let canvas = GraphCanvas::new(&graph);

        let (min_x, max_x, min_y, max_y) = canvas.get_bounds();
        // Empty graph should have default bounds
        assert_eq!(min_x, 0.0);
        assert_eq!(max_x, 100.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_y, 100.0);
    }

    #[test]
    fn test_graph_canvas_custom_bounds() {
        let graph = GraphData::new();
        let canvas = GraphCanvas::new(&graph)
            .x_bounds(-50.0, 50.0)
            .y_bounds(-25.0, 25.0);

        let (min_x, max_x, min_y, max_y) = canvas.get_bounds();
        assert_eq!(min_x, -50.0);
        assert_eq!(max_x, 50.0);
        assert_eq!(min_y, -25.0);
        assert_eq!(max_y, 25.0);
    }
}
