//! Demonstrates the formatting utilities in fusabi-tui
//!
//! Run with: cargo run --example formatting_demo

use fusabi_tui::{format_number, format_bytes, format_latency, format_duration};
use std::time::Duration;

fn main() {
    println!("=== Number Formatting ===");
    println!("999 -> {}", format_number(999));
    println!("1,500 -> {}", format_number(1_500));
    println!("2,500,000 -> {}", format_number(2_500_000));
    println!("3,500,000,000 -> {}", format_number(3_500_000_000));
    println!();

    println!("=== Byte Formatting ===");
    println!("512 bytes -> {}", format_bytes(512));
    println!("2,048 bytes -> {}", format_bytes(2_048));
    println!("5,242,880 bytes -> {}", format_bytes(5_242_880));
    println!("3,221,225,472 bytes -> {}", format_bytes(3_221_225_472));
    println!();

    println!("=== Latency Formatting ===");
    println!("500 μs -> {}", format_latency(500));
    println!("1,500 μs -> {}", format_latency(1_500));
    println!("2,500,000 μs -> {}", format_latency(2_500_000));
    println!();

    println!("=== Duration Formatting ===");
    println!("42 seconds -> {}", format_duration(Duration::from_secs(42)));
    println!("312 seconds -> {}", format_duration(Duration::from_secs(312)));
    println!("9,045 seconds -> {}", format_duration(Duration::from_secs(9_045)));
    println!();

    println!("=== Real-World Example ===");
    let events_processed = 15_420_000_u64;
    let memory_used = 2_147_483_648_u64; // 2 GB
    let avg_latency = 2_500_u64; // 2.5 ms
    let uptime = Duration::from_secs(86_400); // 1 day

    println!("System Status:");
    println!("  Events Processed: {}", format_number(events_processed));
    println!("  Memory Used: {}", format_bytes(memory_used));
    println!("  Avg Latency: {}", format_latency(avg_latency));
    println!("  Uptime: {}", format_duration(uptime));
}
