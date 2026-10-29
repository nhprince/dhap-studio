//! # dhap-export
//!
//! Multi-format documentation exporters (Markdown bundles, standalone responsive HTML5 guides)
//! and preflight validation checks for Dhap.

pub mod html;
pub mod markdown;
pub mod preflight;

use dhap_core::Result;
use dhap_project::Project;
pub use html::export_html;
pub use markdown::export_markdown;
pub use preflight::{validate_project_preflight, PreflightReport};
use std::fs;
use std::path::Path;

/// Supported output documentation formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Markdown,
    Html,
    All,
}

/// High-level orchestration function to export documentation safely with preflight validation.
pub fn export_documentation(
    project: &Project,
    frames_dir: Option<&Path>,
    output_dir: &Path,
    format: ExportFormat,
) -> Result<PreflightReport> {
    // 1. Preflight validation
    let report = validate_project_preflight(project, frames_dir)?;

    // 2. Export to temporary directory before atomic swap
    let parent = output_dir.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)?;

    let temp_export_dir = parent.join(format!(
        ".{}.export_tmp",
        output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("export")
    ));

    if temp_export_dir.exists() {
        let _ = fs::remove_dir_all(&temp_export_dir);
    }
    fs::create_dir_all(&temp_export_dir)?;

    match format {
        ExportFormat::Markdown => {
            export_markdown(project, frames_dir, &temp_export_dir)?;
        }
        ExportFormat::Html => {
            export_html(project, frames_dir, &temp_export_dir)?;
        }
        ExportFormat::All => {
            export_markdown(project, frames_dir, &temp_export_dir)?;
            export_html(project, frames_dir, &temp_export_dir)?;
        }
    }

    // Replace target output directory cleanly
    if output_dir.exists() {
        let _ = fs::remove_dir_all(output_dir);
    }
    fs::rename(&temp_export_dir, output_dir)?;

    Ok(report)
}
