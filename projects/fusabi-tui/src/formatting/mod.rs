//! Formatting utilities for human-readable display of numbers, bytes, and time.
//!
//! This module provides common formatting functions for terminal UI applications,
//! converting raw values into human-readable strings with appropriate units.
//!
//! ## Modules
//!
//! - `numbers`: Large number formatting with K/M/B suffixes
//! - `bytes`: Byte size formatting with B/KB/MB/GB units
//! - `time`: Duration and latency formatting with s/ms/μs units
//!
//! ## Example
//!
//! ```rust
//! use fusabi_tui::formatting::{format_number, format_bytes, format_latency};
//!
//! assert_eq!(format_number(1_500_000), "1.50M");
//! assert_eq!(format_bytes(2048), "2.00 KB");
//! assert_eq!(format_latency(1500), "1.50ms");
//! ```

pub mod numbers;
pub mod bytes;
pub mod time;

// Re-export commonly used functions
pub use numbers::format_number;
pub use bytes::format_bytes;
pub use time::{format_latency, format_duration};
