# Security Policy & Architecture

## 1. Supported Versions

Security updates are applied to the active minor versions:

| Version | Supported |
|:---|:---:|
| 0.1.x | Yes |
| < 0.1.0 | No |

---

## 2. Reporting a Vulnerability

If you discover a security vulnerability in Dhap, please report it privately rather than opening a public issue.

- **Email:** `security@dhap.dev` (or via GitHub Private Vulnerability Reporting on `nhprince/dhap-studio`).
- **Response Target:** Acknowledgement within 48 hours; resolution plan within 7 days.
- **Coordination:** We coordinate responsible disclosure dates once a fix is prepared and validated.

---

## 3. Core Security Principles

### 3.1 Air-Gapped Local-First Architecture
Dhap's core recording, frame extraction, step editing, redaction, and documentation export occur entirely locally. Dhap does not transmit project data, video content, screenshots, or telemetry to external servers by default.

### 3.2 Path Traversal Defense
When extracting `.dhap` project archives or exporting documentation:
- All file paths are strictly validated against directory traversal attacks (`..`).
- Absolute paths in archive headers are rejected.
- File writes are strictly confined within the designated project sandbox.

### 3.3 Irreversible Redaction Assurance
Screen recordings often contain sensitive information (API tokens, email addresses, passwords).
- **Destructive Rasterization:** When redaction masks (solid blackouts, blur, pixelate) are applied, Dhap modifies the pixel values in memory before writing to disk.
- **No Residual Assets:** The export engine verifies that unredacted source images are never bundled into the export folder when safe export is selected.

### 3.4 Subprocess Execution Security
All external tool invocations (such as FFmpeg) use strictly parameterized argument vectors (`std::process::Command`), preventing command injection or shell interpolation vulnerabilities.
