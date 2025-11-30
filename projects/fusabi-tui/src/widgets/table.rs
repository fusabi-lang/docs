//! Generic table widget for rendering tabular data
//!
//! This module provides a flexible table widget that can render any type of data
//! with custom column definitions and styling. It's extracted from Hibana's table
//! rendering logic and made generic for reuse across different data types.
//!
//! ## Example
//!
//! ```rust
//! use fusabi_tui::widgets::table::{TableData, ColumnDef};
//! use ratatui::widgets::Cell;
//! use ratatui::layout::Constraint;
//! use ratatui::style::{Color, Style};
//!
//! #[derive(Clone)]
//! struct User {
//!     name: String,
//!     age: u32,
//!     status: String,
//! }
//!
//! let users = vec![
//!     User { name: "Alice".to_string(), age: 30, status: "Active".to_string() },
//!     User { name: "Bob".to_string(), age: 25, status: "Inactive".to_string() },
//! ];
//!
//! let table = TableData::new()
//!     .title("Users")
//!     .column(ColumnDef {
//!         header: "Name".to_string(),
//!         width: Constraint::Percentage(40),
//!         render: Box::new(|user: &User| Cell::from(user.name.clone())),
//!     })
//!     .column(ColumnDef {
//!         header: "Age".to_string(),
//!         width: Constraint::Percentage(30),
//!         render: Box::new(|user: &User| Cell::from(user.age.to_string())),
//!     })
//!     .column(ColumnDef {
//!         header: "Status".to_string(),
//!         width: Constraint::Percentage(30),
//!         render: Box::new(|user: &User| {
//!             let color = if user.status == "Active" { Color::Green } else { Color::Red };
//!             Cell::from(user.status.clone()).style(Style::default().fg(color))
//!         }),
//!     })
//!     .rows(users);
//! ```

use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

/// Column definition for a table
///
/// Defines how a column should be rendered, including its header, width constraint,
/// and a rendering function that converts the row data into a Cell.
pub struct ColumnDef<T> {
    /// Column header text
    pub header: String,
    /// Width constraint for this column
    pub width: Constraint,
    /// Function to render a cell from row data
    pub render: Box<dyn Fn(&T) -> Cell<'static>>,
}

impl<T> ColumnDef<T> {
    /// Create a new column definition
    pub fn new(
        header: impl Into<String>,
        width: Constraint,
        render: impl Fn(&T) -> Cell<'static> + 'static,
    ) -> Self {
        Self {
            header: header.into(),
            width,
            render: Box::new(render),
        }
    }
}

/// Generic table data structure
///
/// Holds the data and configuration for rendering a table widget.
/// Use the builder pattern methods to construct a table incrementally.
pub struct TableData<T> {
    /// Rows of data to display
    pub rows: Vec<T>,
    /// Column definitions
    pub columns: Vec<ColumnDef<T>>,
    /// Optional table title
    pub title: Option<String>,
    /// Whether to show borders
    pub borders: bool,
    /// Highlight symbol for selection
    pub highlight_symbol: Option<String>,
    /// Whether to highlight on hover/selection
    pub highlight: bool,
}

impl<T> TableData<T> {
    /// Create a new empty table
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            columns: Vec::new(),
            title: None,
            borders: true,
            highlight_symbol: Some(">> ".to_string()),
            highlight: true,
        }
    }

    /// Set the table title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a column definition
    pub fn column(mut self, def: ColumnDef<T>) -> Self {
        self.columns.push(def);
        self
    }

    /// Set multiple rows of data
    pub fn rows(mut self, rows: Vec<T>) -> Self {
        self.rows = rows;
        self
    }

    /// Set whether to show borders (default: true)
    pub fn borders(mut self, borders: bool) -> Self {
        self.borders = borders;
        self
    }

    /// Set the highlight symbol (default: ">> ")
    pub fn highlight_symbol(mut self, symbol: impl Into<String>) -> Self {
        self.highlight_symbol = Some(symbol.into());
        self
    }

    /// Set whether to enable highlighting (default: true)
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = highlight;
        self
    }

    /// Add a single row
    pub fn row(mut self, row: T) -> Self {
        self.rows.push(row);
        self
    }
}

