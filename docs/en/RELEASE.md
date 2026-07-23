**English** | [日本語版](../ja/RELEASE.md)

# Release Procedures: RELEASE

This document defines the process for updating versions and publishing releases for **MyVMSTAT**.

---

## 1. Automated Release Flow Overview

This project implements continuous integration and automated release workflows similar to the `MiSysMon` design via GitHub Actions.

- **Automated Version Bumping**:
  Whenever changes are pushed or merged into the `main` branch, the automated versioning workflow (`.github/workflows/bump-version.yml`) triggers, incrementing the patch version automatically (e.g. `1.3.0` $\rightarrow$ `1.3.1`).
- **Tagging and Compilation**:
  The workflow pushes a new git tag matching the bumped version, which subsequently triggers the release pipeline (`.github/workflows/release.yml`) to compile, package, and upload binaries directly to GitHub Releases.

---

## 2. Version Synchronization Script

To prevent manual version mismatch errors across documentation and package manifests, the PowerShell script `scripts/bump-version.ps1` runs inside the pipeline.

This script scans and replaces version strings in the following files:
1. The `version` field under **`Cargo.toml`**.
2. Version occurrences inside **`docs/ja/SPEC.md`** and **`docs/en/SPEC.md`**.

---

## 3. Manual Releases (Ad-hoc)

If you need to bypass automated patch versions and manually release a major or minor version (e.g. `v1.4.0`), execute the following steps:

1. **Local Telemetry Verification**:
   Execute the verification checks documented in `TESTING.md` to confirm there are no compile warnings or test failures.
2. **Update Package Manifest**:
   Manually set the new version string in `Cargo.toml` and commit the change.
3. **Push New Tag**:
   ```bash
   git tag v1.4.0
   git push origin v1.4.0
   ```
4. **Inspect Pipeline**:
   The tag push event initiates `.github/workflows/release.yml`. Standalone assets (`.zip` / `.tar.gz`) for Windows and Linux will be built and appended to the GitHub Release draft page.
