//! `.dhap` ZIP package serialization, extraction, and atomic persistence.

use crate::model::{Manifest, Project};
use dhap_core::{DhapError, Result};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

/// Serializes and writes a Project into a `.dhap` ZIP bundle at `output_bundle_path`.
///
/// Uses atomic write semantics: writes to a temporary sibling file first and renames.
pub fn save_bundle(
    project: &Project,
    frames_dir: Option<&Path>,
    output_bundle_path: &Path,
) -> Result<()> {
    let parent_dir = output_bundle_path
        .parent()
        .unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent_dir)?;

    let temp_path = parent_dir.join(format!(
        ".{}.tmp",
        output_bundle_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("project.dhap")
    ));

    {
        let file = File::create(&temp_path)?;
        let writer = BufWriter::new(file);
        let mut zip = ZipWriter::new(writer);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated)
            .unix_permissions(0o644);

        // 1. Write manifest.json
        let manifest = Manifest::from_project(project, "linked");
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
        zip.start_file("manifest.json", options)
            .map_err(|e| DhapError::Zip(e.to_string()))?;
        zip.write_all(&manifest_bytes)?;

        // 2. Write project.json
        let project_bytes = serde_json::to_vec_pretty(project)?;
        zip.start_file("project.json", options)
            .map_err(|e| DhapError::Zip(e.to_string()))?;
        zip.write_all(&project_bytes)?;

        // 3. Include frame files if provided
        if let Some(f_dir) = frames_dir {
            if f_dir.exists() && f_dir.is_dir() {
                for entry in WalkDir::new(f_dir).into_iter().filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_file() {
                        let rel = path
                            .strip_prefix(f_dir)
                            .map_err(|e| DhapError::Zip(e.to_string()))?;
                        let zip_entry_name =
                            format!("frames/{}", rel.to_string_lossy().replace('\\', "/"));
                        zip.start_file(&zip_entry_name, options)
                            .map_err(|e| DhapError::Zip(e.to_string()))?;
                        let mut f = File::open(path)?;
                        std::io::copy(&mut f, &mut zip)?;
                    }
                }
            }
        }

        zip.finish().map_err(|e| DhapError::Zip(e.to_string()))?;
    }

    // Atomic rename
    fs::rename(&temp_path, output_bundle_path)?;
    Ok(())
}

/// Extracts a `.dhap` ZIP bundle into a destination directory and deserializes `Project`.
///
/// **Security Guarantee:** Enforces strict path traversal defenses against malicious archive entries.
pub fn extract_bundle(bundle_path: &Path, destination_dir: &Path) -> Result<Project> {
    if !bundle_path.exists() {
        return Err(DhapError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Bundle not found at '{}'", bundle_path.display()),
        )));
    }

    let file = File::open(bundle_path)?;
    let reader = BufReader::new(file);
    let mut archive = ZipArchive::new(reader).map_err(|e| DhapError::Zip(e.to_string()))?;

    fs::create_dir_all(destination_dir)?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| DhapError::Zip(e.to_string()))?;

        // CRITICAL SECURITY CHECK: enclosed_name ensures path does not escape destination
        let enclosed = match entry.enclosed_name() {
            Some(path) => path.to_owned(),
            None => {
                return Err(DhapError::PathTraversal(PathBuf::from(entry.name())));
            }
        };

        // Guard against any relative parent directories
        if enclosed
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err(DhapError::PathTraversal(enclosed));
        }

        let outpath = destination_dir.join(enclosed);

        if entry.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut entry, &mut outfile)?;
        }
    }

    // Read project.json
    let project_file_path = destination_dir.join("project.json");
    if !project_file_path.exists() {
        return Err(DhapError::Validation(
            "Archive is missing mandatory 'project.json'".to_string(),
        ));
    }

    let mut project_bytes = Vec::new();
    let mut pf = File::open(&project_file_path)?;
    pf.read_to_end(&mut project_bytes)?;

    let project: Project = serde_json::from_slice(&project_bytes)?;
    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::GuideStep;
    use tempfile::tempdir;

    #[test]
    fn test_bundle_save_and_extract_roundtrip() {
        let tmp = tempdir().expect("Failed to create tempdir");
        let mut project = Project::new("Test Workflow Guide");
        project.add_step(GuideStep::new(
            "Step 1: Open Settings",
            "Click on the cog icon in the top right corner.",
        ));
        project.add_step(GuideStep::new(
            "Step 2: Enable Local Mode",
            "Toggle the local mode switch to active.",
        ));

        // Create a dummy frame file
        let frames_dir = tmp.path().join("source_frames");
        fs::create_dir_all(&frames_dir).unwrap();
        let frame_file = frames_dir.join("frame_001.webp");
        fs::write(&frame_file, b"DUMMY_WEBP_IMAGE_BYTES").unwrap();

        let bundle_path = tmp.path().join("workflow.dhap");
        save_bundle(&project, Some(&frames_dir), &bundle_path).expect("Failed to save bundle");

        assert!(bundle_path.exists());

        // Extract bundle
        let extract_dir = tmp.path().join("extracted");
        let restored_project =
            extract_bundle(&bundle_path, &extract_dir).expect("Failed to extract bundle");

        assert_eq!(restored_project.title, "Test Workflow Guide");
        assert_eq!(restored_project.steps.len(), 2);
        assert_eq!(restored_project.steps[0].order, 1);
        assert_eq!(restored_project.steps[1].order, 2);

        // Verify frame was extracted
        let extracted_frame = extract_dir.join("frames").join("frame_001.webp");
        assert!(extracted_frame.exists());
        assert_eq!(
            fs::read(&extracted_frame).unwrap(),
            b"DUMMY_WEBP_IMAGE_BYTES"
        );
    }
}
