//! Large number formatting with thousand separators.
//!
//! This module provides utilities for formatting large numbers into human-readable
//! strings with appropriate suffixes (K, M, B).

/// Format large numbers with thousand separators.
///
/// Converts large numbers into compact, human-readable strings using
/// standard SI prefixes:
/// - B (Billions): >= 1,000,000,000
/// - M (Millions): >= 1,000,000
/// - K (Thousands): >= 1,000
/// - Raw number: < 1,000
///
/// # Arguments
///
/// * `n` - The number to format
///
/// # Returns
///
/// A formatted string with 2 decimal places for large numbers,
/// or the raw number as a string for values < 1,000.
///
/// # Examples
///
/// ```rust
/// use fusabi_tui::formatting::format_number;
///
/// assert_eq!(format_number(999), "999");
/// assert_eq!(format_number(1_500), "1.50K");
/// assert_eq!(format_number(2_500_000), "2.50M");
/// assert_eq!(format_number(3_500_000_000), "3.50B");
/// ```
pub fn format_number(n: u64) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number_small() {
        // Numbers below 1,000 should be displayed as-is
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(99), "99");
        assert_eq!(format_number(999), "999");
    }

    #[test]
    fn test_format_number_thousands() {
        // Numbers in thousands should use K suffix
        assert_eq!(format_number(1_000), "1.00K");
        assert_eq!(format_number(1_500), "1.50K");
        assert_eq!(format_number(10_000), "10.00K");
        assert_eq!(format_number(999_999), "1000.00K");
    }

    #[test]
    fn test_format_number_millions() {
        // Numbers in millions should use M suffix
        assert_eq!(format_number(1_000_000), "1.00M");
        assert_eq!(format_number(2_500_000), "2.50M");
        assert_eq!(format_number(100_000_000), "100.00M");
        assert_eq!(format_number(999_999_999), "1000.00M");
    }

    #[test]
    fn test_format_number_billions() {
        // Numbers in billions should use B suffix
        assert_eq!(format_number(1_000_000_000), "1.00B");
        assert_eq!(format_number(3_500_000_000), "3.50B");
        assert_eq!(format_number(10_000_000_000), "10.00B");
        assert_eq!(format_number(u64::MAX), "18446744073.71B");
    }

    #[test]
    fn test_format_number_boundary_cases() {
        // Test exact boundary values
        assert_eq!(format_number(999), "999");
        assert_eq!(format_number(1_000), "1.00K");
        assert_eq!(format_number(999_999), "1000.00K");
        assert_eq!(format_number(1_000_000), "1.00M");
        assert_eq!(format_number(999_999_999), "1000.00M");
        assert_eq!(format_number(1_000_000_000), "1.00B");
    }
}
