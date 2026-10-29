# Dhap (ধাপ) — Developer Guide

Welcome to Dhap! This guide provides everything required to build, test, and contribute to the project.

---

## 1. Prerequisites

Ensure the following tools are installed in your environment:
- **Rust:** Stable 1.85+ (MSVC on Windows, GCC/Clang on Linux/macOS). Install via [rustup.rs](https://rustup.rs).
- **Node.js:** v20+ (LTS or Current) and **npm** v10+.
- **FFmpeg:** 6.0+ installed and available on your system `PATH`.
  - Windows: `winget install Gyan.FFmpeg` or download from [gyan.dev](https://www.gyan.dev/ffmpeg/builds/).
  - macOS: `brew install ffmpeg`
  - Linux: `sudo apt-get install ffmpeg` (or distribution equivalent).
- **Git** and **GitHub CLI (`gh`)** for version control.

---

## 2. Repository Layout

```text
dhap/
  ├── Cargo.toml          # Rust workspace definition
  ├── package.json        # Frontend scripts and orchestration
  ├── crates/
  │   ├── dhap-core/      # Types, identifiers, timestamps, errors
  │   ├── dhap-project/   # Schema serialization, .dhap archive, migrations
  │   ├── dhap-media/     # FFmpeg probing, frame extraction, scene heuristics
  │   ├── dhap-annotations/# Vector geometry, irreversible redaction baking
  │   ├── dhap-export/    # Markdown, HTML5 guide, printable PDF generation
  │   └── dhap-cli/       # Command-line binary (`dhap`)
  ├── apps/
  │   ├── studio/         # React/TypeScript desktop editor UI
  │   └── site/           # Static product website
  └── docs/               # Architecture, specs, and decision records
```

---

## 3. Building the Rust Crates

Build the entire Rust workspace:
```bash
cargo build --workspace
```

Build for release:
```bash
cargo build --workspace --release
```

---

## 4. Running Tests & Quality Checks

Run all unit and integration tests:
```bash
cargo test --workspace
```

Verify formatting:
```bash
cargo fmt --all -- --check
```

Run Clippy linter with strict warning enforcement:
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

---

## 5. Running the `dhap` CLI

Execute the CLI directly from Cargo:
```bash
# Print version and help
cargo run -p dhap-cli -- --help

# Inspect a media file
cargo run -p dhap-cli -- inspect path/to/video.mp4

# Validate a .dhap project file
cargo run -p dhap-cli -- validate path/to/project.dhap

# Export documentation
cargo run -p dhap-cli -- export path/to/project.dhap --format html --out ./dist-docs
```

---

## 6. Running the Studio Frontend

Install dependencies and start the local Vite development server:
```bash
cd apps/studio
npm install
npm run dev
```

Build the production frontend bundle:
```bash
npm run build
```

Run frontend linting and type checks:
```bash
npm run lint
npm run typecheck
```

---

## 7. Golden Fixtures & Verification

Golden tests are located in `crates/dhap-export/tests/fixtures` and verify that generated Markdown and HTML match approved snapshots.
When modifying export templates, update the fixtures and ensure tests pass:
```bash
cargo test -p dhap-export --test golden_exports
```
