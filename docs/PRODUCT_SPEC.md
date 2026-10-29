# Dhap (ধাপ) — Product Specification

**Working Tagline:** Make the steps clear.  
**Core Promise:** Record once. Create the video and guide together.  
**Document Status:** Approved Specification v1.0  
**Target Release:** v0.1.0 (MVP) → v1.0.0 (Public Production Release)

---

## 1. Executive Vision & Core Values

Dhap transforms screen recordings and visual workflows into editable, illustrated documentation and media outputs. It bridges the gap between video editing and technical writing:

1. **Local-First & Offline:** 100% of recording, frame extraction, step editing, redaction, and export generation occurs on-device without cloud dependencies.
2. **No Accounts or Subscriptions:** Free and open-source core with zero artificial feature walls, watermarks, or usage limits.
3. **Single Source of Truth:** A unified project format (`.dhap`) links the source media, extracted frames, timestamps, step narratives, annotations, and export configurations.
4. **Human-in-the-Loop:** All automated proposals (scene changes, candidate steps, AI suggestions) are presented as editable drafts. The user maintains final editorial control.
5. **Absolute Privacy & Irreversible Redaction:** Sensitive data is irreversibly baked out of exported assets, not merely masked with CSS or layers.

---

## 2. Requirements Matrix & Priority Tracking

### 2.1 Capture & Media Ingestion (CAP)

| ID | Priority | Description | Acceptance Criteria |
|:---|:---:|:---|:---|
| **CAP-001** | **P0** | Video File Ingestion | Accept MP4, WebM, MOV, and MKV files; parse duration, resolution, frame rate, and codec cleanly without crashing on corrupt inputs. |
| **CAP-002** | **P0** | Metadata & Frame Inspection | Extract video stream details and calculate exact millisecond timestamps. |
| **CAP-003** | **P0** | Non-Destructive Source Media Handling | Source media is treated as read-only/immutable; never overwritten or mutated during editing. |
| **CAP-004** | **P0** | Frame Extraction Pipeline | Extract representative PNG/WebP frames at arbitrary millisecond timestamps using optimized streaming ffmpeg calls. |
| **CAP-005** | **P0** | Scene-Change & Keyframe Heuristics | Analyze video intervals or scene changes to propose candidate step timestamps automatically. |
| **CAP-006** | **P1** | Integrated Screen Capture | Native desktop recording via platform media APIs or system capture hooks. |
| **CAP-007** | **P1** | Live Step Marker Hotkey | Ability to drop a timestamp marker in real-time during an active recording session. |
| **CAP-008** | **P2** | Browser & DOM Action Capture | Browser extension integration to record DOM events alongside video. |

---

### 2.2 Step & Guide Domain Model (GUIDE)

| ID | Priority | Description | Acceptance Criteria |
|:---|:---:|:---|:---|
| **GUIDE-001** | **P0** | Step CRUD Operations | Create, edit, reorder, and delete steps with instantaneous UI feedback and undo/redo support. |
| **GUIDE-002** | **P0** | Step Attributes | Each step possesses a Title, Imperative Instruction, Optional Expected Result, Notes, and Alt Text. |
| **GUIDE-003** | **P0** | Source Frame Association | Each step links to one or more extracted frames with explicit millisecond timestamps and source media references. |
| **GUIDE-004** | **P0** | Candidate Step Triage | Users can accept, reject, merge, or adjust suggested keyframe steps. |
| **GUIDE-005** | **P1** | Sections & Table of Contents | Group steps into logical chapters/sections and automatically generate navigation indices. |
| **GUIDE-006** | **P1** | Transcription & Audio Alignment | Align optional audio transcripts to step timestamps as contextual hints. |

---

### 2.3 Annotations & Privacy Redactions (ANNO)

| ID | Priority | Description | Acceptance Criteria |
|:---|:---:|:---|:---|
| **ANNO-001** | **P0** | Vector Annotations | Add numbered step badges, directional arrows, bounding boxes, circles, and text callouts to any step screenshot. |
| **ANNO-002** | **P0** | Privacy Redaction Masks | Apply solid opaque blackout masks, pixelation, or blur to selected bounding boxes. |
| **ANNO-003** | **P0** | Non-Destructive Project Representation | Annotation and redaction geometries are stored as clean vector metadata in `project.json` for perpetual re-editing. |
| **ANNO-004** | **P0** | Irreversible Export Baking | During export, redaction masks are permanently rendered onto image pixels; raw sensitive pixels are destroyed and cannot be recovered from the export. |
| **ANNO-005** | **P0** | Preflight Privacy Checklist | Pre-export review highlighting all redacted regions across all steps. |
| **ANNO-006** | **P1** | Pattern-Assisted Secret Detection | Heuristic/regex detection warning for API keys, email addresses, and auth tokens. |

---

### 2.4 Multi-Format Exports (EXP)

| ID | Priority | Description | Acceptance Criteria |
|:---|:---:|:---|:---|
| **EXP-001** | **P0** | Markdown Bundle Export | Export a clean GFM document with relative image links and structured headers in a dedicated folder. |
| **EXP-002** | **P0** | Standalone HTML Guide Export | Export a self-contained, responsive HTML5 guide featuring embedded or local assets, dark/light theme switching, and accessible typography. |
| **EXP-003** | **P0** | Preflight Export Validation | Verify that all referenced images exist, no required step titles are blank, and all outputs write atomically. |
| **EXP-004** | **P0** | Atomic Write & Rollback | Export writes to a temporary directory before replacing target folder; never leaves partial corrupted exports. |
| **EXP-005** | **P1** | Printable Clean PDF Export | Render a print-optimized document with page-break controls and high-resolution assets. |
| **EXP-006** | **P1** | Video / Animated GIF Export | Export video highlights or step clips with annotations overlaid. |
| **EXP-007** | **P1** | Reusable Export Templates | Custom styling themes for technical docs, internal SOPs, and public tutorials. |

---

### 2.5 Project Persistence & Integrity (PRJ)

| ID | Priority | Description | Acceptance Criteria |
|:---|:---:|:---|:---|
| **PRJ-001** | **P0** | Versioned `.dhap` Bundle | Portable ZIP container containing `manifest.json`, `project.json`, `frames/`, `media/`, and `annotations/`. |
| **PRJ-002** | **P0** | Path-Traversal Defense | Reject any bundle containing `..` or absolute paths in file entries; enforce strict sanitization on extraction. |
| **PRJ-003** | **P0** | Atomic Save & Autosave | Save to temporary `.tmp` file and perform atomic rename to prevent corruption on crash/power-loss. |
| **PRJ-004** | **P0** | Schema Versioning & Migrations | Support reading older schema versions with automatic migration and pre-migration backup creation. |
| **PRJ-005** | **P1** | Named Project Snapshots | Create local versioned snapshots for revision comparison and rollback. |

---

### 2.6 Automation, CLI & Tooling (AUTO)

| ID | Priority | Description | Acceptance Criteria |
|:---|:---:|:---|:---|
| **AUTO-001** | **P0** | First-Class `dhap` CLI | Standalone command-line binary supporting `inspect`, `extract`, `validate`, and `export`. |
| **AUTO-002** | **P0** | Stable Machine-Readable Output | CLI supports `--json` flag with structured schema output and explicit POSIX exit codes. |
| **AUTO-003** | **P1** | Headless Batch Processing | Convert folders of recordings into documentation drafts in automated pipelines. |
| **AUTO-004** | **P1** | Model Context Protocol (MCP) Server | Expose safe, explicit Dhap tool calls to local AI coding assistants and agent runtimes. |
