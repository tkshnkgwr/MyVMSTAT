**English** | [日本語版](../ja/TODO.md)

# Development Roadmap and Task Management: TODO

This document tracks the current implementation status, near-term tasks, and future feature proposals (backlog) of **MyVMSTAT**.

---

## 1. Completed Features (Done)

A list of features already implemented and verified in this project.

### 1.1 Core Telemetry Monitoring
- **Linux Native Telemetry**:
  - Parses `/proc/stat` and `/proc/meminfo` directly without invoking external runtime wrappers, retrieving CPU ticks (user/system/idle/iowait), memory status (free/buff/cache/swpd), context switch counts, and interrupt rates with minimal overhead.
- **Cross-Platform (Non-Linux/Windows) Support**:
  - Automatically falls back to telemetry via the `sysinfo` crate. Captures global CPU usage, physical memory size, and scans active process states.
- **dstat-like Visual Formatting (ANSI Color Codes)**:
  - Dulls inactive metric values (0) to gray, bolds warnings and limit states in yellow and red, and distinguishes timestamps in blue.

### 1.2 CLI and Process Controls
- **Robust Argument Parsing**:
  - Supports custom interval intervals (`delay` in seconds, including decimals) and sampling thresholds (`count`).
  - Supports CLI flags for help (`-h`, `--help`) and version (`-v`, `--version`).
  - Safe error recovery for out-of-range parameters, invalid flag options, or excessive arguments.
- **Single Instance Lock (Windows Named Mutex)**:
  - Restricts execution to a single instance on Windows via a named mutex provided by `common_lib`, safely aborting duplicates.
- **Cargo Features Separation**:
  - Optional dependency separation for `sysinfo` and `windows_desktop` features to reduce binary footprints.

### 1.3 CI/CD & Development Environment
- **Release Automation via GitHub Actions**:
  - Triggers automatic version bumps and tag pushes on main branch push.
  - Cross-compiles standalone release assets for Windows and Linux, publishing them directly to GitHub Releases.
- **Optimized Release Compilation Profiles**:
  - Truncates binary footprints to approximately 201KB by configuring sizes optimization (`opt-level = 'z'`, `strip = true`, `lto = true`).

---

## 2. Near-Term Tasks (In Progress / Todo)

Actionable tasks scheduled for development based on current telemetry limits.

### 2.1 Feature Enhancements & Metric Mappings
- `[ ]` **Expand Telemetry on Windows Platforms**:
  - **Current status**: `cs`, `in`, `buff`, `cache`, `sy`, and `wa` are fixed to `0` (shaded gray) on non-Linux hosts.
  - **Proposed action**: Investigate/implement direct Windows API calls (such as performance counters or native system telemetry APIs) or lightweight libraries to acquire system cache, context switch rates, and interrupt rates.
- `[ ]` **Fallback Verification on Stripped Linux Hosts (e.g. Containers)**:
  - **Current status**: Behavior in minimal Linux containers where `/proc` is sandboxed or partially unmounted remains unverified.
  - **Proposed action**: Improve robustness by falling back to `sysinfo` when `/proc` is unreachable or throws access errors.

### 2.2 Testing & Quality Assurance
- `[ ]` **Verify pwsh Encoding on Windows**:
  - Further test ANSI escape styling and encoding issues under Windows terminals, specifically PowerShell 5.1 vs 7.0 in Japanese environments.

---

## 3. Long-term Backlog

Proposals to enhance utility over the long term.

- **Alternative Format Outputs (`--format`)**:
  - Support exporting structured telemetry rows as JSON or CSV, facilitating integrations with external scripts or monitoring agents like Datadog.
- **Telemetry File Logging (`--output <file_path>`)**:
  - Enable continuous telemetry logging to a text file while printing formatting outputs to the terminal simultaneously.
- **Interactive Keyboard Controls**:
  - Detect runtime keystrokes, e.g. redrawing category headers on `Space` or cleanly exiting the loop on `q`.
