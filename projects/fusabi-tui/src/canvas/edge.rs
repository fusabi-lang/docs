//! Edge rendering for graph canvas
//!
//! This module provides utilities for rendering edges between graph nodes,
//! including directional arrows and optional labels.

use super::graph::{GraphEdge, GraphNode};
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line as CanvasLine, Points};
use std::collections::HashMap;

/// Render an edge between two nodes on the canvas
///
/// This function draws a line from the source node to the destination node,
/// including an arrow head to indicate direction. If the edge has a label,
/// a visual indicator is drawn at the midpoint.
///
/// # Arguments
///
/// * `ctx` - Canvas context to draw on
/// * `edge` - Edge data including source, destination, color, and label
/// * `nodes` - Map of node IDs to node data for position lookup
///
/// # Example
///
/// ```
/// use fusabi_tui::canvas::edge::render_edge;
/// use fusabi_tui::canvas::graph::{GraphNode, GraphEdge};
/// use ratatui::style::Color;
/// use std::collections::HashMap;
///
/// let mut nodes = HashMap::new();
/// nodes.insert("node1".to_string(), GraphNode {
///     id: "node1".to_string(),
///     x: 10.0,
///     y: 20.0,
///     width: 8.0,
///     height: 4.0,
///     color: Color::Blue,
///     label: "Node 1".to_string(),
///     selected: false,
/// });
///
/// let edge = GraphEdge {
///     from: "node1".to_string(),
///     to: "node2".to_string(),
///     color: Color::Cyan,
///     label: Some("100 ev/s".to_string()),
/// };
///
/// // In actual use, call with a canvas context:
/// // render_edge(&mut ctx, &edge, &nodes);
/// ```
pub fn render_edge(ctx: &mut Context, edge: &GraphEdge, nodes: &HashMap<String, &GraphNode>) {
    // Look up source and destination nodes
    let from_node = match nodes.get(&edge.from) {
        Some(node) => node,
        None => return, // Skip if source node not found
    };

    let to_node = match nodes.get(&edge.to) {
        Some(node) => node,
        None => return, // Skip if destination node not found
    };

    // Calculate edge endpoints at node centers
    let x1 = from_node.x + from_node.width / 2.0;
    let y1 = from_node.y + from_node.height / 2.0;
    let x2 = to_node.x + to_node.width / 2.0;
    let y2 = to_node.y + to_node.height / 2.0;

    // Calculate direction vector
    let dx = x2 - x1;
    let dy = y2 - y1;
    let length = (dx * dx + dy * dy).sqrt();

    if length < 0.001 {
        return; // Skip zero-length edges
    }

    // Unit vector in direction of edge
    let ux = dx / length;
    let uy = dy / length;

    // Adjust endpoints to stop at node boundaries
    let offset = 2.0; // Distance from node edge
    let start_x = x1 + offset * ux;
    let start_y = y1 + offset * uy;
    let end_x = x2 - offset * ux;
    let end_y = y2 - offset * uy;

    // Draw main line
    ctx.draw(&CanvasLine {
        x1: start_x,
        y1: start_y,
        x2: end_x,
        y2: end_y,
        color: edge.color,
    });

    // Draw arrow head
    draw_arrow_head(ctx, end_x, end_y, ux, uy, edge.color);

    // Draw label indicator if present
    if edge.label.is_some() {
        draw_label_indicator(ctx, start_x, start_y, end_x, end_y, edge.color);
    }
}

/// Draw an arrow head at the end of an edge
///
/// Creates a simple triangle pointing in the direction of the edge.
///
/// # Arguments
///
/// * `ctx` - Canvas context to draw on
/// * `tip_x` - X coordinate of arrow tip
/// * `tip_y` - Y coordinate of arrow tip
/// * `ux` - X component of unit direction vector
/// * `uy` - Y component of unit direction vector
/// * `color` - Color for the arrow head
fn draw_arrow_head(ctx: &mut Context, tip_x: f64, tip_y: f64, ux: f64, uy: f64, color: Color) {
    let arrow_size = 1.5;

    // Calculate arrow base points (perpendicular to direction)
    // Create a triangle: tip -> base1, tip -> base2
    let base1_x = tip_x - arrow_size * ux + arrow_size * 0.5 * uy;
    let base1_y = tip_y - arrow_size * uy - arrow_size * 0.5 * ux;
    let base2_x = tip_x - arrow_size * ux - arrow_size * 0.5 * uy;
    let base2_y = tip_y - arrow_size * uy + arrow_size * 0.5 * ux;

    // Draw two lines from tip to base points
    ctx.draw(&CanvasLine {
        x1: tip_x,
        y1: tip_y,
        x2: base1_x,
        y2: base1_y,
        color,
    });

    ctx.draw(&CanvasLine {
        x1: tip_x,
        y1: tip_y,
        x2: base2_x,
        y2: base2_y,
        color,
    });

    // Optionally draw the base of the arrow triangle
    ctx.draw(&CanvasLine {
        x1: base1_x,
        y1: base1_y,
        x2: base2_x,
        y2: base2_y,
        color,
    });
}

