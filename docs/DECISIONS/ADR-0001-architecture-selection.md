# ADR-0001: Architecture & Technology Selection for Dhap (ধাপ)

- **Status:** Accepted  
- **Date:** 29 October 2026  
- **Authors:** Dhap Core Architectural Team  
- **Deciders:** Lead Architect, Product Designer, Senior Rust Engineer  

---

## Context and Problem Statement

Dhap (ধাপ) is an open-source, local-first desktop application designed to transform screen recordings and visual workflows into editable, illustrated documentation and media outputs. 

The core product promise is: **"Record once. Create the video and guide together."**

Unlike a pure video editor or a simple markdown editor, Dhap must maintain a single source of truth connecting:
1. Source media (screen recordings, window captures, imported video files).
2. Keyframes and timestamps.
3. Editable step-by-step instructions (titles, steps, expected outcomes, notes).
4. Visual annotations (arrows, number badges, callouts, highlights).
5. Irreversible redaction masks (solid masks, blur, pixelation baked into export pixels).
6. Multi-format exports (Markdown bundles, standalone HTML guides, printable PDFs, video/GIF clips).
7. Versioned, portable project files (`.dhap` bundles).

We need an architecture that balances native performance, rock-solid media processing, responsive creative UI, cross-platform packaging, licensing safety, and solo-maintainer development velocity.

---

## Evaluated Architectural Options

### Option A: Direct Fork & Monolithic Extension of OpenScreen
- **Description:** Directly fork `getopenscreen/openscreen` (Electron + React + Rust GPU recording/rendering core) and embed the documentation editor inside its existing Electron codebase.
- **Pros:**
  - Instant access to OpenScreen's existing OS-specific screen/window recording and GPU rendering pipelines.
  - Retains existing timeline and canvas controls.
- **Cons:**
  - Upstream documentation explicitly warns that `.openscreen` project formats and CLI commands are undergoing rapid, breaking internal changes.
  - Tightly couples the documentation data model to OpenScreen's internal state machine.
  - Huge repository size, long build times, and high merge overhead from an external upstream.
  - Electron runtime footprint (150MB+ bundle size, 200MB+ baseline RAM).

### Option B: Modular Rust Domain Engine with Modern Web Studio Shell (Selected)
- **Description:** Decouple domain logic from UI. Build a standalone, portable Rust workspace (`dhap-core`, `dhap-project`, `dhap-media`, `dhap-annotations`, `dhap-export`, `dhap-cli`) and pair it with a tailored TypeScript/React studio frontend running in a lightweight desktop shell.
- **Pros:**
  - **Separation of Concerns:** 100% of project serialization, atomic saves, migration, media extraction, annotation rasterization, redaction baking, and export generation lives in pure, testable Rust crates.
  - **Deterministic Testing:** Domain logic, golden export tests, archive security, and CLI runs headless in CI on every OS without requiring an active display server or GUI runner.
  - **CLI & MCP Automation:** The `dhap` CLI shares identical crates with the desktop application, enabling scriptable documentation pipelines and future AI agent tools.
  - **Clean Upstream Boundary:** Interfaces with media tools (FFmpeg, system recorders, or OpenScreen export adapters) via clean contracts rather than internal memory coupling.
  - **Creative UI Velocity:** TypeScript + React provides the richest ecosystem for canvas-based annotation, accessible keyboard navigation, rich text editing, and live HTML/CSS export previewing.
  - **Licensing Safety:** Core crates are strictly MIT/Apache-2.0. No copyleft or viral licensing concerns.
- **Cons:**
  - Requires maintaining the IPC/communication bridge between the Rust core and the web frontend.

### Option C: Pure Native Rust UI (Slint or Iced)
- **Description:** Build both the backend and GUI in 100% Rust using Slint or Iced.
- **Pros:**
  - Single language across the entire codebase; tiny binary footprint (~15-20MB).
  - Fast startup and low memory usage.
