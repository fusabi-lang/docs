//! Table widget demonstration
//!
//! This example shows how to use the generic table widget with custom data types,
//! column definitions, and styling - similar to Hibana's component table.

use fusabi_tui::{ColumnDef, TableData, render_table, Result};
use ratatui::{
    backend::CrosstermBackend,
    layout::Constraint,
    style::{Color, Style},
    widgets::Cell,
    Terminal,
};
use std::io;

#[derive(Clone, Debug)]
struct ComponentMetric {
    name: String,
    component_type: ComponentType,
    events_in: u64,
    events_out: u64,
    throughput_eps: f64,
    latency_p95_us: u64,
    errors: u64,
    health: HealthStatus,
}

#[derive(Clone, Debug, PartialEq)]
enum ComponentType {
    Source,
    Transform,
    Sink,
}

impl ComponentType {
    fn as_str(&self) -> &str {
        match self {
            ComponentType::Source => "Source",
            ComponentType::Transform => "Transform",
            ComponentType::Sink => "Sink",
        }
    }

    fn color(&self) -> Color {
        match self {
            ComponentType::Source => Color::Green,
            ComponentType::Transform => Color::Yellow,
            ComponentType::Sink => Color::Blue,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    fn as_str(&self) -> &str {
        match self {
            HealthStatus::Healthy => "HEALTHY",
            HealthStatus::Degraded => "DEGRADED",
            HealthStatus::Unhealthy => "UNHEALTHY",
        }
    }

    fn color(&self) -> Color {
        match self {
            HealthStatus::Healthy => Color::Green,
            HealthStatus::Degraded => Color::Yellow,
            HealthStatus::Unhealthy => Color::Red,
        }
    }
}

fn format_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.2}B", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.2}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.2}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_latency(us: u64) -> String {
    if us >= 1_000_000 {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    } else if us >= 1_000 {
        format!("{:.2}ms", us as f64 / 1_000.0)
    } else {
        format!("{}μs", us)
    }
}

fn main() -> Result<()> {
    // Create sample data (similar to Hibana's component metrics)
    let metrics = vec![
        ComponentMetric {
            name: "gpu_metrics_source".to_string(),
            component_type: ComponentType::Source,
            events_in: 0,
            events_out: 125_000,
            throughput_eps: 1250.5,
            latency_p95_us: 150,
            errors: 0,
            health: HealthStatus::Healthy,
        },
        ComponentMetric {
            name: "json_parser".to_string(),
            component_type: ComponentType::Transform,
            events_in: 125_000,
            events_out: 125_000,
            throughput_eps: 1250.5,
            latency_p95_us: 85,
            errors: 12,
            health: HealthStatus::Degraded,
        },
        ComponentMetric {
            name: "enrichment".to_string(),
            component_type: ComponentType::Transform,
            events_in: 125_000,
            events_out: 125_000,
            throughput_eps: 1248.2,
            latency_p95_us: 320,
            errors: 0,
            health: HealthStatus::Healthy,
        },
        ComponentMetric {
            name: "http_sink".to_string(),
            component_type: ComponentType::Sink,
            events_in: 125_000,
            events_out: 120_000,
            throughput_eps: 1200.0,
            latency_p95_us: 5_500,
            errors: 5_000,
            health: HealthStatus::Unhealthy,
        },
    ];

    // Create table with column definitions (extracted pattern from Hibana)
    let table = TableData::new()
        .title("Components (sorted by throughput)")
        .column(ColumnDef::new(
            "Component",
            Constraint::Percentage(20),
            |m: &ComponentMetric| Cell::from(m.name.clone()),
        ))
        .column(ColumnDef::new(
            "Type",
            Constraint::Percentage(10),
            |m: &ComponentMetric| {
                Cell::from(m.component_type.as_str().to_string())
                    .style(Style::default().fg(m.component_type.color()))
            },
        ))
        .column(ColumnDef::new(
            "Events In",
            Constraint::Percentage(10),
            |m: &ComponentMetric| Cell::from(format_number(m.events_in)),
        ))
        .column(ColumnDef::new(
            "Events Out",
            Constraint::Percentage(10),
            |m: &ComponentMetric| Cell::from(format_number(m.events_out)),
        ))
        .column(ColumnDef::new(
            "Throughput",
            Constraint::Percentage(15),
            |m: &ComponentMetric| Cell::from(format!("{:.2}/s", m.throughput_eps)),
        ))
        .column(ColumnDef::new(
            "Latency (p95)",
            Constraint::Percentage(12),
            |m: &ComponentMetric| Cell::from(format_latency(m.latency_p95_us)),
        ))
        .column(ColumnDef::new(
            "Errors",
            Constraint::Percentage(10),
            |m: &ComponentMetric| {
                let style = if m.errors > 0 {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                };
                Cell::from(format_number(m.errors)).style(style)
            },
        ))
        .column(ColumnDef::new(
            "Status",
            Constraint::Percentage(13),
            |m: &ComponentMetric| {
                Cell::from(m.health.as_str().to_string())
                    .style(Style::default().fg(m.health.color()))
            },
        ))
        .rows(metrics);

    // Render to terminal
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    terminal.draw(|f| {
        let area = f.area();
        render_table(f, area, &table);
    })?;

    println!("\n\nTable widget demo completed!");
    println!("This demonstrates the generic table extracted from Hibana's UI rendering logic.");

    Ok(())
}
