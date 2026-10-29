//! GitHub-Flavored Markdown documentation exporter.

use dhap_core::Result;
use dhap_project::Project;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Exports a project to a standard GitHub-Flavored Markdown bundle at `output_dir`.
///
/// Output structure:
/// ```text
/// output_dir/
///   ├── guide.md
///   └── assets/
///       ├── step_1.png
///       └── step_2.png
/// ```
pub fn export_markdown(
    project: &Project,
    frames_dir: Option<&Path>,
    output_dir: &Path,
) -> Result<()> {
    fs::create_dir_all(output_dir)?;
    let assets_dir = output_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;

    let mut md = String::new();

    // 1. Frontmatter
    md.push_str("---\n");
    md.push_str(&format!("title: \"{}\"\n", project.title));
    if let Some(desc) = &project.description {
        md.push_str(&format!("description: \"{desc}\"\n"));
    }
    md.push_str(&format!(
        "generator: \"Dhap (ধাপ) v{}\"\n",
        project.schema_version
    ));
    md.push_str(&format!(
        "date: \"{}\"\n",
        project.updated_at.format("%Y-%m-%d")
    ));
    md.push_str("---\n\n");

    // 2. Title & Header
    md.push_str(&format!("# {}\n\n", project.title));
    if let Some(desc) = &project.description {
        md.push_str(&format!("> {desc}\n\n"));
    }

    // 3. Table of Contents
    if project.export_profile.include_toc && !project.steps.is_empty() {
        md.push_str("## Table of Contents\n\n");
        for step in &project.steps {
            let anchor = step
                .title
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
                .replace(' ', "-");
            md.push_str(&format!(
                "- [Step {}: {}](#step-{}-{})\n",
                step.order, step.title, step.order, anchor
            ));
        }
        md.push_str("\n---\n\n");
    }

    // 4. Steps
    for step in &project.steps {
        md.push_str(&format!("### Step {}: {}\n\n", step.order, step.title));

        if project.export_profile.include_timestamps && step.timestamp_ms.as_millis() > 0 {
            md.push_str(&format!(
                "*Timestamp: `{}`*\n\n",
                step.timestamp_ms.format_mm_ss()
            ));
        }

        md.push_str(&format!("{}\n\n", step.instruction));

        // Copy and reference frame asset
        if let Some(frame_rel) = &step.frame_relative_path {
            let asset_name = format!("step_{}.png", step.order);
            let dest_asset_path = assets_dir.join(&asset_name);

            if let Some(f_base) = frames_dir {
                let src_path = f_base.join(frame_rel);
                if src_path.exists() {
                    let _ = fs::copy(&src_path, &dest_asset_path);
                }
            }

            md.push_str(&format!(
                "![Step {}: {}](assets/{})\n\n",
                step.order, step.title, asset_name
            ));
        }

        if let Some(expected) = &step.expected_result {
            md.push_str(&format!("**Expected Result:** {}\n\n", expected));
        }

        if let Some(notes) = &step.notes {
            md.push_str(&format!("> **Note:** {}\n\n", notes));
        }

        md.push_str("---\n\n");
    }

    // 5. Footer
    md.push_str(
        "*Documentation generated locally with [Dhap (ধাপ)](https://github.com/nhprince/dhap-studio) — Record once. Create the video and guide together.*\n",
    );

    let md_path = output_dir.join("guide.md");
    let mut file = File::create(md_path)?;
    file.write_all(md.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dhap_project::GuideStep;
    use tempfile::tempdir;

    #[test]
    fn test_export_markdown_generation() {
        let tmp = tempdir().unwrap();
        let mut proj = Project::new("Getting Started with Dhap");
        proj.add_step(GuideStep::new(
            "Launch Dhap Studio",
            "Open Dhap from your desktop applications menu.",
        ));

        let out = tmp.path().join("markdown_out");
        export_markdown(&proj, None, &out).unwrap();

        let guide_md = out.join("guide.md");
        assert!(guide_md.exists());

        let content = fs::read_to_string(guide_md).unwrap();
        assert!(content.contains("# Getting Started with Dhap"));
        assert!(content.contains("### Step 1: Launch Dhap Studio"));
        assert!(content.contains("Open Dhap from your desktop applications menu."));
    }
}
