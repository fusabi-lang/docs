//! Byte size formatting with binary units (KiB, MiB, GiB).
//!
//! This module provides utilities for formatting byte sizes into human-readable
//! strings using binary units (powers of 1024).

/// Format memory size from bytes to human-readable string.
///
/// Converts byte counts into compact, human-readable strings using
/// binary units (powers of 1024):
/// - GB (Gigabytes): >= 1,073,741,824 bytes (1024³)
/// - MB (Megabytes): >= 1,048,576 bytes (1024²)
/// - KB (Kilobytes): >= 1,024 bytes
/// - B (Bytes): < 1,024 bytes
///
/// # Arguments
///
/// * `bytes` - The number of bytes to format
///
/// # Returns
///
/// A formatted string with 2 decimal places for KB/MB/GB,
/// or the raw byte count for values < 1024.
///
/// # Examples
///
/// ```rust
/// use fusabi_tui::formatting::format_bytes;
///
/// assert_eq!(format_bytes(512), "512 B");
/// assert_eq!(format_bytes(2048), "2.00 KB");
/// assert_eq!(format_bytes(5_242_880), "5.00 MB");
/// assert_eq!(format_bytes(3_221_225_472), "3.00 GB");
/// ```
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_small() {
        // Bytes below 1024 should be displayed as-is
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1), "1 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1023), "1023 B");
    }

    #[test]
    fn test_format_bytes_kilobytes() {
        // Bytes in KB range should use KB suffix
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(2048), "2.00 KB");
        assert_eq!(format_bytes(10_240), "10.00 KB");
        assert_eq!(format_bytes(524_288), "512.00 KB");
        assert_eq!(format_bytes(1_048_575), "1024.00 KB");
    }

    #[test]
    fn test_format_bytes_megabytes() {
        // Bytes in MB range should use MB suffix
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
        assert_eq!(format_bytes(5_242_880), "5.00 MB");
        assert_eq!(format_bytes(104_857_600), "100.00 MB");
        assert_eq!(format_bytes(536_870_912), "512.00 MB");
        assert_eq!(format_bytes(1_073_741_823), "1024.00 MB");
    }

    #[test]
    fn test_format_bytes_gigabytes() {
        // Bytes in GB range should use GB suffix
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
        assert_eq!(format_bytes(3_221_225_472), "3.00 GB");
        assert_eq!(format_bytes(10_737_418_240), "10.00 GB");
        assert_eq!(format_bytes(107_374_182_400), "100.00 GB");
    }

    #[test]
    fn test_format_bytes_boundary_cases() {
        // Test exact boundary values
        assert_eq!(format_bytes(1023), "1023 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1_048_575), "1024.00 KB");
        assert_eq!(format_bytes(1_048_576), "1.00 MB");
        assert_eq!(format_bytes(1_073_741_823), "1024.00 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.00 GB");
    }

    #[test]
    fn test_format_bytes_fractional() {
        // Test fractional values
        assert_eq!(format_bytes(1536), "1.50 KB"); // 1.5 KB
        assert_eq!(format_bytes(2_621_440), "2.50 MB"); // 2.5 MB
        assert_eq!(format_bytes(5_905_580_032), "5.50 GB"); // 5.5 GB
    }

    #[test]
    fn test_format_bytes_large_values() {
        // Test very large values (note: due to floating point precision,
        // u64::MAX / GB will round to 17179869184.00)
        let result = format_bytes(u64::MAX);
        assert!(result.starts_with("17179869184") || result.starts_with("17179869183"));
        assert!(result.ends_with(" GB"));
    }
}
