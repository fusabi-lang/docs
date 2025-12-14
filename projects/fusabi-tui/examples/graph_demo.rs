//! Graph visualization demo
//!
//! Demonstrates the GraphCanvas widget with a sample directed graph
//! showing nodes connected with labeled edges.

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use fusabi_tui::canvas::{GraphCanvas, GraphData, GraphEdge, GraphNode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

struct App {
    graph: GraphData,
    selected_node: Option<String>,
}

impl App {
    fn new() -> Self {
        let mut graph = GraphData::new();

        // Create a sample processing pipeline graph
        // Input source
        graph.add_node(
            GraphNode::builder("source")
                .position(10.0, 30.0)
                .size(10.0, 6.0)
                .color(Color::Green)
                .label("Source")
                .build(),
        );

        // Processing stages
        graph.add_node(
            GraphNode::builder("parser")
                .position(35.0, 30.0)
                .size(10.0, 6.0)
                .color(Color::Blue)
                .label("Parser")
                .build(),
        );

        graph.add_node(
            GraphNode::builder("filter")
                .position(60.0, 20.0)
                .size(10.0, 6.0)
                .color(Color::Blue)
                .label("Filter")
                .build(),
        );

        graph.add_node(
            GraphNode::builder("transform")
                .position(60.0, 40.0)
                .size(10.0, 6.0)
                .color(Color::Blue)
                .label("Transform")
                .build(),
        );

        // Output sink
        graph.add_node(
            GraphNode::builder("sink")
                .position(85.0, 30.0)
                .size(10.0, 6.0)
                .color(Color::Magenta)
                .label("Sink")
                .build(),
        );

        // Add edges with labels
        graph.add_edge(
            GraphEdge::new("source", "parser")
                .color(Color::Cyan)
                .label("1000 ev/s"),
        );

        graph.add_edge(
            GraphEdge::new("parser", "filter")
                .color(Color::Cyan)
                .label("800 ev/s"),
        );

        graph.add_edge(
            GraphEdge::new("parser", "transform")
                .color(Color::Cyan)
                .label("200 ev/s"),
        );

        graph.add_edge(
            GraphEdge::new("filter", "sink")
                .color(Color::Green)
                .label("800 ev/s"),
        );

        graph.add_edge(
            GraphEdge::new("transform", "sink")
                .color(Color::Green)
                .label("200 ev/s"),
        );

        Self {
            graph,
            selected_node: None,
        }
    }

    fn select_next_node(&mut self) {
        if self.graph.nodes.is_empty() {
            return;
        }

        // Clear current selection
        for node in &mut self.graph.nodes {
            node.selected = false;
        }

        // Find next node to select
        let current_idx = self.selected_node.as_ref().and_then(|id| {
            self.graph.nodes.iter().position(|n| &n.id == id)
        });

        let next_idx = match current_idx {
            Some(idx) => (idx + 1) % self.graph.nodes.len(),
            None => 0,
        };

        self.graph.nodes[next_idx].selected = true;
        self.selected_node = Some(self.graph.nodes[next_idx].id.clone());
    }

    fn get_node_info(&self) -> Vec<Line> {
        if let Some(ref selected_id) = self.selected_node {
            if let Some(node) = self.graph.find_node(selected_id) {
                let mut lines = vec![
                    Line::from(vec![
                        Span::styled("Selected: ", Style::default().fg(Color::Yellow)),
                        Span::raw(node.label.clone()),
                    ]),
                    Line::from(vec![
                        Span::raw("ID: "),
                        Span::styled(node.id.clone(), Style::default().fg(Color::Cyan)),
                    ]),
                    Line::from(format!("Position: ({:.1}, {:.1})", node.x, node.y)),
                    Line::from(""),
                ];

                // Show incoming edges
                let incoming = self.graph.edges_to(&node.id);
                if !incoming.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Incoming:",
                        Style::default().fg(Color::Green),
                    )));
                    for edge in incoming {
                        let label = edge.label.as_deref().unwrap_or("no label");
                        lines.push(Line::from(format!("  {} -> {} ({})", edge.from, edge.to, label)));
                    }
                }

                // Show outgoing edges
                let outgoing = self.graph.edges_from(&node.id);
                if !outgoing.is_empty() {
                    lines.push(Line::from(Span::styled(
                        "Outgoing:",
                        Style::default().fg(Color::Magenta),
                    )));
                    for edge in outgoing {
                        let label = edge.label.as_deref().unwrap_or("no label");
                        lines.push(Line::from(format!("  {} -> {} ({})", edge.from, edge.to, label)));
                    }
                }

                return lines;
            }
        }

        vec![
            Line::from("Press Tab to select nodes"),
            Line::from("Press Q to quit"),
        ]
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(f.area());

            // Render graph
            let graph_widget = GraphCanvas::new(&app.graph).block(
                Block::default()
                    .title("Graph Visualization Demo")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            );
            f.render_widget(graph_widget, chunks[0]);

            // Render info panel
            let info = Paragraph::new(app.get_node_info())
                .block(
                    Block::default()
                        .title("Node Info")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::White)),
                )
                .style(Style::default().fg(Color::White));
            f.render_widget(info, chunks[1]);
        })?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Tab => app.select_next_node(),
                _ => {}
            }
        }
    }
}
