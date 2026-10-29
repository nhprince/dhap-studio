//! Structured error hierarchy for the Dhap studio and domain engine.

use std::path::PathBuf;
use thiserror::Error;

/// Core error enum representing failure modes across all Dhap crates.
#[derive(Debug, Error)]
pub enum DhapError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization / JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP package error: {0}")]
    Zip(String),

    #[error("Path traversal security violation: attempted access to '{0}'")]
    PathTraversal(PathBuf),

    #[error("Project validation error: {0}")]
    Validation(String),

    #[error("Unsupported project schema version: found {found}, supported is {supported}")]
    UnsupportedSchema { found: u32, supported: u32 },

    #[error("Media processing error: {0}")]
    Media(String),

    #[error("FFmpeg execution failed with code {code:?}: {message}")]
    FfmpegExecution { code: Option<i32>, message: String },

    #[error("Image processing / rasterization error: {0}")]
    Image(String),

    #[error("Export generation error: {0}")]
    Export(String),

    #[error("Entity not found: {entity_type} with ID '{id}'")]
    NotFound { entity_type: String, id: String },

    #[error("Operation cancelled by user or timeout")]
    Cancelled,
}

/// Convenience result alias for Dhap operations.
pub type Result<T> = std::result::Result<T, DhapError>;
