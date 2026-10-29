# Dhap (ধাপ) — Technical Architecture

## 1. System Overview

Dhap is architected around a **Modular Rust Domain Core** decoupled from the presentation layer. The application is organized as a unified monorepo containing Rust domain crates, a command-line interface tool, a modern web-based desktop studio, and a static product showcase site.

```text
                                  +-----------------------+
                                  |     Dhap Studio       |
                                  |   (TypeScript/React)  |
                                  +-----------+-----------+
                                              |
                             Typed IPC / CLI JSON Boundary
                                              |
+---------------------------------------------v----------------------------------------------+
|                                    Dhap Core Rust Workspace                                |
|                                                                                            |
|   +-------------------+   +--------------------+   +-------------------+   +-----------+   |
|   |    dhap-cli       |   |    dhap-export     |   |  dhap-annotations |   | dhap-media|   |
|   |  (CLI / Automation|   | (MD, HTML, PDF)    |   | (Vector & Redact) |   | (FFmpeg)  |   |
|   +---------+---------+   +----------+---------+   +---------+---------+   +-----+-----+   |
|             |                        |                       |                   |         |
|             +------------------------+-----------+-----------+-------------------+         |
|                                                  |                                         |
|                                       +----------v---------+                               |
|                                       |    dhap-project    |                               |
|                                       | (.dhap / Migrations|                               |
|                                       +----------+---------+                               |
|                                                  |                                         |
|                                       +----------v---------+                               |
|                                       |     dhap-core      |                               |
|                                       |  (Types, Errors)   |                               |
|                                       +--------------------+                               |
+--------------------------------------------------------------------------------------------+
```

---

## 2. Monorepo Organization

```text
dhap/
  ├── Cargo.toml                 # Root Rust workspace definition
  ├── package.json               # Monorepo root npm scripts
  ├── apps/
  │   ├── studio/                # Dhap Studio UI (React 19 + TypeScript + Vite)
  │   └── site/                  # Static product showcase & docs site (Cloudflare Pages)
  ├── crates/
  │   ├── dhap-core/             # Shared types, identifiers, timestamps, error models
  │   ├── dhap-project/          # .dhap container, manifest, schema migrations, security
  │   ├── dhap-media/            # FFmpeg integration, frame extraction, scene heuristics
  │   ├── dhap-annotations/      # Vector annotations, geometry, pixel redaction baking
  │   ├── dhap-export/           # Markdown, HTML5 guide, printable PDF generation
  │   └── dhap-cli/              # Command-line binary (`dhap`)
  ├── fixtures/                  # Synthetic media and sample projects for tests
  ├── docs/                      # Technical documentation, specifications, and ADRs
  └── .github/
      └── workflows/             # Automated CI, multi-platform build, and release pipelines
```

---

## 3. Crate Architecture & Responsibilities

### 3.1 `dhap-core`
- **Purpose:** Foundation layer containing zero domain side-effects.
- **Responsibilities:**
  - Standard typed identifier `Id` (UUID v4 / base62 formatted string).
  - Rational millisecond time representations (`TimestampMs`).
  - Structured error hierarchy (`DhapError`, `Result<T>`).
  - Common configuration traits.

### 3.2 `dhap-project`
- **Purpose:** Serialization, deserialization, and persistence of `.dhap` project archives.
- **Responsibilities:**
  - Canonical `Project` struct defining metadata, ordered sections, steps, frame references, and export profiles.
  - Portable `.dhap` ZIP bundle packaging and extraction.
  - **Security Gate:** Path traversal prevention. All ZIP entry paths are normalized and verified to prevent writing outside the target directory.
  - Atomic file writing (write to `.tmp` followed by atomic filesystem rename).
  - Versioned schema migrations from older project specifications.

### 3.3 `dhap-media`
- **Purpose:** Media inspection and frame extraction.
- **Responsibilities:**
  - Non-destructive probing of input media (codecs, dimensions, durations, frame rates).
  - Keyframe extraction at millisecond offsets via optimized FFmpeg subprocess execution.
  - Keyframe proposal algorithms based on scene difference thresholds and temporal intervals.
  - Clean error handling for corrupt, truncated, or unsupported media files.

### 3.4 `dhap-annotations`
- **Purpose:** Vector annotation primitives and pixel-level redaction rendering.
- **Responsibilities:**
  - Geometry models: `Rectangle`, `Arrow`, `Badge` (numbered markers), `Highlight`, `TextCallout`, `RedactionMask`.
  - Layer serialization: Stored as non-destructive JSON vectors in project metadata.
  - **Irreversible Redaction Baking Engine:** Renders masks onto raster image buffers (`image` crate). Blackout masks replace pixels with solid color; blur/pixelate destructively alters underlying pixel arrays. Unredacted source pixels are purged from exported artifacts.

### 3.5 `dhap-export`
- **Purpose:** Multi-format documentation generation.
- **Responsibilities:**
  - **Markdown Bundle:** Generates standard GitHub Flavored Markdown with clean frontmatter, relative image links, and step instructions.
  - **HTML5 Standalone Guide:** Self-contained, responsive single-page guide featuring light/dark theme toggles, accessible keyboard navigation, and embedded or local assets.
  - **Preflight Validator:** Enforces completeness (e.g., non-empty titles, resolved asset paths, privacy redaction checklists) prior to export execution.

### 3.6 `dhap-cli`
- **Purpose:** Command-line interface for automation and CI/CD pipelines.
- **Responsibilities:**
  - `dhap inspect <file>`: Report duration, streams, and suggested keyframes.
  - `dhap extract <file> --timestamps <list> --out <dir>`: Extract frames to disk.
  - `dhap validate <project.dhap>`: Validate schema and integrity.
  - `dhap export <project.dhap> --format <markdown|html|all> --out <dir>`: Render documentation.
  - `dhap batch <folder> --out <dir>`: Batch conversion of recordings.

---

## 4. The `.dhap` Project Format

A `.dhap` project is a self-contained ZIP archive with a documented internal structure:

```text
my-tutorial.dhap
├── manifest.json            # Bundle metadata, schema version, creation timestamp
├── project.json             # Canonical project document (steps, sections, references)
├── frames/                  # Extracted and annotated frame images (PNG / WebP)
│   ├── frame_001.webp
│   └── frame_002.webp
├── media/                   # Optional embedded source media (or relative link)
│   └── screen_record.mp4
└── exports/                 # Cached or recent export manifests
```

### Manifest Schema (`manifest.json`)
```json
{
  "dhap_version": "0.1.0",
  "schema_version": 1,
  "project_id": "018d45f3-...",
  "created_at": "2026-10-29T22:00:00Z",
  "updated_at": "2026-10-29T22:30:00Z",
  "media_mode": "linked"
}
```

---

## 5. Security & Threat Modeling

1. **Path Traversal Defense:** Both project extraction and export paths strictly disallow relative escape sequences (`..`) and absolute path injection.
2. **Untrusted Media Handling:** Media files are probed with execution timeouts and memory limits; malformed headers trigger controlled error states rather than crashes.
3. **Redaction Assurance:** Redacted image regions are overwritten in-memory before encoding to disk. The export pipeline verifies that no raw unredacted original image files are copied into the public export directory.
4. **Air-Gapped Privacy:** Core functionality does not initiate HTTP/WebSocket requests. Network activity occurs only when an optional external AI provider or Cloudflare Pages deployment is explicitly triggered by user action.