/// Draw a visual indicator at the edge midpoint for labeled edges
///
/// Uses a colored point to indicate that the edge has associated data.
/// In the future, this could be enhanced to show actual text labels.
///
/// # Arguments
///
/// * `ctx` - Canvas context to draw on
/// * `x1` - Start X coordinate
/// * `y1` - Start Y coordinate
/// * `x2` - End X coordinate
/// * `y2` - End Y coordinate
/// * `color` - Color for the indicator
fn draw_label_indicator(ctx: &mut Context, x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
    let mid_x = (x1 + x2) / 2.0;
    let mid_y = (y1 + y2) / 2.0;

    // Draw a point at the midpoint
    ctx.draw(&Points {
        coords: &[(mid_x, mid_y)],
        color,
    });
}

/// Calculate arrow head geometry
///
/// Returns the coordinates of the arrow head triangle points.
///
/// # Arguments
///
/// * `tip_x` - X coordinate of arrow tip
/// * `tip_y` - Y coordinate of arrow tip
/// * `ux` - X component of unit direction vector
/// * `uy` - Y component of unit direction vector
/// * `size` - Size of the arrow head
///
/// # Returns
///
/// Tuple of ((base1_x, base1_y), (base2_x, base2_y))
pub fn calculate_arrow_points(
    tip_x: f64,
    tip_y: f64,
    ux: f64,
    uy: f64,
    size: f64,
) -> ((f64, f64), (f64, f64)) {
    let base1_x = tip_x - size * ux + size * 0.5 * uy;
    let base1_y = tip_y - size * uy - size * 0.5 * ux;
    let base2_x = tip_x - size * ux - size * 0.5 * uy;
    let base2_y = tip_y - size * uy + size * 0.5 * ux;

    ((base1_x, base1_y), (base2_x, base2_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrow_points_horizontal() {
        // Arrow pointing right
        let (base1, base2) = calculate_arrow_points(10.0, 5.0, 1.0, 0.0, 2.0);

        // Base should be to the left of tip
        assert!(base1.0 < 10.0);
        assert!(base2.0 < 10.0);

        // Base points should be symmetric around the center line
        assert!((base1.1 - 5.0).abs() - (5.0 - base2.1).abs() < 0.001);
    }

    #[test]
    fn test_arrow_points_vertical() {
        // Arrow pointing up
        let (base1, base2) = calculate_arrow_points(5.0, 10.0, 0.0, 1.0, 2.0);

        // Base should be below tip
        assert!(base1.1 < 10.0);
        assert!(base2.1 < 10.0);

        // Base points should be symmetric around the center line
        assert!((base1.0 - 5.0).abs() - (5.0 - base2.0).abs() < 0.001);
    }

    #[test]
    fn test_arrow_points_diagonal() {
        // Arrow pointing northeast (45 degrees)
        let ux = 1.0 / 2.0_f64.sqrt();
        let uy = 1.0 / 2.0_f64.sqrt();
        let (base1, base2) = calculate_arrow_points(10.0, 10.0, ux, uy, 2.0);

        // Both base points should be behind (lower and to the left of) the tip
        assert!(base1.0 < 10.0);
        assert!(base1.1 < 10.0);
        assert!(base2.0 < 10.0);
        assert!(base2.1 < 10.0);
    }

    #[test]
    fn test_arrow_size_scaling() {
        let (base1_small, _) = calculate_arrow_points(10.0, 10.0, 1.0, 0.0, 1.0);
        let (base1_large, _) = calculate_arrow_points(10.0, 10.0, 1.0, 0.0, 2.0);

        // Larger arrow should have base further from tip
        let dist_small = 10.0 - base1_small.0;
        let dist_large = 10.0 - base1_large.0;
        assert!(dist_large > dist_small);
    }
}
