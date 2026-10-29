# Dhap (ধাপ) — Release Engineering & Automation

## 1. Release Philosophy

Dhap follows **Semantic Versioning 2.0.0** (`MAJOR.MINOR.PATCH`).

- **Automated & Reproducible:** Releases are triggered via annotated Git tags (`v*`) or manual GitHub Actions workflow dispatch.
- **Dry-Run Capability:** Every release workflow supports a dry-run execution mode that builds all platform binaries, executes test suites, and verifies checksums without publishing public releases or updating release tags.
- **Draft-First Release Model:** New releases are initially drafted on GitHub. Public publication occurs only after automated verification and manual smoke validation pass.
- **Transparency:** Every published release bundle includes SHA-256 checksums, a software bill of materials (SBOM), and an auto-generated changelog.

---

## 2. Release Pipeline Overview

```text
[Tag v0.1.0 or Manual Dispatch]
           │
           ▼
┌──────────────────────────┐
│  Validate Tag & Version  │
└──────────┬───────────────┘
           │
           ▼
┌────────────────────────────────────────────────────────┐
│  Multi-Platform Build Matrix (Windows, Linux, macOS)   │
│  - Compile Rust release binaries                       │
│  - Package Studio UI bundle                            │
│  - Execute unit, integration, and golden export tests  │
└──────────┬─────────────────────────────────────────────┘
           │
           ▼
┌──────────────────────────┐
│  Generate Artifacts:     │
│  - dhap CLI executable   │
│  - Tarballs / Zip bundles│
│  - SHA-256 Checksums     │
└──────────┬───────────────┘
           │
           ▼
┌──────────────────────────┐
│  Create Draft Release    │
│  - Upload assets         │
│  - Attach release notes  │
└──────────┬───────────────┘
           │
           ▼
┌──────────────────────────┐
│  Manual QA Approval      │
│  -> Publish Release      │
└──────────────────────────┘
```

---

## 3. Creating a Release

### Step 1: Pre-Release Verification
Run all automated checks locally:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

### Step 2: Triggering the Release Workflow
Create and push an annotated Git tag:
```bash
git tag -a v0.1.0 -m "Release v0.1.0: Initial MVP vertical slice"
git push origin v0.1.0
```

Alternatively, navigate to **Actions → Release Pipeline** in GitHub and trigger a manual workflow run with your chosen version and `dry_run: false`.

### Step 3: Review Draft Release
1. Inspect the newly generated draft release on GitHub.
2. Verify that binaries for Windows, Linux, and macOS are attached.
3. Verify that `SHA256SUMS.txt` accurately reflects the uploaded assets.
4. Promote the release from Draft to Public.
