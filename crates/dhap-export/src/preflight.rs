//! Preflight checks and validation prior to documentation export.

use dhap_core::{DhapError, Result};
use dhap_project::Project;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Result summary of preflight validation checks.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreflightReport {
    pub is_valid: bool,
    pub step_count: usize,
    pub missing_frame_count: usize,
    pub redaction_count: usize,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Runs comprehensive preflight validation on a project before triggering export.
pub fn validate_project_preflight(
    project: &Project,
    frames_base_dir: Option<&Path>,
) -> Result<PreflightReport> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut missing_frame_count = 0;
    let redaction_count = 0;

    if project.title.trim().is_empty() {
        errors.push("Project title cannot be empty.".to_string());
    }

    if project.steps.is_empty() {
        warnings.push("Project contains zero guide steps.".to_string());
    }

    for (idx, step) in project.steps.iter().enumerate() {
        let step_num = idx + 1;
        if step.title.trim().is_empty() {
            errors.push(format!("Step {step_num} has an empty title."));
        }
        if step.instruction.trim().is_empty() {
            warnings.push(format!(
                "Step {step_num} ('{}') has empty instructions.",
                step.title
            ));
        }

        // Verify frame asset existence if frames_base_dir is specified
        if let Some(frame_rel) = &step.frame_relative_path {
            if let Some(base) = frames_base_dir {
                let full_path = base.join(frame_rel);
                if !full_path.exists() {
                    missing_frame_count += 1;
                    warnings.push(format!(
                        "Step {step_num} references frame '{}' which does not exist on disk.",
                        frame_rel
                    ));
                }
            }
        }
    }

    let is_valid = errors.is_empty();

    if !is_valid {
        return Err(DhapError::Validation(format!(
            "Preflight failed with {} error(s): {}",
            errors.len(),
            errors.join("; ")
        )));
    }

    Ok(PreflightReport {
        is_valid,
        step_count: project.steps.len(),
        missing_frame_count,
        redaction_count,
        warnings,
        errors,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use dhap_project::GuideStep;

    #[test]
    fn test_preflight_valid_project() {
        let mut proj = Project::new("Valid Guide");
        proj.add_step(GuideStep::new("Step 1", "Do something useful."));

        let report = validate_project_preflight(&proj, None).unwrap();
        assert!(report.is_valid);
        assert_eq!(report.step_count, 1);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_preflight_empty_title_fails() {
        let mut proj = Project::new("");
        proj.add_step(GuideStep::new("Step 1", "Instructions."));

        let res = validate_project_preflight(&proj, None);
        assert!(res.is_err());
    }
}
