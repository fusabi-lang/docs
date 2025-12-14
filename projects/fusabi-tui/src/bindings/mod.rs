//! Bindings module - Fusabi VM integration
//!
//! This module provides the bridge between Fusabi scripts and the TUI library,
//! exposing native functions that can be called from F# scripts.
//!
//! ## Native Functions
//!
//! Functions exposed to Fusabi scripts for:
//! - Formatting utilities (numbers, bytes, time)
//! - Widget specification builders (tables, canvas)
//!
//! ## Limitations
//!
//! Due to Fusabi's current `Rc<RefCell<T>>` design (not Send+Sync), full widget
//! lifecycle management is not yet supported. Instead, this module provides:
//!
//! 1. **Formatting functions** - Pure functions for data formatting
//! 2. **Widget specifications** - JSON-serializable structures that F# can build
//!
//! The Rust application is responsible for interpreting widget specifications
//! and rendering them. This is a temporary workaround until Fusabi supports
//! Send+Sync Engine instances.
//!
//! See `/home/beengud/raibid-labs/hibana/docs/FUSABI_SEND_SYNC_ISSUE_DRAFT.md`
//! for details on the upstream limitation.
//!
//! ## Example
//!
//! ```rust
//! use fusabi_tui::bindings::FusabiTuiModule;
//! use fusabi::Engine;
//!
//! let mut engine = Engine::new();
//! let module = FusabiTuiModule::new();
//! module.register(&mut engine).unwrap();
//!
//! // Now F# scripts can call tui_format_number, tui_format_bytes, etc.
//! ```

use anyhow::Result;
use fusabi::Engine;
use fusabi_vm::VmError;
use fusabi::Value;
use std::rc::Rc;
use std::cell::RefCell;

use crate::formatting::{format_number, format_bytes, format_latency};
use std::time::Duration;

/// Widget specification types that can be serialized to/from JSON
pub mod specs;

/// Main module for registering TUI functions with the Fusabi VM
#[derive(Clone)]
pub struct FusabiTuiModule;

impl FusabiTuiModule {
    /// Create a new Fusabi TUI module
    pub fn new() -> Self {
        Self
    }

