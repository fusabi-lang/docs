//! Integration tests for Fusabi TUI bindings
//!
//! This test suite validates that the Fusabi bindings register correctly
//! and that widget specifications can be constructed.
//!
//! Note: Due to Fusabi Engine's internal API, we cannot directly test
//! calling the registered functions. Instead, these tests verify:
//! 1. Registration completes without errors
//! 2. Widget specification structures work correctly
//! 3. Serialization/deserialization works

use fusabi::Engine;
use fusabi_tui::bindings::FusabiTuiModule;
use fusabi_tui::bindings::specs::{TableSpec, ColumnSpec, WidthSpec, GraphSpec, NodeSpec, EdgeSpec, GraphType};

#[test]
fn test_module_registration() {
    // Test that module registration succeeds
    let mut engine = Engine::new();
    let module = FusabiTuiModule::new();

    let result = module.register(&mut engine);
    assert!(result.is_ok(), "Module registration should succeed");
}

#[test]
fn test_module_clone() {
    // Verify module is Clone
    let module1 = FusabiTuiModule::new();
    let module2 = module1.clone();

    // Both should be able to register
    let mut engine1 = Engine::new();
    let mut engine2 = Engine::new();

    assert!(module1.register(&mut engine1).is_ok());
    assert!(module2.register(&mut engine2).is_ok());
}

#[test]
fn test_module_default() {
    // Verify Default trait
    let module = FusabiTuiModule::default();
    let mut engine = Engine::new();

    assert!(module.register(&mut engine).is_ok());
}

// ==============================================================================
// Widget Specification Tests
// ==============================================================================

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
fn test_table_spec_builder() {
    let spec = TableSpec {
        title: Some("Test Table".to_string()),
        columns: vec![
            ColumnSpec {
                header: "Name".to_string(),
                width: WidthSpec::Percentage(40),
            },
            ColumnSpec {
                header: "Value".to_string(),
                width: WidthSpec::Percentage(60),
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

    assert_eq!(spec.title, Some("Test Table".to_string()));
    assert_eq!(spec.columns.len(), 2);
    assert_eq!(spec.rows.len(), 2);
    assert_eq!(spec.rows[0], vec!["Alice".to_string(), "100".to_string()]);
}

#[test]
fn test_table_spec_json_serialization() {
    let spec = TableSpec {
        title: Some("GPU Metrics".to_string()),
        columns: vec![
            ColumnSpec {
                header: "GPU".to_string(),
                width: WidthSpec::Length(20),
            },
            ColumnSpec {
                header: "Memory".to_string(),
                width: WidthSpec::Percentage(50),
            },
        ],
        rows: vec![
            vec!["GPU 0".to_string(), "8 GB".to_string()],
            vec!["GPU 1".to_string(), "8 GB".to_string()],
        ],
        borders: true,
        highlight_symbol: Some(">> ".to_string()),
        highlight: true,
    };

    // Serialize to JSON
    let json = serde_json::to_string(&spec).unwrap();

    // Deserialize back
    let deserialized: TableSpec = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.title, spec.title);
    assert_eq!(deserialized.columns.len(), spec.columns.len());
    assert_eq!(deserialized.rows.len(), spec.rows.len());
    assert_eq!(deserialized.rows[0][0], "GPU 0");
}

#[test]
fn test_column_spec_creation() {
    let col = ColumnSpec {
        header: "Test Column".to_string(),
        width: WidthSpec::Min(10),
    };

    assert_eq!(col.header, "Test Column");
    match col.width {
        WidthSpec::Min(val) => assert_eq!(val, 10),
        _ => panic!("Expected Min width"),
    }
}

#[test]
fn test_width_spec_variants() {
    let percentage = WidthSpec::Percentage(50);
    let length = WidthSpec::Length(20);
    let min = WidthSpec::Min(10);
    let max = WidthSpec::Max(100);

    // Verify each variant serializes correctly
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
        edges: vec![
            EdgeSpec {
                from: "n1".to_string(),
                to: "n2".to_string(),
                label: Some("connects".to_string()),
                color: None,
            },
        ],
    };

    assert_eq!(spec.title, "Test Graph");
    assert_eq!(spec.nodes.len(), 2);
    assert_eq!(spec.edges.len(), 1);
    assert_eq!(spec.nodes[0].id, "n1");
    assert_eq!(spec.edges[0].from, "n1");
    assert_eq!(spec.edges[0].to, "n2");
}

#[test]
fn test_graph_spec_json_serialization() {
    let spec = GraphSpec {
        title: "Pipeline".to_string(),
        graph_type: GraphType::Directed,
        nodes: vec![
            NodeSpec {
                id: "source".to_string(),
                label: "Data Source".to_string(),
                x: 0.1,
                y: 0.5,
                color: Some("#00FF00".to_string()),
            },
            NodeSpec {
                id: "transform".to_string(),
                label: "Transform".to_string(),
                x: 0.5,
                y: 0.5,
                color: Some("#0000FF".to_string()),
            },
            NodeSpec {
                id: "sink".to_string(),
                label: "Sink".to_string(),
                x: 0.9,
                y: 0.5,
                color: Some("#FF0000".to_string()),
            },
        ],
        edges: vec![
            EdgeSpec {
                from: "source".to_string(),
                to: "transform".to_string(),
                label: None,
                color: None,
            },
            EdgeSpec {
                from: "transform".to_string(),
                to: "sink".to_string(),
                label: None,
                color: None,
            },
        ],
    };

    // Serialize to JSON
    let json = serde_json::to_string(&spec).unwrap();

    // Deserialize back
    let deserialized: GraphSpec = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.title, "Pipeline");
    assert_eq!(deserialized.nodes.len(), 3);
    assert_eq!(deserialized.edges.len(), 2);
    assert_eq!(deserialized.nodes[0].label, "Data Source");
}

