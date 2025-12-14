// Fusabi TUI Demo Script
//
// This F# script demonstrates the Fusabi TUI bindings for formatting
// and widget specification building.
//
// Usage:
//   From Rust:
//     let mut engine = Engine::new();
//     let module = FusabiTuiModule::new();
//     module.register(&mut engine).unwrap();
//     fusabi::run_file("examples/fusabi_demo.fsx").unwrap();

// =============================================================================
// Formatting Functions Demo
// =============================================================================

printfn "=== Fusabi TUI Formatting Demo ==="
printfn ""

// Format large numbers with K/M/B suffixes
printfn "--- Number Formatting ---"
let smallNum = 999L
let thousands = 1500L
let millions = 2500000L
let billions = 3500000000L

printfn "Small: %s" (tui_format_number smallNum)        // "999"
printfn "Thousands: %s" (tui_format_number thousands)   // "1.50K"
printfn "Millions: %s" (tui_format_number millions)     // "2.50M"
printfn "Billions: %s" (tui_format_number billions)     // "3.50B"
printfn ""

// Format byte sizes with KB/MB/GB units
printfn "--- Byte Formatting ---"
let bytes = 512L
let kilobytes = 2048L
let megabytes = 5242880L
let gigabytes = 3221225472L

printfn "Bytes: %s" (tui_format_bytes bytes)            // "512 B"
printfn "Kilobytes: %s" (tui_format_bytes kilobytes)    // "2.00 KB"
printfn "Megabytes: %s" (tui_format_bytes megabytes)    // "5.00 MB"
printfn "Gigabytes: %s" (tui_format_bytes gigabytes)    // "3.00 GB"
printfn ""

// Format latency in microseconds
printfn "--- Latency Formatting ---"
let micros = 500L
let millis = 1500L
let seconds = 2500000L

printfn "Microseconds: %s" (tui_format_latency micros)  // "500μs"
printfn "Milliseconds: %s" (tui_format_latency millis)  // "1.50ms"
printfn "Seconds: %s" (tui_format_latency seconds)      // "2.50s"
printfn ""

// Format duration in seconds
printfn "--- Duration Formatting ---"
let secs = 42L
let mins = 312L
let hours = 9045L

printfn "Seconds: %s" (tui_format_duration secs)        // "42s"
printfn "Minutes: %s" (tui_format_duration mins)        // "5m 12s"
printfn "Hours: %s" (tui_format_duration hours)         // "2h 30m 45s"
printfn ""

// =============================================================================
// Table Specification Demo
// =============================================================================

printfn "=== Table Specification Demo ==="
printfn ""

// Create a table specification for GPU metrics
let gpuTable = tui_table_spec_new ()

// Set title
let gpuTable = tui_table_set_title gpuTable "GPU Metrics"

// Add columns (header, width as percentage)
let gpuTable = tui_table_add_column gpuTable "GPU" 20L
let gpuTable = tui_table_add_column gpuTable "Memory" 30L
let gpuTable = tui_table_add_column gpuTable "Utilization" 25L
let gpuTable = tui_table_add_column gpuTable "Temperature" 25L

// Add data rows (each row is an array of strings)
let row1 = ["GPU 0"; tui_format_bytes 8589934592L; "95%"; "72°C"]
let gpuTable = tui_table_add_row gpuTable row1

let row2 = ["GPU 1"; tui_format_bytes 8589934592L; "87%"; "68°C"]
let gpuTable = tui_table_add_row gpuTable row2

let row3 = ["GPU 2"; tui_format_bytes 4294967296L; "45%"; "58°C"]
let gpuTable = tui_table_add_row gpuTable row3

printfn "Created table specification with 4 columns and 3 rows"
printfn "Title: GPU Metrics"
printfn "Columns: GPU, Memory, Utilization, Temperature"
printfn ""

// =============================================================================
// Performance Metrics Table Demo
// =============================================================================

printfn "=== Performance Metrics Table Demo ==="
printfn ""

let perfTable = tui_table_spec_new ()
let perfTable = tui_table_set_title perfTable "Performance Metrics"

// Add columns
let perfTable = tui_table_add_column perfTable "Operation" 40L
let perfTable = tui_table_add_column perfTable "Count" 20L
let perfTable = tui_table_add_column perfTable "Avg Latency" 20L
let perfTable = tui_table_add_column perfTable "Total Time" 20L

// Sample data with formatting
let ops = [
    ("Matrix Multiply", 1500000L, 50L, 75L)
    ("Tensor Conv2D", 850000L, 120L, 102L)
    ("Data Transfer", 3200000L, 15L, 48L)
    ("Memory Alloc", 5500000L, 8L, 44L)
]

// Add formatted rows
let addPerfRow table (name, count, latency, duration) =
    let row = [
        name
        tui_format_number count
        tui_format_latency latency
        tui_format_duration duration
    ]
    tui_table_add_row table row

let perfTable = List.fold addPerfRow perfTable ops

printfn "Created performance metrics table with %d rows" (List.length ops)
printfn "All values are formatted using TUI formatting functions"
printfn ""

// =============================================================================
// Practical Example: System Resource Monitor
// =============================================================================

printfn "=== System Resource Monitor Demo ==="
printfn ""

let sysTable = tui_table_spec_new ()
let sysTable = tui_table_set_title sysTable "System Resources"

let sysTable = tui_table_add_column sysTable "Resource" 30L
let sysTable = tui_table_add_column sysTable "Used" 25L
let sysTable = tui_table_add_column sysTable "Total" 25L
let sysTable = tui_table_add_column sysTable "Uptime" 20L

// Mock system data
let sysRow1 = [
    "CPU Cores"
    tui_format_number 16L
    tui_format_number 64L
    tui_format_duration 259200L  // 3 days
]
let sysTable = tui_table_add_row sysTable sysRow1

let sysRow2 = [
    "RAM"
    tui_format_bytes 137438953472L  // 128 GB
    tui_format_bytes 274877906944L  // 256 GB
    tui_format_duration 259200L
]
let sysTable = tui_table_add_row sysTable sysRow2

let sysRow3 = [
    "GPU Memory"
    tui_format_bytes 68719476736L   // 64 GB
    tui_format_bytes 137438953472L  // 128 GB
    tui_format_duration 259200L
]
let sysTable = tui_table_add_row sysTable sysRow3

let sysRow4 = [
    "Disk I/O"
    tui_format_bytes 1099511627776L  // 1 TB
    tui_format_bytes 5497558138880L  // 5 TB
    tui_format_duration 259200L
]
let sysTable = tui_table_add_row sysTable sysRow4

printfn "Created system resource monitor table"
printfn "Demonstrates mixed formatting: numbers, bytes, and durations"
printfn ""

// =============================================================================
// Summary
// =============================================================================

printfn "=== Demo Complete ==="
printfn ""
printfn "This script demonstrated:"
printfn "  - Number formatting (K/M/B suffixes)"
printfn "  - Byte size formatting (KB/MB/GB)"
printfn "  - Latency formatting (μs/ms/s)"
printfn "  - Duration formatting (s/m/h)"
printfn "  - Table specification building"
printfn "  - Column and row management"
printfn "  - Practical use cases"
printfn ""
printfn "Note: This creates widget *specifications* only."
printfn "The Rust application is responsible for rendering these specs."
printfn "This is a workaround for the Fusabi Send+Sync limitation."
printfn ""
printfn "See docs/FUSABI_SEND_SYNC_ISSUE_DRAFT.md for details."