    /// Register native functions with the Fusabi VM
    ///
    /// Registers the following functions:
    /// - `tui_format_number(i64) -> string` - Format large numbers (K/M/B)
    /// - `tui_format_bytes(i64) -> string` - Format byte sizes (KB/MB/GB)
    /// - `tui_format_latency(i64) -> string` - Format latency in microseconds
    /// - `tui_format_duration(i64) -> string` - Format duration in seconds
    ///
    /// ## Example F# Usage
    ///
    /// ```fsharp
    /// let count = 1500000L
    /// let formatted = tui_format_number count  // Returns "1.50M"
    ///
    /// let size = 2048L
    /// let formatted = tui_format_bytes size  // Returns "2.00 KB"
    /// ```
    pub fn register(&self, engine: &mut Engine) -> Result<()> {
        // Register formatting functions as global Fusabi functions
        // These are pure functions with no side effects, so they work
        // despite the Send+Sync limitation.

        // tui_format_number(n: int) -> string
        engine.register_raw("tui_format_number", |_vm, args| {
            if args.len() != 1 {
                return Err(VmError::Runtime(
                    "tui_format_number expects 1 argument: number (int)".into(),
                ));
            }
            let num = args[0]
                .as_int()
                .ok_or(VmError::TypeMismatch {
                    expected: "int",
                    got: args[0].type_name(),
                })? as u64;

            let formatted = format_number(num);
            Ok(Value::Str(formatted))
        });

        // tui_format_bytes(bytes: int) -> string
        engine.register_raw("tui_format_bytes", |_vm, args| {
            if args.len() != 1 {
                return Err(VmError::Runtime(
                    "tui_format_bytes expects 1 argument: bytes (int)".into(),
                ));
            }
            let bytes = args[0]
                .as_int()
                .ok_or(VmError::TypeMismatch {
                    expected: "int",
                    got: args[0].type_name(),
                })? as u64;

            let formatted = format_bytes(bytes);
            Ok(Value::Str(formatted))
        });

        // tui_format_latency(microseconds: int) -> string
        engine.register_raw("tui_format_latency", |_vm, args| {
            if args.len() != 1 {
                return Err(VmError::Runtime(
                    "tui_format_latency expects 1 argument: microseconds (int)".into(),
                ));
            }
            let us = args[0]
                .as_int()
                .ok_or(VmError::TypeMismatch {
                    expected: "int",
                    got: args[0].type_name(),
                })? as u64;

            let formatted = format_latency(us);
            Ok(Value::Str(formatted))
        });

        // tui_format_duration(seconds: int) -> string
        engine.register_raw("tui_format_duration", |_vm, args| {
            if args.len() != 1 {
                return Err(VmError::Runtime(
                    "tui_format_duration expects 1 argument: seconds (int)".into(),
                ));
            }
            let secs = args[0]
                .as_int()
                .ok_or(VmError::TypeMismatch {
                    expected: "int",
                    got: args[0].type_name(),
                })? as u64;

            let formatted = crate::formatting::format_duration(Duration::from_secs(secs));
            Ok(Value::Str(formatted))
        });

        // tui_table_spec_new() -> record
        // Returns an empty table specification that can be built up
        engine.register_raw("tui_table_spec_new", |_vm, args| {
            if !args.is_empty() {
                return Err(VmError::Runtime(
                    "tui_table_spec_new expects no arguments".into(),
                ));
            }

            let mut fields = std::collections::HashMap::new();
            fields.insert("columns".to_string(), Value::Array(Rc::new(RefCell::new(vec![]))));
            fields.insert("rows".to_string(), Value::Array(Rc::new(RefCell::new(vec![]))));
            fields.insert("title".to_string(), Value::Unit);
            fields.insert("borders".to_string(), Value::Bool(true));

            Ok(Value::Record(Rc::new(RefCell::new(fields))))
        });

        // tui_table_add_column(spec: record, header: string, width: int) -> record
        // Add a column definition to a table spec
        engine.register_raw("tui_table_add_column", |_vm, args| {
            if args.len() != 3 {
                return Err(VmError::Runtime(
                    "tui_table_add_column expects 3 arguments: spec (record), header (string), width (int)".into(),
                ));
            }

            let spec = args[0].as_record().ok_or(VmError::TypeMismatch {
                expected: "record",
                got: args[0].type_name(),
            })?;

            let header = args[1].as_str().ok_or(VmError::TypeMismatch {
                expected: "string",
                got: args[1].type_name(),
            })?;

            let width = args[2].as_int().ok_or(VmError::TypeMismatch {
                expected: "int",
                got: args[2].type_name(),
            })?;

            // Create column definition
            let mut col_fields = std::collections::HashMap::new();
            col_fields.insert("header".to_string(), Value::Str(header.to_string()));
            col_fields.insert("width".to_string(), Value::Int(width));
            let col = Value::Record(Rc::new(RefCell::new(col_fields)));

            // Add to columns array
            let mut spec_mut = spec.borrow_mut();
            if let Some(columns_val) = spec_mut.get("columns") {
                if let Some(columns_arr) = columns_val.as_array() {
                    columns_arr.borrow_mut().push(col);
                }
            }

            drop(spec_mut);
            Ok(Value::Record(spec.clone()))
        });

        // tui_table_add_row(spec: record, cells: array<string>) -> record
        // Add a data row to the table spec
        engine.register_raw("tui_table_add_row", |_vm, args| {
            if args.len() != 2 {
                return Err(VmError::Runtime(
                    "tui_table_add_row expects 2 arguments: spec (record), cells (array)".into(),
                ));
            }

            let spec = args[0].as_record().ok_or(VmError::TypeMismatch {
                expected: "record",
                got: args[0].type_name(),
            })?;

            let cells = args[1].as_array().ok_or(VmError::TypeMismatch {
                expected: "array",
                got: args[1].type_name(),
            })?;

            // Add to rows array
            let mut spec_mut = spec.borrow_mut();
            if let Some(rows_val) = spec_mut.get("rows") {
                if let Some(rows_arr) = rows_val.as_array() {
                    rows_arr.borrow_mut().push(Value::Array(cells.clone()));
                }
            }

            drop(spec_mut);
            Ok(Value::Record(spec.clone()))
        });

        // tui_table_set_title(spec: record, title: string) -> record
        engine.register_raw("tui_table_set_title", |_vm, args| {
            if args.len() != 2 {
                return Err(VmError::Runtime(
                    "tui_table_set_title expects 2 arguments: spec (record), title (string)".into(),
                ));
            }

            let spec = args[0].as_record().ok_or(VmError::TypeMismatch {
                expected: "record",
                got: args[0].type_name(),
            })?;

            let title = args[1].as_str().ok_or(VmError::TypeMismatch {
                expected: "string",
                got: args[1].type_name(),
            })?;

            spec.borrow_mut().insert("title".to_string(), Value::Str(title.to_string()));
            Ok(Value::Record(spec.clone()))
        });

        Ok(())
    }
}

impl Default for FusabiTuiModule {
    fn default() -> Self {
        Self::new()
    }
}
