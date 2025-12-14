use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::{Deserialize, Serialize};

/// A loaded Fusabi script asset.
///
/// This asset contains the compiled bytecode ready for execution.
///
/// Note: We store the bytecode as `Vec<u8>` instead of `fusabi_vm::Chunk`
/// because `Chunk` contains `Rc` types which are !Send and !Sync, making
/// them incompatible with Bevy's asset system.
#[derive(Asset, TypePath, Debug, Clone)]
pub struct FusabiScript {
    /// The name of the script (usually derived from the filename)
    pub name: String,
    /// The serialized bytecode.
    /// To execute, deserialized this using `fusabi_vm::deserialize_chunk`.
    pub bytecode: Vec<u8>,
}

/// Metadata header for .fzb files
#[derive(Serialize, Deserialize, Debug)]
pub struct FusabiHeader {
    pub magic: u32,
    pub version: u32,
    pub timestamp: u64,
}

impl FusabiScript {
    pub fn new(name: String, bytecode: Vec<u8>) -> Self {
        Self { name, bytecode }
    }

    /// Helper to get a runnable Chunk from the asset.
    /// Note: This is expensive as it deserializes the bytecode.
    pub fn to_chunk(&self) -> Result<fusabi_vm::Chunk, String> {
        fusabi_vm::deserialize_chunk(&self.bytecode).map_err(|e| e.to_string())
    }
}
