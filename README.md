# Dhap (ধাপ) — Desktop Studio for Illustrated Guides

<p align="center">
  <strong>Make the steps clear.</strong><br>
  <em>Record once. Create the video and guide together.</em>
</p>

<p align="center">
  <a href="#about">About</a> •
  <a href="#key-features">Key Features</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#the-dhap-project-format">Project Format</a> •
  <a href="#roadmap">Roadmap</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>

---

## About

**Dhap (ধাপ)** is a modern, local-first, open-source desktop application that transforms screen recordings and visual workflows into editable, illustrated step-by-step documentation and media exports.

In Bengali, **ধাপ (dhap)** translates to a step in a staircase, or a stage/phase in a progression. “ধাপে ধাপে” means *step by step*. 

Technical creators, educators, maintainers, and support engineers often find documentation fragmented: one tool records a video, another grabs screenshots, an image editor draws arrows, and a word processor compiles the text. When the user interface changes, the guide and video drift apart.

Dhap solves this by providing a **single editable project** that unifies:
- The immutable source recording.
- Extracted keyframes with exact millisecond timestamps.
- Editable step titles, imperative instructions, expected outcomes, and notes.
- Vector annotations (numbered badges, directional arrows, callouts, highlights).
- Irreversible privacy redactions (baked directly into exported pixels).
- Multi-format exports: GitHub-Flavored Markdown bundles, standalone responsive HTML5 guides, printable documents, and video/GIF highlights.

---

## Core Product Commitments

1. **Local-First & Offline:** Core recording, frame extraction, step editing, redactions, and exports run entirely on your local machine. No internet required.
2. **Zero Paywalls & No Accounts:** No registration, no telemetry, no watermarks, and no artificial export limits.
3. **Single Source of Truth:** Changes made to a step or screenshot automatically synchronize across all exported formats.
4. **Human Editorial Control:** Automated step detections and AI hints are drafts. You review, edit, reorder, or reject before publishing.
5. **Irreversible Privacy Redaction:** Solid blackout masks and pixelation permanently overwrite underlying image data upon export, guaranteeing that private tokens, passwords, and emails are destroyed.

---

## Architecture Overview

Dhap is built on a high-performance **Modular Rust Domain Core** decoupled from the presentation layer:

```text
┌────────────────────────────────────────────────────────┐
│             Dhap Studio (React 19 + TypeScript)        │
│          Guide Layout • Timeline View • Canvas Annotate│
└───────────────────────────┬────────────────────────────┘
                            │ Typed IPC / CLI Contract
┌───────────────────────────v────────────────────────────┐
│                  Dhap Rust Domain Crates               │
│                                                        │
│  dhap-media       dhap-annotations       dhap-export   │
│  (FFmpeg Frame)   (Vector & Redaction)   (MD, HTML5)   │
│           │               │                   │        │
│           └───────────────┼───────────────────┘        │
│                           ▼                            │
│                      dhap-project                      │
│                  (.dhap ZIP / Migrations)              │
│                           ▼                            │
│                       dhap-core                        │
│               (Typed IDs, Timestamps, Errors)          │
└────────────────────────────────────────────────────────┘
```

- **`crates/dhap-core`**: Foundation crate defining strongly typed identifiers, rational millisecond timestamps, and structured errors.
- **`crates/dhap-project`**: Project schema v1, `.dhap` ZIP packaging, path-traversal security guards, and atomic file saving.
- **`crates/dhap-media`**: Non-destructive video probing, streaming frame extraction via FFmpeg, and scene-change detection heuristics.
- **`crates/dhap-annotations`**: Vector annotation geometry and pixel-level destructive redaction baking.
- **`crates/dhap-export`**: Multi-format documentation exporters (Markdown with assets, responsive standalone HTML5 single-page guide).
- **`crates/dhap-cli`**: Headless automation CLI for batch processing and CI pipelines.
- **`apps/studio`**: Modern, accessible desktop editor with Guide, Timeline, and Preview layouts.
- **`apps/site`**: Static documentation and showcase website.

---

## Quick Start

### 1. Build the Rust Engine & CLI

Ensure you have Rust 1.85+ and FFmpeg 6.0+ installed.

```bash
# Clone the repository
git clone https://github.com/nhprince/dhap-studio.git
cd dhap-studio

# Build all crates
cargo build --workspace

# Run all automated tests
cargo test --workspace
```

### 2. Inspect Media & Export with `dhap` CLI

```bash
# Inspect a screen recording
cargo run -p dhap-cli -- inspect fixtures/sample_recording.mp4

# Validate an existing .dhap project
cargo run -p dhap-cli -- validate my_guide.dhap

# Export to standalone HTML and Markdown
cargo run -p dhap-cli -- export my_guide.dhap --format all --out ./dist-guide
```

### 3. Launch Dhap Studio UI

```bash
cd apps/studio
npm install
npm run dev
```

---

## The `.dhap` Project Format

A `.dhap` file is a portable, versioned ZIP bundle designed for longevity and transparency:

```text
my-workflow.dhap
├── manifest.json       # Bundle schema version, project ID, and timestamps
├── project.json        # Canonical step definitions, sections, and references
├── frames/             # Extracted and annotated frame assets
│   ├── frame_001.webp
│   └── frame_002.webp
└── media/              # Linked or embedded source recording
    └── recording.mp4
```

---

## Documentation

- [Product Specification](docs/PRODUCT_SPEC.md)
- [Technical Architecture](docs/ARCHITECTURE.md)
- [Architecture Decision Records (ADRs)](docs/DECISIONS/ADR-0001-architecture-selection.md)
- [Developer Guide](docs/DEVELOPMENT.md)
- [Release Engineering](docs/RELEASING.md)
- [Security Policy](docs/SECURITY.md)
- [Implementation Status](docs/IMPLEMENTATION_STATUS.md)
- [Contributing Guide](docs/CONTRIBUTING.md)
- [Third-Party Notices](docs/THIRD_PARTY_NOTICES.md)

---

## License

Dhap is free, open-source software licensed under the **[MIT License](LICENSE)**.
