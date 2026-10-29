# Contributing to Dhap (ধাপ)

Thank you for your interest in contributing to Dhap! We are building an open-source, local-first studio for converting screen recordings into editable, polished documentation.

---

## 1. Development Philosophy

1. **User Experience First:** Software should feel fast, thoughtful, and dependable.
2. **Deterministic & Test-Driven:** Every core behavior, format serialization, and export generation must be backed by unit or integration tests.
3. **Local-First & Privacy Conscious:** Never add telemetry or background network calls without explicit user consent.
4. **Permissive Licensing:** Ensure all dependencies use permissive licenses (MIT/Apache-2.0).

---

## 2. Commit Message Conventions

We use standard Conventional Commits:

- `feat:` New user-facing or programmatic features.
- `fix:` Bug fixes.
- `docs:` Documentation additions or revisions.
- `test:` Adding or refining automated tests.
- `ci:` Modifying GitHub Actions workflows or automation scripts.
- `refactor:` Code improvements without behavioral changes.
- `build:` Dependency or toolchain updates.

---

## 3. Pull Request Checklist

Before submitting a pull request, ensure:
1. `cargo fmt --all -- --check` passes cleanly.
2. `cargo clippy --workspace --all-targets --all-features -- -D warnings` reports zero warnings.
3. `cargo test --workspace` passes with 100% success.
4. If frontend code was modified, `npm run lint` and `npm run typecheck` in `apps/studio` pass.
5. Relevant documentation under `docs/` has been updated to reflect your changes.
