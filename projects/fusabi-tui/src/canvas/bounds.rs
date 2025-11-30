//! Bounds calculation for graph canvas
//!
//! This module provides utilities for calculating the bounding box
//! of graph nodes to properly set up canvas coordinate systems.

use super::graph::GraphNode;

/// Calculate canvas bounds from node positions
///
/// Returns (min_x, max_x, min_y, max_y) with padding to ensure
/// all nodes are visible and there's space around the edges.
///
/// # Arguments
///
/// * `nodes` - Slice of graph nodes to calculate bounds for
///
/// # Returns
///
/// Tuple of (min_x, max_x, min_y, max_y) in canvas coordinates
///
/// # Example
///
/// ```
/// use fusabi_tui::canvas::bounds::calculate_bounds;
/// use fusabi_tui::canvas::graph::GraphNode;
/// use ratatui::style::Color;
///
/// let nodes = vec![
///     GraphNode {
///         id: "node1".to_string(),
///         x: 10.0,
///         y: 20.0,
///         width: 8.0,
///         height: 4.0,
///         color: Color::Blue,
///         label: "Node 1".to_string(),
///         selected: false,
///     },
/// ];
///
/// let (min_x, max_x, min_y, max_y) = calculate_bounds(&nodes);
/// assert!(min_x <= 10.0);
/// assert!(max_x >= 10.0);
/// ```
pub fn calculate_bounds(nodes: &[GraphNode]) -> (f64, f64, f64, f64) {
    if nodes.is_empty() {
        return (0.0, 100.0, 0.0, 100.0);
    }

    // Calculate initial bounds including node dimensions
    let mut min_x = nodes[0].x;
    let mut max_x = nodes[0].x + nodes[0].width;
    let mut min_y = nodes[0].y;
    let mut max_y = nodes[0].y + nodes[0].height;

    for node in nodes.iter().skip(1) {
        min_x = min_x.min(node.x);
        max_x = max_x.max(node.x + node.width);
        min_y = min_y.min(node.y);
        max_y = max_y.max(node.y + node.height);
    }

    // Add padding (10% of the range or minimum 5 units)
    let x_range = max_x - min_x;
    let y_range = max_y - min_y;
    let x_padding = (x_range * 0.1).max(5.0);
    let y_padding = (y_range * 0.1).max(5.0);

    min_x -= x_padding;
    max_x += x_padding;
    min_y -= y_padding;
    max_y += y_padding;

    // Ensure minimum size
    if max_x - min_x < 20.0 {
        let center = (min_x + max_x) / 2.0;
        min_x = center - 10.0;
        max_x = center + 10.0;
    }
    if max_y - min_y < 20.0 {
        let center = (min_y + max_y) / 2.0;
        min_y = center - 10.0;
        max_y = center + 10.0;
    }

    (min_x, max_x, min_y, max_y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Color;

    fn create_test_node(id: &str, x: f64, y: f64) -> GraphNode {
        GraphNode {
            id: id.to_string(),
            x,
            y,
            width: 8.0,
            height: 4.0,
            color: Color::Blue,
            label: id.to_string(),
            selected: false,
        }
    }

    #[test]
    fn test_empty_nodes() {
        let nodes = vec![];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&nodes);
        assert_eq!(min_x, 0.0);
        assert_eq!(max_x, 100.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_y, 100.0);
    }

    #[test]
    fn test_single_node() {
        let nodes = vec![create_test_node("node1", 10.0, 20.0)];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&nodes);

        // Should have padding and ensure minimum size
        assert!(min_x < 10.0);
        assert!(max_x > 18.0); // 10 + 8 (width)
        assert!(min_y < 20.0);
        assert!(max_y > 24.0); // 20 + 4 (height)

        // Minimum size enforced
        assert!((max_x - min_x) >= 20.0);
        assert!((max_y - min_y) >= 20.0);
    }

    #[test]
    fn test_multiple_nodes() {
        let nodes = vec![
            create_test_node("node1", 0.0, 0.0),
            create_test_node("node2", 50.0, 50.0),
            create_test_node("node3", 100.0, 100.0),
        ];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&nodes);

        // Should include all nodes with padding
        assert!(min_x < 0.0);
        assert!(max_x > 108.0); // 100 + 8 (width)
        assert!(min_y < 0.0);
        assert!(max_y > 104.0); // 100 + 4 (height)
    }

    #[test]
    fn test_bounds_with_negative_coordinates() {
        let nodes = vec![
            create_test_node("node1", -50.0, -30.0),
            create_test_node("node2", 50.0, 30.0),
        ];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&nodes);

        // Should handle negative coordinates properly
        assert!(min_x < -50.0);
        assert!(max_x > 58.0); // 50 + 8 (width)
        assert!(min_y < -30.0);
        assert!(max_y > 34.0); // 30 + 4 (height)
    }

    #[test]
    fn test_bounds_with_clustered_nodes() {
        // Nodes very close together - should still have minimum size
        let nodes = vec![
            create_test_node("node1", 50.0, 50.0),
            create_test_node("node2", 51.0, 51.0),
            create_test_node("node3", 52.0, 52.0),
        ];
        let (min_x, max_x, min_y, max_y) = calculate_bounds(&nodes);

        // Minimum size should be enforced
        assert!((max_x - min_x) >= 20.0);
        assert!((max_y - min_y) >= 20.0);
    }
}
