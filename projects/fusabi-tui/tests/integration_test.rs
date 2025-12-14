//! Integration tests for fusabi-tui

use fusabi_tui::{Result, VERSION};

#[test]
fn test_version_constant() {
    assert_eq!(VERSION, "0.1.0");
}

#[test]
fn test_module_structure() -> Result<()> {
    // Test that core modules are accessible
    use fusabi_tui::formatting;

    // Test formatting functions
    let formatted = formatting::format_number(1500);
    assert_eq!(formatted, "1.50K");

    Ok(())
}

// Additional integration tests will be added as features are implemented
