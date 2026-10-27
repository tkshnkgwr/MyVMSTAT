**English** | [日本語版](../ja/CHANGELOG.md)

# Changelog: MyVMSTAT

All notable changes to this project are documented in this file. This project adheres to Semantic Versioning.

---

## [1.3.5] - 2026-10-27

### Fixed
- **CI/CD Release Workflow Fix**:
  - Fixed Windows artifact packaging file path in `release.yml` from obsolete `README.ja.md` to `README_JA.md`.

---

## [1.3.4] - 2026-10-27

### Added
- **Agent-Specific Skills Integration**:
  - Introduced `.agents/skills/` containing `rust-verification`, `git-commit`, `markdown-table`, and `vault-memo`.
- **Enhanced Version Display**:
  - Improved `-v` / `--version` CLI output to display Copyright notice (`Copyright (c) 2026 tkshnkgwr`) and License (`MIT`) alongside package version.

### Changed
- **Unified Absolute Development Rules**:
  - Established `RULES.md` and `GEMINI.md` defining strict guidelines for auto-commit bans, table vertical alignment, `%USERPROFILE%` variable usage, and high-performance CLI tools (`rg`/`fd`/`jq`).
  - Synchronized repository layout and document indices across `README.md`, `README_JA.md`, and `ARCHITECTURE.md`.

---

## [1.3.2] - 2026-07-23

### Added
- **Multilingual Documentation & Operations Manuals**:
  - Split technical documentation into multilingual structures (`docs/en/` and `docs/ja/`).
  - Added operational documents: `TESTING.md`, `RELEASE.md`, `CONTRIBUTING.md`, and `SECURITY.md`.

---

## [1.3.1] - 2026-07-21

### Added
- **Decoupled Dependencies with Cargo Features**:
  - Added `[features]` section in `Cargo.toml` introducing `sysinfo` and `windows_desktop` feature flags.
  - Made `sysinfo` and `common_lib` optional dependencies.
  - Maintained backward compatibility via `default = ["sysinfo", "windows_desktop"]`.

---

## [1.3.0] - 2026-07-21

### Added
- **Cargo Features Integration for Decoupling Dependencies**:
  - Introduced the `[features]` section in `Cargo.toml` with `sysinfo` and `windows_desktop` flags.
  - Converted `sysinfo` and `common_lib` into optional dependencies (`optional = true`).
  - Maintained backward compatibility by default (`default = ["sysinfo", "windows_desktop"]`) while allowing slim compilation profiles.
- **1,000 Lines Refactoring Rule**:
  - Added a guideline to `.agents/AGENTS.md` and `docs/INSTRUCTIONS.md` to modularize/refactor `src/main.rs` once it exceeds 1,000 lines.

---

## [1.2.14] - 2026-07-16

### Added
- **Architecture Design Document (`docs/ARCHITECTURE.md`)**:
  - Added system descriptions, tech stack details, directory architecture details, and inter-module data flows.
- **Coding Style Guideline (`docs/INSTRUCTIONS.md`)**:
  - Defined naming conventions, panic avoidance strategies, module boundaries, and formatting instructions.
- **Roadmap Management (`docs/TODO.md`)**:
  - Introduced progress tracker for completed items, near-term tasks, and future backlog items.

### Changed
- **UPPER_SNAKE_CASE Document Naming**:
  - Renamed document files to UPPER_SNAKE_CASE to ensure consistency.
- **AI Agent Guidelines Refactoring**:
  - Integrated the UPPER_SNAKE_CASE rule into `.agents/AGENTS.md`.

---

## [1.2.13] - 2026-07-14

### Added
- **RustDoc Comments**:
  - Added comprehensive RustDoc comments (`//!`, `///`) for structs (`VmstatData`), traits (`TelemetryProvider`), concrete providers (`LinuxProvider`, `SysinfoProvider`), and utility functions.

### Changed
- **Agent Instruction Updates**:
  - Enforced RustDoc comment maintenance on future modifications under `.agents/AGENTS.md`.

---

## [1.2.10] - 2026-07-06

### Changed
- **CI/CD Workflow and Release Automation**:
  - Aligned automatic versioning and release workflows with `MiSysMon` design.
  - Replaced ad-hoc tagging steps inside `ci.yml` with separate workflows.
  - Created `.github/workflows/bump-version.yml` to automatically increment patch versions and push git tags on commits.
  - Added version bump PowerShell script (`scripts/bump-version.ps1`) to replace versions in `Cargo.toml` and documentation.
  - Upgraded action versions to `actions/checkout@v4` and `softprops/action-gh-release@v2`.
  - Configured workflows to check out the dependent `common_lib` repository relatively.
  - Added `defaults.run.working-directory: MyVMSTAT` globally in YAML workflows.
  - Fixed asset compression paths and Linux packaging issues.