impl<T> Default for TableData<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Render a generic table to a frame
///
/// This function extracts the rendering logic from Hibana's `draw_components_table`
/// and makes it generic over any data type T. It:
///
/// 1. Creates a header row with yellow styling
/// 2. Applies column render functions to each row
/// 3. Constructs a Ratatui Table widget with borders
/// 4. Renders to the provided frame area
///
/// ## Arguments
///
/// * `frame` - The Ratatui frame to render to
/// * `area` - The rectangular area to render the table in
/// * `data` - The table data containing rows and column definitions
pub fn render_table<T: Clone>(frame: &mut Frame, area: Rect, data: &TableData<T>) {
    // Build header cells with yellow styling (from Hibana's pattern)
    let header_cells: Vec<Cell> = data
        .columns
        .iter()
        .map(|col| Cell::from(col.header.clone()).style(Style::default().fg(Color::Yellow)))
        .collect();

    let header = Row::new(header_cells).height(1);

    // Build data rows by applying each column's render function
    let rows = data.rows.iter().map(|row_data| {
        let cells: Vec<Cell> = data
            .columns
            .iter()
            .map(|col| (col.render)(row_data))
            .collect();
        Row::new(cells)
    });

    // Collect column widths
    let widths: Vec<Constraint> = data.columns.iter().map(|col| col.width).collect();

    // Build the table widget
    let mut table = Table::new(rows, widths).header(header);

    // Add block with borders and optional title
    if data.borders {
        let mut block = Block::default().borders(Borders::ALL);
        if let Some(ref title) = data.title {
            block = block.title(title.clone());
        }
        table = table.block(block);
    }

    // Add highlight styling if enabled
    if data.highlight {
        table = table.highlight_style(Style::default().add_modifier(Modifier::BOLD));
        if let Some(ref symbol) = data.highlight_symbol {
            table = table.highlight_symbol(symbol.clone());
        }
    }

    frame.render_widget(table, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[derive(Clone, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
        status: String,
    }

    #[test]
    fn test_table_data_construction() {
        let table = TableData::<TestData>::new()
            .title("Test Table")
            .column(ColumnDef::new(
                "Name",
                Constraint::Percentage(40),
                |d: &TestData| Cell::from(d.name.clone()),
            ))
            .column(ColumnDef::new(
                "Value",
                Constraint::Percentage(30),
                |d: &TestData| Cell::from(d.value.to_string()),
            ))
            .column(ColumnDef::new(
                "Status",
                Constraint::Percentage(30),
                |d: &TestData| Cell::from(d.status.clone()),
            ))
            .rows(vec![
                TestData {
                    name: "Item 1".to_string(),
                    value: 100,
                    status: "Active".to_string(),
                },
                TestData {
                    name: "Item 2".to_string(),
                    value: 200,
                    status: "Inactive".to_string(),
                },
            ]);

        assert_eq!(table.title, Some("Test Table".to_string()));
        assert_eq!(table.columns.len(), 3);
        assert_eq!(table.rows.len(), 2);
        assert!(table.borders);
        assert!(table.highlight);
    }

    #[test]
    fn test_column_def_creation() {
        let col = ColumnDef::new("Test Column", Constraint::Length(20), |d: &TestData| {
            Cell::from(d.name.clone())
        });

        assert_eq!(col.header, "Test Column");
        assert_eq!(col.width, Constraint::Length(20));

        // Test render function
        let test_data = TestData {
            name: "Test".to_string(),
            value: 42,
            status: "OK".to_string(),
        };
        let cell = (col.render)(&test_data);
        // Cell doesn't expose content directly, but we can verify it doesn't panic
        drop(cell);
    }

    #[test]
    fn test_builder_pattern() {
        let table = TableData::<TestData>::new()
            .title("My Table")
            .borders(false)
            .highlight(false)
            .highlight_symbol("->")
            .row(TestData {
                name: "First".to_string(),
                value: 1,
                status: "OK".to_string(),
            })
            .row(TestData {
                name: "Second".to_string(),
                value: 2,
                status: "OK".to_string(),
            });

        assert_eq!(table.title, Some("My Table".to_string()));
        assert!(!table.borders);
        assert!(!table.highlight);
        assert_eq!(table.highlight_symbol, Some("->".to_string()));
        assert_eq!(table.rows.len(), 2);
    }

    #[test]
    fn test_default_values() {
        let table = TableData::<TestData>::default();

        assert_eq!(table.rows.len(), 0);
        assert_eq!(table.columns.len(), 0);
        assert_eq!(table.title, None);
        assert!(table.borders);
        assert!(table.highlight);
        assert_eq!(table.highlight_symbol, Some(">> ".to_string()));
    }

    #[test]
    fn test_render_table() {
        // Create a test backend
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        // Create test data
        let table = TableData::new()
            .title("Test Table")
            .column(ColumnDef::new(
                "Name",
                Constraint::Percentage(50),
                |d: &TestData| Cell::from(d.name.clone()),
            ))
            .column(ColumnDef::new(
                "Value",
                Constraint::Percentage(50),
                |d: &TestData| {
                    let color = if d.value > 100 {
                        Color::Green
                    } else {
                        Color::Red
                    };
                    Cell::from(d.value.to_string()).style(Style::default().fg(color))
                },
            ))
            .rows(vec![
                TestData {
                    name: "High".to_string(),
                    value: 200,
                    status: "Active".to_string(),
                },
                TestData {
                    name: "Low".to_string(),
                    value: 50,
                    status: "Inactive".to_string(),
                },
            ]);

        // Render the table
        terminal
            .draw(|f| {
                let area = f.area();
                render_table(f, area, &table);
            })
            .unwrap();

        // Verify rendering didn't panic and produced output
        let buffer = terminal.backend().buffer();
        assert!(buffer.area.width > 0);
        assert!(buffer.area.height > 0);
    }

    #[test]
    fn test_render_table_without_borders() {
        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        let table = TableData::new()
            .borders(false)
            .column(ColumnDef::new(
                "Test",
                Constraint::Percentage(100),
                |d: &TestData| Cell::from(d.name.clone()),
            ))
            .row(TestData {
                name: "Item".to_string(),
                value: 0,
                status: "OK".to_string(),
            });

        terminal
            .draw(|f| {
                let area = f.area();
                render_table(f, area, &table);
            })
            .unwrap();

        // Should render without panicking
        let buffer = terminal.backend().buffer();
        assert!(buffer.area.width > 0);
    }

    #[test]
    fn test_multiple_column_widths() {
        let table = TableData::<TestData>::new()
            .column(ColumnDef::new(
                "Fixed",
                Constraint::Length(10),
                |d: &TestData| Cell::from(d.name.clone()),
            ))
            .column(ColumnDef::new(
                "Percentage",
                Constraint::Percentage(50),
                |d: &TestData| Cell::from(d.value.to_string()),
            ))
            .column(ColumnDef::new(
                "Min",
                Constraint::Min(5),
                |d: &TestData| Cell::from(d.status.clone()),
            ))
            .column(ColumnDef::new(
                "Max",
                Constraint::Max(20),
                |_d: &TestData| Cell::from("X"),
            ));

        assert_eq!(table.columns.len(), 4);
        assert_eq!(table.columns[0].width, Constraint::Length(10));
        assert_eq!(table.columns[1].width, Constraint::Percentage(50));
        assert_eq!(table.columns[2].width, Constraint::Min(5));
        assert_eq!(table.columns[3].width, Constraint::Max(20));
    }
}
