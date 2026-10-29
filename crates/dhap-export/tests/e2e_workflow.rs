use dhap_annotations::Annotation;
use dhap_core::TimestampMs;
use dhap_export::{export_documentation, ExportFormat};
use dhap_media::create_synthetic_frame;
use dhap_project::{extract_bundle, save_bundle, GuideStep, Project};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_full_e2e_vertical_slice() {
    let tmp = tempdir().expect("Failed to create temporary test directory");
    let workspace = tmp.path();

    // 1. Create frames directory and generate representative screenshots
    let frames_dir = workspace.join("frames");
    fs::create_dir_all(&frames_dir).unwrap();

    let frame1_path = frames_dir.join("step_01.png");
    let frame2_path = frames_dir.join("step_02.png");

    create_synthetic_frame(1280, 720, "Step 1: Dashboard", &frame1_path).unwrap();
    create_synthetic_frame(1280, 720, "Step 2: API Keys", &frame2_path).unwrap();

    assert!(frame1_path.exists());
    assert!(frame2_path.exists());

    // 2. Create project model
    let mut project = Project::new("Production Deployment Walkthrough");
    project.description = Some(
        "A step-by-step verified workflow guide for deploying services with Dhap.".to_string(),
    );

    let mut step1 = GuideStep::new(
        "Access Cloud Settings",
        "Navigate to the organization settings tab on the primary sidebar.",
    );
    step1.frame_relative_path = Some("step_01.png".to_string());
    step1.timestamp_ms = TimestampMs::from_seconds_f64(4.5);
    step1.expected_result = Some("Settings panel opens showing organization overview.".to_string());
    project.add_step(step1);

    let mut step2 = GuideStep::new(
        "Generate Secret Token",
        "Click on 'Create API Token' and store it in your local password manager.",
    );
    step2.frame_relative_path = Some("step_02.png".to_string());
    step2.timestamp_ms = TimestampMs::from_seconds_f64(14.2);
    step2.notes = Some("Never share secret tokens over unencrypted channels.".to_string());
    project.add_step(step2);

    // 3. Apply redaction and annotation on step 2 frame
    let redaction = Annotation::blackout(100, 200, 300, 40);
    let badge = Annotation::number_badge(120, 180, 1);
    let mut step2_image = image::open(&frame2_path).unwrap().to_rgba8();
    dhap_annotations::bake_annotations_in_place(&mut step2_image, &[redaction, badge], true)
        .unwrap();
    step2_image.save(&frame2_path).unwrap();

    // 4. Save to versioned .dhap bundle
    let bundle_path = workspace.join("production_guide.dhap");
    save_bundle(&project, Some(&frames_dir), &bundle_path).unwrap();
    assert!(bundle_path.exists());

    // 5. Reopen and verify project data integrity
    let extract_dir = workspace.join("restored_project");
    let restored = extract_bundle(&bundle_path, &extract_dir).unwrap();
    assert_eq!(restored.title, "Production Deployment Walkthrough");
    assert_eq!(restored.steps.len(), 2);
    assert_eq!(restored.steps[0].order, 1);
    assert_eq!(restored.steps[1].order, 2);

    // 6. Export to Markdown and Standalone HTML
    let export_dir = workspace.join("dist_documentation");
    let report = export_documentation(
        &restored,
        Some(&extract_dir.join("frames")),
        &export_dir,
        ExportFormat::All,
    )
    .unwrap();

    assert!(report.is_valid);
    assert_eq!(report.step_count, 2);

    // 7. Validate exported artifacts
    let md_file = export_dir.join("guide.md");
    let html_file = export_dir.join("index.html");
    let asset1 = export_dir.join("assets").join("step_1.png");
    let asset2 = export_dir.join("assets").join("step_2.png");

    assert!(md_file.exists(), "guide.md must exist");
    assert!(html_file.exists(), "index.html must exist");
    assert!(asset1.exists(), "assets/step_1.png must exist");
    assert!(asset2.exists(), "assets/step_2.png must exist");

    let md_content = fs::read_to_string(&md_file).unwrap();
    assert!(md_content.contains("# Production Deployment Walkthrough"));
    assert!(md_content.contains("### Step 1: Access Cloud Settings"));
    assert!(md_content.contains("### Step 2: Generate Secret Token"));
    assert!(md_content.contains("![Step 1: Access Cloud Settings](assets/step_1.png)"));

    let html_content = fs::read_to_string(&html_file).unwrap();
    assert!(html_content.contains("Production Deployment Walkthrough — Dhap Guide"));
    assert!(html_content.contains("Access Cloud Settings"));
    assert!(html_content.contains("Generate Secret Token"));
    assert!(html_content.contains("assets/step_2.png"));
}
