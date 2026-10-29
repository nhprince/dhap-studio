# Dhap (ধাপ) — Product Roadmap

**Tagline:** Make the steps clear.  
**Version Horizon:** v0.1.0 (MVP) through v1.0.0 (Production Launch)

---

## Phase 1: MVP Vertical Slice (Current Milestone — v0.1.0)

**Goal:** Establish a robust, reproducible core workflow that validates the promise: "Record once. Create the video and guide together."

- [x] **Repository & CI Foundation:**
  - [x] Monorepo workspace initialized with clean Git hygiene.
  - [x] GitHub repository configured (`nhprince/dhap-studio`).
  - [x] Pinned GitHub Actions workflows for PR checks, multi-platform matrix, and releases.
  - [x] Zero-dependency license compliance (MIT).
- [ ] **Core Domain Crates:**
  - [ ] `dhap-core`: Strongly typed identifiers, millisecond timestamps, contextual error hierarchy.
  - [ ] `dhap-project`: Schema specification v1, atomic project file IO, ZIP path traversal prevention.
  - [ ] `dhap-media`: Non-destructive video probing, streaming frame extraction via FFmpeg, scene heuristics.
  - [ ] `dhap-annotations`: Vector geometry primitives, non-destructive layer state, and irreversible pixel-level redaction baking.
  - [ ] `dhap-export`: Markdown bundle generation, responsive standalone HTML guide exporter, preflight validation.
  - [ ] `dhap-cli`: Command-line interface with machine-readable JSON output and deterministic exit codes.
- [ ] **Studio Desktop Experience:**
  - [ ] Initializing Dhap Studio UI (React 19 + TypeScript).
  - [ ] Guide layout, timeline view, and real-time export preview.
  - [ ] Canvas-based annotation and redaction tool.
- [ ] **Quality Assurance:**
  - [ ] Unit and integration test coverage across all crates.
  - [ ] Golden export fixtures for Markdown and HTML.
  - [ ] Fault injection tests for malformed projects and corrupt media.

---

## Phase 2: Enhanced Capture & Creator Capabilities (v0.2.0 – v0.5.0)

**Goal:** Deepen recording capabilities and creator customization.

- [ ] Native screen and window capture integration via OS-level APIs.
- [ ] Live timestamp marking hotkey during active recording sessions.
- [ ] PDF document export with printable CSS and custom page breaks.
- [ ] Multiple documentation templates (Technical SOP, API Walkthrough, Education Handout).
- [ ] Project snapshots and revision comparison.
- [ ] Batch processing pipeline for automated folder conversion.

---

## Phase 3: Intelligent Assistance & Integrations (v0.6.0 – v0.9.0)

**Goal:** Optional, privacy-preserving AI assistance and automation hooks.

- [ ] Local-first transcription engine (Whisper.cpp integration).
- [ ] Provider-agnostic AI adapter for step title and description refinement (Bring Your Own Key: OpenAI, Gemini, Anthropic, or Ollama).
- [ ] Preflight privacy inspector: Explicit consent modal detailing exactly which text or frames leave the machine.
- [ ] Model Context Protocol (MCP) server for local AI assistant orchestration.
- [ ] Optical Character Recognition (OCR) for search and auto-detecting sensitive fields (tokens, email addresses).

---

## Phase 4: Production Release v1.0.0

**Goal:** Comprehensive general availability release across all target operating systems.

- [ ] Code-signed desktop installers for Windows (`.msi`, `.exe`), macOS (`.dmg`), and Linux (`.AppImage`, `.deb`).
- [ ] Production showcase website on Cloudflare Pages.
- [ ] Interactive online guide viewer.
- [ ] Comprehensive documentation and tutorials.
