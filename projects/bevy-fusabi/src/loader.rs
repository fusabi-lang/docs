use crate::asset::FusabiScript;
use bevy::asset::{io::Reader, AssetLoader, LoadContext};
use fusabi_frontend::{Compiler, Lexer, Parser};
use fusabi_vm::{serialize_chunk, Chunk};
use thiserror::Error;

#[derive(Default)]
pub struct FusabiLoader;

/// Errors that can occur during script loading
#[derive(Error, Debug)]
pub enum FusabiLoaderError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Lexer error: {0}")]
    Lexer(String),
    #[error("Parser error: {0:?}")]
    Parser(String),
    #[error("Compiler error: {0:?}")]
    Compiler(String),
    #[error("Bytecode serialization error: {0}")]
    Bytecode(String),
    #[error("UTF-8 encoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl AssetLoader for FusabiLoader {
    type Asset = FusabiScript;
    type Settings = ();
    type Error = FusabiLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let ext = load_context
            .path()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let name = load_context
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let bytecode = if ext == "fzb" {
            // It's already bytecode
            bytes
        } else {
            // Compile from source (.fsx)
            let source = String::from_utf8(bytes)?;
            let chunk = compile_source(&source)?;
            // Serialize to bytecode
            serialize_chunk(&chunk).map_err(|e| FusabiLoaderError::Bytecode(e.to_string()))?
        };

        Ok(FusabiScript::new(name, bytecode))
    }

    fn extensions(&self) -> &[&str] {
        &["fsx", "fzb"]
    }
}

/// Helper to compile source code into a Chunk
fn compile_source(source: &str) -> Result<Chunk, FusabiLoaderError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| FusabiLoaderError::Lexer(format!("{:?}", e)))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse_program()
        .map_err(|e| FusabiLoaderError::Parser(format!("{:?}", e)))?;

    let chunk = Compiler::compile_program(&program)
        .map_err(|e| FusabiLoaderError::Compiler(format!("{:?}", e)))?;

    Ok(chunk)
}