- **Cons:**
  - **Licensing Pitfalls:** Slint is triple-licensed (GPLv3, Commercial, or Royalty-Free with mandatory "About Slint" attribution branding). A GPLv3 core limits permissive adoption, and mandatory attribution widgets restrict branding freedom.
  - **Ecosystem Limitations:** Slint/Iced lack mature, accessible components for rich document rendering, syntax highlighting, print CSS previewing, and interactive HTML guide export sandboxing.
  - Significantly higher UI implementation effort for a solo developer.

---

## Decision Matrix

| Evaluation Criterion | Weight | Option A (OpenScreen Fork) | Option B (Modular Rust + Web Studio) | Option C (Slint / Native Rust) |
|:---|:---:|:---:|:---:|:---:|
| **Media & Frame Processing Performance** | 15% | 8/10 | **9/10** | 9/10 |
| **Documentation Editor Flexibility** | 20% | 6/10 | **10/10** | 4/10 |
| **Data Model & Project Format Ownership** | 15% | 5/10 | **10/10** | 9/10 |
| **Development Speed & Maintainability** | 15% | 6/10 | **9/10** | 5/10 |
| **Cross-Platform Reliability** | 10% | 8/10 | **9/10** | 7/10 |
| **Licensing Purity (Permissive MIT)** | 10% | 9/10 | **10/10** | 5/10 |
| **Headless CI & Automated Golden Testing**| 10% | 5/10 | **10/10** | 7/10 |
| **CLI / Agent / MCP Readiness** | 5% | 5/10 | **10/10** | 7/10 |
| **Weighted Total** | **100%** | **6.75** | **9.55** | **6.10** |

---

## Decision Outcome

**Chosen Alternative:** **Option B (Modular Rust Domain Engine + Modern Web Studio Shell)**.

### Concrete Implementation Blueprint

1. **Rust Workspace (`crates/`):**
   - `dhap-core`: Core types (UUIDs, timestamps, error types, semantic statuses).
   - `dhap-project`: Project format specification (`.dhap`), JSON schema validation, manifest handling, atomic saves, schema migration, and zip path traversal protection.
   - `dhap-media`: Probe video files, extract keyframes at timestamp millisecond offsets via FFmpeg/synthetic adapters, scene-change detection heuristics.
   - `dhap-annotations`: Vector geometry (arrows, rectangles, callouts, text), non-destructive serialization, and pixel-level raster baking (blur, pixelate, solid opaque redaction mask).
   - `dhap-export`: Markdown exporter with assets, standalone HTML5 interactive guide with responsive design and light/dark modes, printable PDF layouts, preflight validation checklist.
   - `dhap-cli`: Command-line tool `dhap` offering `inspect`, `extract`, `validate`, `export`, and `batch` operations.

2. **Studio Frontend (`apps/studio`):**
   - Built with TypeScript 5.x (strict), React 19, and Vite.
   - Design system featuring warm creative aesthetics, 4/8pt spacing rhythm, accessible contrast, and full keyboard shortcut support.
   - 3 switchable workspace modes: **Guide Mode**, **Timeline Mode**, **Preview Mode**.
   - Interactive annotation canvas supporting drag-and-drop, bounding-box redaction, and callout placement.

3. **Public Product Website (`apps/site`):**
   - High-performance static showcase and documentation portal deployable to Cloudflare Pages.

4. **Licensing:**
   - Permissive **MIT License** for the entire Dhap repository.
   - Clear third-party attribution and notice compliance in `docs/THIRD_PARTY_NOTICES.md`.

---

## Consequences & Mitigations

- **Consequence:** Need to coordinate data transfer between Rust core and Studio UI.
  - **Mitigation:** Generate typed schemas or use clean JSON contracts between CLI/engine and frontend.
- **Consequence:** Native screen capture requires platform hooks.
  - **Mitigation:** In Phase 1, leverage FFmpeg and system recording capabilities with full video import support; integrate native capture adapters with clean interface boundaries.
