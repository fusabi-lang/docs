//! Node rendering for graph canvas
//!
//! This module provides utilities for rendering graph nodes with
//! selection highlighting and visual styling.

use super::graph::GraphNode;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Points, Rectangle};

/// Render a node on the canvas
///
/// Draws a rectangular node with its color, and adds a highlight
/// border if the node is selected.
///
/// # Arguments
///
/// * `ctx` - Canvas context to draw on
/// * `node` - Node data including position, size, color, and selection state
///
/// # Example
///
/// ```
/// use fusabi_tui::canvas::node::render_node;
/// use fusabi_tui::canvas::graph::GraphNode;
/// use ratatui::style::Color;
///
/// let node = GraphNode {
///     id: "node1".to_string(),
///     x: 10.0,
///     y: 20.0,
///     width: 8.0,
///     height: 4.0,
///     color: Color::Blue,
///     label: "Node 1".to_string(),
///     selected: false,
/// };
///
/// // In actual use, call with a canvas context:
/// // render_node(&mut ctx, &node);
/// ```
pub fn render_node(ctx: &mut Context, node: &GraphNode) {
    // Draw selection highlight first (behind the node)
    if node.selected {
        draw_selection_highlight(ctx, node);
    }

    // Draw main node rectangle
    ctx.draw(&Rectangle {
        x: node.x,
        y: node.y,
        width: node.width,
        height: node.height,
        color: node.color,
    });

    // Draw center point for visual emphasis
    // This helps identify the node center on low-resolution terminals
    let center_x = node.x + node.width / 2.0;
    let center_y = node.y + node.height / 2.0;

    ctx.draw(&Points {
        coords: &[(center_x, center_y)],
        color: node.color,
    });
}

/// Draw a selection highlight border around a node
///
/// Renders a larger rectangle behind the node in a highlight color
/// to indicate that the node is currently selected.
///
/// # Arguments
///
/// * `ctx` - Canvas context to draw on
/// * `node` - Selected node to highlight
fn draw_selection_highlight(ctx: &mut Context, node: &GraphNode) {
    let border_width = 1.0;
    let highlight_color = Color::Yellow;

    // Draw outer rectangle as selection border
    ctx.draw(&Rectangle {
        x: node.x - border_width,
        y: node.y - border_width,
        width: node.width + 2.0 * border_width,
        height: node.height + 2.0 * border_width,
        color: highlight_color,
    });

    // Draw corner points for emphasis
    let corners = [
        (node.x - border_width, node.y - border_width),
        (
            node.x + node.width + border_width,
            node.y - border_width,
        ),
        (
            node.x - border_width,
            node.y + node.height + border_width,
        ),
        (
            node.x + node.width + border_width,
            node.y + node.height + border_width,
        ),
    ];

    ctx.draw(&Points {
        coords: &corners,
        color: highlight_color,
    });
}

/// Get node color based on health status
///
/// Helper function to determine node color based on various states.
/// Can be extended to support health metrics, warnings, etc.
///
/// # Arguments
///
/// * `node` - Node to get color for
/// * `is_error` - Whether the node is in an error state
/// * `is_warning` - Whether the node has warnings
///
/// # Returns
///
/// Appropriate color for the node's current state
pub fn get_node_color(node: &GraphNode, is_error: bool, is_warning: bool) -> Color {
    if is_error {
        Color::Red
    } else if is_warning {
        Color::Yellow
    } else if node.selected {
        Color::Green
    } else {
        node.color
    }
}

/// Calculate node center point
///
/// Returns the (x, y) coordinates of the node's center.
///
/// # Arguments
///
/// * `node` - Node to calculate center for
///
/// # Returns
///
/// Tuple of (center_x, center_y)
pub fn calculate_node_center(node: &GraphNode) -> (f64, f64) {
    (
        node.x + node.width / 2.0,
        node.y + node.height / 2.0,
    )
}

/// Check if a point is inside a node's bounds
///
/// Useful for hit testing and mouse interaction.
///
/// # Arguments
///
/// * `node` - Node to test against
/// * `x` - X coordinate to test
/// * `y` - Y coordinate to test
///
/// # Returns
///
/// True if the point is inside the node's bounding rectangle
pub fn point_in_node(node: &GraphNode, x: f64, y: f64) -> bool {
    x >= node.x
        && x <= node.x + node.width
        && y >= node.y
        && y <= node.y + node.height
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(x: f64, y: f64, selected: bool) -> GraphNode {
        GraphNode {
            id: "test".to_string(),
            x,
            y,
            width: 8.0,
            height: 4.0,
            color: Color::Blue,
            label: "Test Node".to_string(),
            selected,
        }
    }

    #[test]
    fn test_calculate_node_center() {
        let node = create_test_node(10.0, 20.0, false);
        let (cx, cy) = calculate_node_center(&node);

        assert_eq!(cx, 14.0); // 10 + 8/2
        assert_eq!(cy, 22.0); // 20 + 4/2
    }

    #[test]
    fn test_point_in_node_inside() {
        let node = create_test_node(10.0, 20.0, false);

        // Test points inside the node
        assert!(point_in_node(&node, 10.0, 20.0)); // Top-left corner
        assert!(point_in_node(&node, 18.0, 24.0)); // Bottom-right corner
        assert!(point_in_node(&node, 14.0, 22.0)); // Center
    }

    #[test]
    fn test_point_in_node_outside() {
        let node = create_test_node(10.0, 20.0, false);

        // Test points outside the node
        assert!(!point_in_node(&node, 9.9, 22.0)); // Just left
        assert!(!point_in_node(&node, 18.1, 22.0)); // Just right
        assert!(!point_in_node(&node, 14.0, 19.9)); // Just above
        assert!(!point_in_node(&node, 14.0, 24.1)); // Just below
    }

    #[test]
    fn test_get_node_color_error() {
        let node = create_test_node(10.0, 20.0, false);
        let color = get_node_color(&node, true, false);
        assert_eq!(color, Color::Red);
    }

    #[test]
    fn test_get_node_color_warning() {
        let node = create_test_node(10.0, 20.0, false);
        let color = get_node_color(&node, false, true);
        assert_eq!(color, Color::Yellow);
    }

    #[test]
    fn test_get_node_color_selected() {
        let node = create_test_node(10.0, 20.0, true);
        let color = get_node_color(&node, false, false);
        assert_eq!(color, Color::Green);
    }

    #[test]
    fn test_get_node_color_normal() {
        let node = create_test_node(10.0, 20.0, false);
        let color = get_node_color(&node, false, false);
        assert_eq!(color, Color::Blue);
    }

    #[test]
    fn test_point_in_node_edge_cases() {
        let node = GraphNode {
            id: "test".to_string(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            color: Color::Blue,
            label: "Test".to_string(),
            selected: false,
        };

        // Test exact boundaries
        assert!(point_in_node(&node, 0.0, 0.0));
        assert!(point_in_node(&node, 10.0, 10.0));
        assert!(point_in_node(&node, 0.0, 10.0));
        assert!(point_in_node(&node, 10.0, 0.0));
    }
}
