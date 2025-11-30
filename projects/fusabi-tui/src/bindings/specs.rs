//! Widget specification structures
//!
//! This module provides JSON-serializable structures that F# scripts can build
//! to specify widget configurations. The Rust application interprets these
//! specifications and renders the actual widgets.
//!
//! This is a workaround for the Fusabi Send+Sync limitation - instead of
//! managing widget lifecycles in Fusabi, we build declarative specifications
//! that can be passed as data.

use serde::{Deserialize, Serialize};

/// Table specification that can be built in F# and interpreted in Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSpec {
    /// Table title (optional)
    pub title: Option<String>,
    /// Column definitions
    pub columns: Vec<ColumnSpec>,
    /// Data rows (each row is a Vec of cell strings)
    pub rows: Vec<Vec<String>>,
    /// Show borders (default: true)
    pub borders: bool,
    /// Highlight symbol (default: ">> ")
    pub highlight_symbol: Option<String>,
    /// Enable highlighting (default: true)
    pub highlight: bool,
}

impl Default for TableSpec {
    fn default() -> Self {
        Self {
            title: None,
            columns: Vec::new(),
            rows: Vec::new(),
            borders: true,
            highlight_symbol: Some(">> ".to_string()),
            highlight: true,
        }
    }
}

/// Column specification for a table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSpec {
    /// Column header text
    pub header: String,
    /// Column width as percentage (0-100) or fixed width
    pub width: WidthSpec,
}

/// Width specification for columns
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum WidthSpec {
    /// Percentage of available space (0-100)
    Percentage(u16),
    /// Fixed character width
    Length(u16),
    /// Minimum width
    Min(u16),
    /// Maximum width
    Max(u16),
}

impl Default for WidthSpec {
    fn default() -> Self {
        WidthSpec::Percentage(10)
    }
}

/// Canvas graph specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSpec {
    /// Graph title
    pub title: String,
    /// Graph type
    pub graph_type: GraphType,
    /// Nodes in the graph
    pub nodes: Vec<NodeSpec>,
    /// Edges between nodes
    pub edges: Vec<EdgeSpec>,
}

/// Type of graph to render
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphType {
    /// Directed graph
    Directed,
    /// Undirected graph
    Undirected,
}

/// Node specification for canvas graphs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSpec {
    /// Node ID (unique identifier)
    pub id: String,
    /// Display label
    pub label: String,
    /// X position (0.0 to 1.0, relative to canvas width)
    pub x: f64,
    /// Y position (0.0 to 1.0, relative to canvas height)
    pub y: f64,
    /// Optional color (hex string like "#FF0000")
    pub color: Option<String>,
}

/// Edge specification for canvas graphs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeSpec {
    /// Source node ID
    pub from: String,
    /// Target node ID
    pub to: String,
    /// Optional edge label
    pub label: Option<String>,
    /// Optional color (hex string)
    pub color: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_table_spec_default() {
        let spec = TableSpec::default();
        assert_eq!(spec.title, None);
        assert_eq!(spec.columns.len(), 0);
        assert_eq!(spec.rows.len(), 0);
        assert!(spec.borders);
        assert!(spec.highlight);
        assert_eq!(spec.highlight_symbol, Some(">> ".to_string()));
    }

    #[test]
    fn test_table_spec_serialization() {
        let spec = TableSpec {
            title: Some("Test Table".to_string()),
            columns: vec![
                ColumnSpec {
                    header: "Name".to_string(),
                    width: WidthSpec::Percentage(50),
                },
                ColumnSpec {
                    header: "Value".to_string(),
                    width: WidthSpec::Length(20),
                },
            ],
            rows: vec![
                vec!["Alice".to_string(), "100".to_string()],
                vec!["Bob".to_string(), "200".to_string()],
            ],
            borders: true,
            highlight_symbol: Some("->".to_string()),
            highlight: true,
        };

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: TableSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, spec.title);
        assert_eq!(deserialized.columns.len(), 2);
        assert_eq!(deserialized.rows.len(), 2);
    }

    #[test]
    fn test_width_spec_variants() {
        let percentage = WidthSpec::Percentage(50);
        let length = WidthSpec::Length(20);
        let min = WidthSpec::Min(10);
        let max = WidthSpec::Max(100);

        let json_p = serde_json::to_string(&percentage).unwrap();
        let json_l = serde_json::to_string(&length).unwrap();
        let json_min = serde_json::to_string(&min).unwrap();
        let json_max = serde_json::to_string(&max).unwrap();

        assert!(json_p.contains("Percentage"));
        assert!(json_l.contains("Length"));
        assert!(json_min.contains("Min"));
        assert!(json_max.contains("Max"));
    }

    #[test]
    fn test_graph_spec_creation() {
        let spec = GraphSpec {
            title: "Test Graph".to_string(),
            graph_type: GraphType::Directed,
            nodes: vec![
                NodeSpec {
                    id: "n1".to_string(),
                    label: "Node 1".to_string(),
                    x: 0.25,
                    y: 0.5,
                    color: Some("#FF0000".to_string()),
                },
                NodeSpec {
                    id: "n2".to_string(),
                    label: "Node 2".to_string(),
                    x: 0.75,
                    y: 0.5,
                    color: None,
                },
            ],
            edges: vec![EdgeSpec {
                from: "n1".to_string(),
                to: "n2".to_string(),
                label: Some("connects".to_string()),
                color: None,
            }],
        };

        let json = serde_json::to_string(&spec).unwrap();
        let deserialized: GraphSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, "Test Graph");
        assert_eq!(deserialized.nodes.len(), 2);
        assert_eq!(deserialized.edges.len(), 1);
    }

    #[test]
    fn test_node_spec_serialization() {
        let node = NodeSpec {
            id: "test".to_string(),
            label: "Test Node".to_string(),
            x: 0.5,
            y: 0.5,
            color: Some("#00FF00".to_string()),
        };

        let json = serde_json::to_string(&node).unwrap();
        let deserialized: NodeSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, "test");
        assert_eq!(deserialized.label, "Test Node");
        assert_eq!(deserialized.x, 0.5);
        assert_eq!(deserialized.y, 0.5);
        assert_eq!(deserialized.color, Some("#00FF00".to_string()));
    }

    #[test]
    fn test_edge_spec_serialization() {
        let edge = EdgeSpec {
            from: "n1".to_string(),
            to: "n2".to_string(),
            label: Some("edge label".to_string()),
            color: Some("#0000FF".to_string()),
        };

        let json = serde_json::to_string(&edge).unwrap();
        let deserialized: EdgeSpec = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.from, "n1");
        assert_eq!(deserialized.to, "n2");
        assert_eq!(deserialized.label, Some("edge label".to_string()));
        assert_eq!(deserialized.color, Some("#0000FF".to_string()));
    }
}
