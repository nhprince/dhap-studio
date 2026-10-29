//! # dhap-project
//!
//! Project format definitions, JSON schemas, atomic persistence, schema migrations,
//! and secure `.dhap` ZIP bundle packaging for Dhap.

pub mod bundle;
pub mod migration;
pub mod model;

pub use bundle::{extract_bundle, save_bundle};
pub use migration::migrate_project_json;
pub use model::{
    ExportProfile, GuideSection, GuideStep, Manifest, Project, SourceMedia, CURRENT_SCHEMA_VERSION,
    DHAP_VERSION,
};
