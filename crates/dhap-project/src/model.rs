//! Data models and schemas for Dhap projects.

use chrono::{DateTime, Utc};
use dhap_core::{now_utc, Id, TimestampMs};
use serde::{Deserialize, Serialize};

/// Current supported project schema version.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Application release version string.
pub const DHAP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The root project document for a Dhap workflow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    pub id: Id,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema_version: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(default)]
    pub sources: Vec<SourceMedia>,
    #[serde(default)]
    pub sections: Vec<GuideSection>,
    #[serde(default)]
    pub steps: Vec<GuideStep>,
    #[serde(default)]
    pub export_profile: ExportProfile,
}

impl Project {
    /// Creates a new project with a default section and timestamps.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        let now = now_utc();
        Self {
            id: Id::new(),
            title: title.into(),
            description: None,
            schema_version: CURRENT_SCHEMA_VERSION,
            created_at: now,
            updated_at: now,
            sources: Vec::new(),
            sections: Vec::new(),
            steps: Vec::new(),
            export_profile: ExportProfile::default(),
        }
    }

    /// Appends a new step to the project.
    pub fn add_step(&mut self, mut step: GuideStep) {
        step.order = self.steps.len() + 1;
        self.steps.push(step);
        self.updated_at = now_utc();
    }

    /// Reorders steps sequentially starting from 1.
    pub fn normalize_order(&mut self) {
        for (idx, step) in self.steps.iter_mut().enumerate() {
            step.order = idx + 1;
        }
    }

    /// Removes a step by ID and re-normalizes ordering.
    pub fn remove_step(&mut self, step_id: &Id) -> bool {
        let initial_len = self.steps.len();
        self.steps.retain(|s| s.id != *step_id);
        let removed = self.steps.len() < initial_len;
        if removed {
            self.normalize_order();
            self.updated_at = now_utc();
        }
        removed
    }
}

/// A media file referenced or embedded within the project.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceMedia {
    pub id: Id,
    pub file_name: String,
    pub relative_path: String,
    pub duration_ms: TimestampMs,
    pub width: u32,
    pub height: u32,
    pub frame_rate: f64,
    pub format: String,
    #[serde(default)]
    pub is_embedded: bool,
}

/// A titled group of steps for table of contents and structuring.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuideSection {
    pub id: Id,
    pub title: String,
    pub order: usize,
}

/// A single step in the generated or authored guide.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuideStep {
    pub id: Id,
    pub order: usize,
    pub title: String,
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_relative_path: Option<String>,
    #[serde(default)]
    pub timestamp_ms: TimestampMs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_id: Option<Id>,
    #[serde(default)]
    pub is_reviewed: bool,
}

impl GuideStep {
    /// Creates a new basic step.
    #[must_use]
    pub fn new(title: impl Into<String>, instruction: impl Into<String>) -> Self {
        Self {
            id: Id::with_prefix("step"),
            order: 1,
            title: title.into(),
            instruction: instruction.into(),
            expected_result: None,
            notes: None,
            frame_id: None,
            frame_relative_path: None,
            timestamp_ms: TimestampMs::ZERO,
            section_id: None,
            is_reviewed: false,
        }
    }
}

/// Configuration profile governing documentation exports.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportProfile {
    pub title_override: Option<String>,
    #[serde(default = "default_true")]
    pub include_timestamps: bool,
    #[serde(default = "default_true")]
    pub include_toc: bool,
    #[serde(default = "default_true")]
    pub safe_redaction_mode: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_true() -> bool {
    true
}

fn default_theme() -> String {
    "auto".to_string()
}

impl Default for ExportProfile {
    fn default() -> Self {
        Self {
            title_override: None,
            include_timestamps: true,
            include_toc: true,
            safe_redaction_mode: true,
            theme: default_theme(),
        }
    }
}

/// Manifest stored at the root of a `.dhap` ZIP bundle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Manifest {
    pub dhap_version: String,
    pub schema_version: u32,
    pub project_id: Id,
    pub created_at: DateTime<Utc>,
    pub media_mode: String,
}

impl Manifest {
    #[must_use]
    pub fn from_project(project: &Project, media_mode: impl Into<String>) -> Self {
        Self {
            dhap_version: DHAP_VERSION.to_string(),
            schema_version: project.schema_version,
            project_id: project.id.clone(),
            created_at: project.created_at,
            media_mode: media_mode.into(),
        }
    }
}
