# Dhap Licensing Policy & Provenance

**Last updated:** 29 October 2026

## 1. Project License

Dhap (ধাপ) is licensed under the **MIT License**. You are free to use, copy, modify, merge, publish, distribute, sublicense, and sell copies of the software for personal, academic, or commercial purposes, subject to the conditions of the MIT License.

## 2. Permitted Dependency Licenses

All direct dependencies integrated into the Dhap codebase must conform to permissive open-source licenses:
- **MIT License**
- **Apache License 2.0**
- **BSD 2-Clause and 3-Clause Licenses**
- **ISC License**
- **CC0-1.0** / Public Domain

Copyleft licenses (such as GNU GPL or AGPL) are prohibited from being statically or dynamically linked into Dhap binaries to ensure that Dhap remains unencumbered and commercially redistributable as permissive open-source software.

## 3. FFmpeg and Media Tooling Isolation

Dhap utilizes FFmpeg for video decoding, metadata inspection, and frame extraction:
- **Execution Boundary:** FFmpeg is invoked strictly as an external subprocess (via CLI execution) or through decoupled system processes.
- **No Static/Dynamic Linking:** Dhap's compiled Rust binaries do not link against `libavcodec`, `libavformat`, or other FFmpeg libraries.
- **Redistribution:** When bundling FFmpeg binaries with desktop distribution packages, all licensing obligations under the GNU LGPL v2.1+ or GNU GPL v2.0+ (depending on the build flags of the bundled FFmpeg binary) are strictly observed, and corresponding source-code access and licenses are provided in `docs/THIRD_PARTY_NOTICES.md`.

## 4. Upstream Attribution & Trademarks

- **OpenScreen Attribution:** Dhap draws inspiration and architecture concepts from OpenScreen (MIT License). Dhap does not claim affiliation with or endorsement by the OpenScreen maintainers.
- **Original Branding:** The name "Dhap", Bengali script "ধাপ", logo designs, and application artwork are original creations of NH Prince Pradhan and the Dhap project.

See [docs/THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for full notices and license texts of upstream and bundled components.
