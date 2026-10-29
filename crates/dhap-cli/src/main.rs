//! Dhap (ধাপ) CLI — Automation and terminal interface for illustrated guides.

use clap::{Parser, Subcommand, ValueEnum};
use dhap_core::{DhapError, Result, TimestampMs};
use dhap_export::{export_documentation, validate_project_preflight, ExportFormat};
use dhap_media::{extract_frame_at_timestamp, probe_media_file, propose_keyframes_by_interval};
use dhap_project::{extract_bundle, DHAP_VERSION};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "dhap",
    about = "Dhap (ধাপ) — Record once. Create the video and guide together.",
    version = DHAP_VERSION
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output results in structured JSON format
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe media metadata and calculate candidate keyframes
    Inspect {
        /// Path to video file
        video: PathBuf,

        /// Interval in seconds for candidate keyframe proposal
        #[arg(long, default_value = "5")]
        interval: u64,
    },

    /// Extract still frame images at designated millisecond timestamps
    Extract {
        /// Path to video file
        video: PathBuf,

        /// Timestamp offset in milliseconds (e.g. 1500 for 1.5s)
        #[arg(long)]
        timestamp: u64,

        /// Destination path for extracted frame image
        #[arg(long, short)]
        out: PathBuf,
    },

    /// Validate a .dhap project bundle for schema compliance and asset integrity
    Validate {
        /// Path to .dhap bundle file
        project: PathBuf,
    },

    /// Export documentation from a .dhap project bundle
    Export {
        /// Path to .dhap bundle file
        project: PathBuf,

        /// Documentation format to export
        #[arg(long, short, value_enum, default_value_t = CliExportFormat::All)]
        format: CliExportFormat,

        /// Output destination directory
        #[arg(long, short)]
        out: PathBuf,
    },

    /// Print version, license, and system diagnostic information
    Diagnostics,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum CliExportFormat {
    Markdown,
    Html,
    All,
}

impl From<CliExportFormat> for ExportFormat {
    fn from(c: CliExportFormat) -> Self {
        match c {
            CliExportFormat::Markdown => ExportFormat::Markdown,
            CliExportFormat::Html => ExportFormat::Html,
            CliExportFormat::All => ExportFormat::All,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let res = match cli.command {
        Commands::Inspect { video, interval } => handle_inspect(video, interval, cli.json),
        Commands::Extract {
            video,
            timestamp,
            out,
        } => handle_extract(video, timestamp, out, cli.json),
        Commands::Validate { project } => handle_validate(project, cli.json),
        Commands::Export {
            project,
            format,
            out,
        } => handle_export(project, format.into(), out, cli.json),
        Commands::Diagnostics => handle_diagnostics(cli.json),
    };

    if let Err(e) = res {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn handle_inspect(video: PathBuf, interval_sec: u64, as_json: bool) -> Result<()> {
    let probe = probe_media_file(&video)?;
    let proposals = propose_keyframes_by_interval(
        probe.duration,
        TimestampMs::from_seconds_f64(interval_sec as f64),
    );

    if as_json {
        let json_val = serde_json::json!({
            "probe": probe,
            "proposals": proposals,
        });
        println!("{}", serde_json::to_string_pretty(&json_val)?);
    } else {
        println!("=== Dhap Media Inspection: {} ===", probe.file_name);
        println!("  Duration:   {}", probe.duration.format_hh_mm_ss_mmm());
        println!("  Resolution: {}x{}", probe.width, probe.height);
        println!("  Frame Rate: {:.2} fps", probe.frame_rate);
        println!("  Codec:      {}", probe.codec);
        println!("  Format:     {}", probe.format);
        println!("\nSuggested Keyframes (every {interval_sec}s):");
        for (i, p) in proposals.iter().enumerate() {
            println!(
                "  [{:>2}] {} — {}",
                i + 1,
                p.timestamp.format_mm_ss(),
                p.reason
            );
        }
    }
    Ok(())
}

fn handle_extract(video: PathBuf, timestamp_ms: u64, out: PathBuf, as_json: bool) -> Result<()> {
    let ts = TimestampMs::from_millis(timestamp_ms);
    extract_frame_at_timestamp(&video, ts, &out)?;

    if as_json {
        let val = serde_json::json!({
            "status": "extracted",
            "timestamp_ms": timestamp_ms,
            "output_path": out.display().to_string(),
        });
        println!("{}", serde_json::to_string_pretty(&val)?);
    } else {
        println!(
            "Extracted frame at {} -> {}",
            ts.format_hh_mm_ss_mmm(),
            out.display()
        );
    }
    Ok(())
}

fn handle_validate(bundle_path: PathBuf, as_json: bool) -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let project = extract_bundle(&bundle_path, tmp.path())?;
    let report = validate_project_preflight(&project, Some(&tmp.path().join("frames")))?;

    if as_json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("=== Dhap Project Validation: '{}' ===", project.title);
        println!("  Schema Version: {}", project.schema_version);
        println!("  Steps Count:    {}", report.step_count);
        println!(
            "  Valid Status:   {}",
            if report.is_valid { "PASS" } else { "FAIL" }
        );
        if !report.warnings.is_empty() {
            println!("  Warnings ({}):", report.warnings.len());
            for w in &report.warnings {
                println!("    - {w}");
            }
        }
    }

    if !report.is_valid {
        return Err(DhapError::Validation(
            "Project validation failed preflight checks.".to_string(),
        ));
    }
    Ok(())
}

fn handle_export(
    bundle_path: PathBuf,
    format: ExportFormat,
    out: PathBuf,
    as_json: bool,
) -> Result<()> {
    let tmp = tempfile::tempdir()?;
    let project = extract_bundle(&bundle_path, tmp.path())?;
    let frames_dir = tmp.path().join("frames");

    let report = export_documentation(&project, Some(&frames_dir), &out, format)?;

    if as_json {
        let val = serde_json::json!({
            "status": "success",
            "output_directory": out.display().to_string(),
            "report": report,
        });
        println!("{}", serde_json::to_string_pretty(&val)?);
    } else {
        println!("=== Export Complete ===");
        println!("  Project:          '{}'", project.title);
        println!("  Steps Exported:   {}", report.step_count);
        println!("  Destination:      {}", out.display());
    }
    Ok(())
}

fn handle_diagnostics(as_json: bool) -> Result<()> {
    if as_json {
        let diag = serde_json::json!({
            "app_name": "Dhap",
            "bengali_name": "ধাপ",
            "tagline": "Make the steps clear.",
            "version": DHAP_VERSION,
            "license": "MIT",
            "repository": "https://github.com/nhprince/dhap-studio"
        });
        println!("{}", serde_json::to_string_pretty(&diag)?);
    } else {
        println!("Dhap (ধাপ) — v{}", DHAP_VERSION);
        println!("Tagline:     Make the steps clear.");
        println!("License:     MIT");
        println!("Repository:  https://github.com/nhprince/dhap-studio");
        println!("Status:      Local-first, offline-ready documentation studio.");
    }
    Ok(())
}
