**English** | [日本語版](../ja/CONTRIBUTING.md)

# Contributing Guidelines: CONTRIBUTING

Thank you for your interest in contributing to **MyVMSTAT**. Below are the guidelines for proposing changes, submitting pull requests, and setting up the development workspace.

---

## 1. Development Setup

Before writing code, ensure you have the standard Rust toolchain installed.

- **Rust Edition**: `Edition 2021` or later
- **Recommended Terminal**: Windows 11 (PowerShell 7) or Linux/macOS
- **Dependency Repository**:
  `MyVMSTAT` links relatively to a shared repository named `common_lib` (`../common_lib`). For local compilation, clone the `common_lib` repository into the same parent directory tree:

```text
workspace/
├── common_lib/  (https://github.com/tkshnkgwr/common_lib)
└── MyVMSTAT/    (This repository)
```

---

## 2. Commit Message Conventions

To keep version histories readable, adhere to the **Conventional Commits** standard:

```text
<type>: <short summary>
```

### Approved `<type>` Mappings:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Changes to documentation only
- `refactor:` Structural code edits that do not alter features
- `perf:` Performance enhancements
- `test:` Adding or repairing automated tests
- `chore:` Changes to build settings, dependency configurations, or CI files

---

## 3. Pull Request Procedures

1. **Branch Management**:
   Branch off from the `main` branch to create a feature branch (e.g. `feature/add-json-format`).
2. **Implementation and Verification**:
   Make code edits and execute the local verification tests listed in `TESTING.md` (e.g. `cargo test`, `cargo clippy`). Correct all linter alerts before continuing.
3. **Commit & Push**:
   Construct commit messages matching Conventional Commits syntax and push.
4. **Documentation Sync**:
   If your code changes modify features or specifications, ensure related documentations (including `CHANGELOG.md`) are updated and committed alongside the code as specified under `.agents/AGENTS.md`.
