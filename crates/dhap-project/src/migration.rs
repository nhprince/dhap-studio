//! Schema version detection, migration logic, and automatic backup creation.

use crate::model::{Project, CURRENT_SCHEMA_VERSION};
use dhap_core::{DhapError, Result};
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Inspects arbitrary project JSON and migrates it to the current schema version if required.
pub fn migrate_project_json(raw_json: &str, backup_dir: Option<&Path>) -> Result<Project> {
    let mut val: Value = serde_json::from_str(raw_json)?;

    let schema_ver = val
        .get("schema_version")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as u32;

    if schema_ver > CURRENT_SCHEMA_VERSION {
        return Err(DhapError::UnsupportedSchema {
            found: schema_ver,
            supported: CURRENT_SCHEMA_VERSION,
        });
    }

    if let Some(b_dir) = backup_dir {
        fs::create_dir_all(b_dir)?;
        let backup_file = b_dir.join(format!("project_backup_v{schema_ver}.json"));
        fs::write(backup_file, raw_json)?;
    }

    // Apply incremental migrations here as schema evolves
    if schema_ver == 0 {
        // Example legacy v0 to v1 migration
        if let Some(obj) = val.as_object_mut() {
            obj.insert(
                "schema_version".to_string(),
                serde_json::json!(CURRENT_SCHEMA_VERSION),
            );
            if !obj.contains_key("sections") {
                obj.insert("sections".to_string(), serde_json::json!([]));
            }
        }
    }

    let project: Project = serde_json::from_value(val)?;
    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_from_v0_unversioned() {
        let legacy_json = r#"{
            "id": "legacy_001",
            "title": "Legacy Project Without Schema Version",
            "created_at": "2026-10-22T10:00:00Z",
            "updated_at": "2026-10-22T10:00:00Z",
            "steps": []
        }"#;

        let migrated = migrate_project_json(legacy_json, None).expect("Migration failed");
        assert_eq!(migrated.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(migrated.title, "Legacy Project Without Schema Version");
    }

    #[test]
    fn test_unsupported_future_schema() {
        let future_json = r#"{
            "id": "future_001",
            "title": "Future Project",
            "schema_version": 999,
            "created_at": "2026-10-22T10:00:00Z",
            "updated_at": "2026-10-22T10:00:00Z"
        }"#;

        let res = migrate_project_json(future_json, None);
        assert!(matches!(res, Err(DhapError::UnsupportedSchema { .. })));
    }
}
