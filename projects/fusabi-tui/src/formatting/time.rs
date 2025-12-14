//! Time and duration formatting utilities.
//!
//! This module provides utilities for formatting durations and latencies
//! into human-readable strings with appropriate time units.

use std::time::Duration;

/// Format duration into human-readable string with hours, minutes, and seconds.
///
/// Converts a `Duration` into a compact, human-readable string showing
/// hours, minutes, and seconds. Only the relevant units are shown:
/// - Hours, minutes, seconds: >= 1 hour
/// - Minutes, seconds: >= 1 minute
/// - Seconds only: < 1 minute
///
/// # Arguments
///
/// * `duration` - The duration to format
///
/// # Returns
///
/// A formatted string like "2h 30m 45s", "5m 12s", or "42s".
///
/// # Examples
///
/// ```rust
/// use std::time::Duration;
/// use fusabi_tui::formatting::format_duration;
///
/// assert_eq!(format_duration(Duration::from_secs(42)), "42s");
/// assert_eq!(format_duration(Duration::from_secs(312)), "5m 12s");
/// assert_eq!(format_duration(Duration::from_secs(9045)), "2h 30m 45s");
/// ```
pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

/// Format latency from microseconds to human-readable string.
///
/// Converts latency in microseconds into a compact, human-readable string
/// with appropriate time units:
/// - s (seconds): >= 1,000,000 μs
/// - ms (milliseconds): >= 1,000 μs
/// - μs (microseconds): < 1,000 μs
///
/// # Arguments
///
/// * `us` - The latency in microseconds to format
///
/// # Returns
///
/// A formatted string with 2 decimal places for s/ms,
/// or the raw microsecond count for values < 1,000.
///
/// # Examples
///
/// ```rust
/// use fusabi_tui::formatting::format_latency;
///
/// assert_eq!(format_latency(500), "500μs");
/// assert_eq!(format_latency(1_500), "1.50ms");
/// assert_eq!(format_latency(2_500_000), "2.50s");
/// ```
pub fn format_latency(us: u64) -> String {
    if us >= 1_000_000 {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    } else if us >= 1_000 {
        format!("{:.2}ms", us as f64 / 1_000.0)
    } else {
        format!("{}μs", us)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_seconds_only() {
        // Durations less than a minute should show seconds only
        assert_eq!(format_duration(Duration::from_secs(0)), "0s");
        assert_eq!(format_duration(Duration::from_secs(1)), "1s");
        assert_eq!(format_duration(Duration::from_secs(42)), "42s");
        assert_eq!(format_duration(Duration::from_secs(59)), "59s");
    }

    #[test]
    fn test_format_duration_minutes() {
        // Durations >= 1 minute should show minutes and seconds
        assert_eq!(format_duration(Duration::from_secs(60)), "1m 0s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(312)), "5m 12s");
        assert_eq!(format_duration(Duration::from_secs(3599)), "59m 59s");
    }

    #[test]
    fn test_format_duration_hours() {
        // Durations >= 1 hour should show hours, minutes, and seconds
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h 0m 0s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
        assert_eq!(format_duration(Duration::from_secs(9045)), "2h 30m 45s");
        assert_eq!(format_duration(Duration::from_secs(86399)), "23h 59m 59s");
    }

    #[test]
    fn test_format_duration_days() {
        // Test durations spanning multiple days
        let one_day = Duration::from_secs(86400);
        assert_eq!(format_duration(one_day), "24h 0m 0s");

        let two_days_plus = Duration::from_secs(172800 + 3661);
        assert_eq!(format_duration(two_days_plus), "49h 1m 1s");
    }

    #[test]
    fn test_format_duration_boundary_cases() {
        // Test exact boundary values
        assert_eq!(format_duration(Duration::from_secs(59)), "59s");
        assert_eq!(format_duration(Duration::from_secs(60)), "1m 0s");
        assert_eq!(format_duration(Duration::from_secs(3599)), "59m 59s");
        assert_eq!(format_duration(Duration::from_secs(3600)), "1h 0m 0s");
    }

    #[test]
    fn test_format_latency_microseconds() {
        // Latencies below 1000 μs should be displayed as-is
        assert_eq!(format_latency(0), "0μs");
        assert_eq!(format_latency(1), "1μs");
        assert_eq!(format_latency(500), "500μs");
        assert_eq!(format_latency(999), "999μs");
    }

    #[test]
    fn test_format_latency_milliseconds() {
        // Latencies in milliseconds should use ms suffix
        assert_eq!(format_latency(1_000), "1.00ms");
        assert_eq!(format_latency(1_500), "1.50ms");
        assert_eq!(format_latency(10_000), "10.00ms");
        assert_eq!(format_latency(100_000), "100.00ms");
        assert_eq!(format_latency(999_999), "1000.00ms");
    }

    #[test]
    fn test_format_latency_seconds() {
        // Latencies in seconds should use s suffix
        assert_eq!(format_latency(1_000_000), "1.00s");
        assert_eq!(format_latency(2_500_000), "2.50s");
        assert_eq!(format_latency(10_000_000), "10.00s");
        assert_eq!(format_latency(100_000_000), "100.00s");
    }

    #[test]
    fn test_format_latency_boundary_cases() {
        // Test exact boundary values
        assert_eq!(format_latency(999), "999μs");
        assert_eq!(format_latency(1_000), "1.00ms");
        assert_eq!(format_latency(999_999), "1000.00ms");
        assert_eq!(format_latency(1_000_000), "1.00s");
    }

    #[test]
    fn test_format_latency_common_values() {
        // Test typical latency values seen in practice
        assert_eq!(format_latency(10), "10μs");      // Fast operation
        assert_eq!(format_latency(100), "100μs");    // Memory access
        assert_eq!(format_latency(5_000), "5.00ms"); // Network request
        assert_eq!(format_latency(50_000), "50.00ms"); // Slow query
        assert_eq!(format_latency(1_000_000), "1.00s"); // Timeout
    }

    #[test]
    fn test_format_latency_large_values() {
        // Test very large latency values
        assert_eq!(format_latency(u64::MAX), "18446744073709.55s");
    }
}