#[test]
fn test_node_spec_coordinates() {
    let node = NodeSpec {
        id: "test".to_string(),
        label: "Test Node".to_string(),
        x: 0.33,
        y: 0.67,
        color: None,
    };

    assert_eq!(node.x, 0.33);
    assert_eq!(node.y, 0.67);
    assert!(node.x >= 0.0 && node.x <= 1.0);
    assert!(node.y >= 0.0 && node.y <= 1.0);
}

#[test]
fn test_edge_spec_with_label() {
    let edge = EdgeSpec {
        from: "a".to_string(),
        to: "b".to_string(),
        label: Some("test edge".to_string()),
        color: Some("#FFFFFF".to_string()),
    };

    assert_eq!(edge.from, "a");
    assert_eq!(edge.to, "b");
    assert_eq!(edge.label, Some("test edge".to_string()));
    assert_eq!(edge.color, Some("#FFFFFF".to_string()));
}

#[test]
fn test_graph_type_serialization() {
    let directed = GraphType::Directed;
    let undirected = GraphType::Undirected;

    let json_d = serde_json::to_string(&directed).unwrap();
    let json_u = serde_json::to_string(&undirected).unwrap();

    assert_eq!(json_d, "\"Directed\"");
    assert_eq!(json_u, "\"Undirected\"");
}

#[test]
fn test_practical_table_spec() {
    // Simulate creating a table spec that would be built from F# script
    let mut spec = TableSpec::default();

    // Set title
    spec.title = Some("GPU Performance".to_string());

    // Add columns
    spec.columns.push(ColumnSpec {
        header: "GPU ID".to_string(),
        width: WidthSpec::Length(10),
    });

    spec.columns.push(ColumnSpec {
        header: "Utilization".to_string(),
        width: WidthSpec::Percentage(30),
    });

    spec.columns.push(ColumnSpec {
        header: "Memory".to_string(),
        width: WidthSpec::Percentage(30),
    });

    spec.columns.push(ColumnSpec {
        header: "Temp".to_string(),
        width: WidthSpec::Percentage(30),
    });

    // Add rows
    spec.rows.push(vec!["GPU 0".to_string(), "95%".to_string(), "8 GB".to_string(), "72°C".to_string()]);
    spec.rows.push(vec!["GPU 1".to_string(), "87%".to_string(), "8 GB".to_string(), "68°C".to_string()]);
    spec.rows.push(vec!["GPU 2".to_string(), "45%".to_string(), "4 GB".to_string(), "58°C".to_string()]);

    assert_eq!(spec.columns.len(), 4);
    assert_eq!(spec.rows.len(), 3);
    assert_eq!(spec.rows[0].len(), 4);

    // Verify it serializes
    let json = serde_json::to_string_pretty(&spec).unwrap();
    assert!(json.contains("GPU Performance"));
}
