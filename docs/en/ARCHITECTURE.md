**English** | [日本語版](../ja/ARCHITECTURE.md)

# System Architecture Document: MyVMSTAT

This document defines the overall design, tech stack, directory layout rationale, and inter-module data flows of **MyVMSTAT**.

---

## 1. System Overview and Goals

`MyVMSTAT` is a colorized command line utility (CLI) for real-time system resource monitoring, heavily inspired by the standard Linux `vmstat` command and `dstat`.

### Goals
- **Multi-Platform Support**: Enable `vmstat`-like resource monitoring not only on Linux but also on non-Linux platforms like Windows.
- **High Visibility (dstat-like)**: Colorize terminal outputs using ANSI escape sequences based on the value magnitudes and state criticality (inactive/active/caution/critical) to let users scan resource consumption instantly.
- **Ultra-lightweight & Resource-efficient**: Optimized for minimal memory usage and binary footprint, suitable for background executions or long-running daemon terminal windows.

---

## 2. Technology Stack

The project is implemented in Rust, offering a memory-safe, high-performance, and single-binary deployment.

| Component | Technology | Version | Main Usage / Role |
| :--- | :--- | :--- | :--- |
| **Language** | Rust | Edition 2021 | Core programming language, static type safety |
| **System Info** | `sysinfo` | v0.39 | Cross-platform resource telemetry (Optional: `feature = "sysinfo"`) |
| **Date/Time** | `chrono` | v0.4 | Parsing local timestamps and rendering formatted output strings |
| **Common Lib** | `common_lib` | Custom | Desktop functionalities for Windows (Named Mutex for single-instance lock) (Optional: `feature = "windows_desktop"`) |

---

## 3. Architecture and Directory Layout Rationale

### 3.1 Directory Structure
The repository layout is organized as follows:

```text
MyVMSTAT/
├── .agents/
│   └── AGENTS.md          # AI Assistant guidelines and instructions
├── .github/
│   └── workflows/         # GitHub Actions (CI/CD, auto release)
├── docs/
│   ├── ARCHITECTURE.md    # System Architecture design document (This file)
│   ├── DIAGRAM.md         # Mermaid system architecture and control flow diagrams
│   ├── FOOTPRINTS.md      # Binary size and memory footprint logs
│   ├── INSTRUCTIONS.md    # AI/developer coding style guidelines
│   ├── SPEC.md            # Detailed technical specification
│   ├── TEST_REPORT.md     # Automated and manual verification reports
│   └── TODO.md            # Roadmap, completed tasks, and backlog
├── scripts/               # Helper utilities for automation and versioning
├── src/
│   └── main.rs            # Core application source code
├── Cargo.toml             # Cargo package configuration and release profiles
└── Cargo.lock             # Dependency lockfile
```

### 3.2 Design Rationale
- **High Cohesion in a Single Source File (`src/main.rs`)**:
  Since the size of the program is highly compact (less than 1,000 lines), we intentionally avoid the overhead of module decomposition. Gathering all code inside `main.rs` maximizes compile speed and structural clarity.
- **Platform Abstraction**:
  Platform-dependent code paths are split using conditional compilation (`#[cfg(...)]`). In Linux environments, `/proc` is parsed directly for zero-overhead metrics. In non-Linux environments, the `sysinfo` crate is invoked.
- **Independent Document Management**:
  Specifications, architectural notes, and test reports are decoupled from code. Maintaining them in uppercase snake-case files under `docs/` enhances portability and read experience.

---

## 4. Data Flow and Module Interactions

The application runs on a single main thread, conducting initialization, arguments parsing, and a polling loop.

### 4.1 Provider Abstraction and Factory Pattern
System data acquisition is abstracted via the `TelemetryProvider` trait, which yields concrete providers depending on the platform target.

```text
                       +-----------------------------+
                       |      TelemetryProvider      | <--- Trait Definition
                       +--------------+--------------+
                                      |
               +----------------------+----------------------+
               | (Linux Targets)                             | (Non-Linux Targets)
               v                                             v
   +-----------------------+                     +-----------------------+
   |     LinuxProvider     |                     |    SysinfoProvider    |
   |  - Parses /proc/stat  |                     |  - sysinfo::System    |
   |  - /proc/meminfo      |                     +-----------------------+
   +-----------------------+
```

1. **Factory Function `get_provider()`**:
   - Compiling for Linux (`#[cfg(target_os = "linux")]`): Returns a boxed `LinuxProvider`.
   - Compiling for non-Linux with sysinfo (`#[cfg(all(not(target_os = "linux"), feature = "sysinfo"))]`): Returns a boxed `SysinfoProvider`.
   - Compiling without sysinfo/Linux (`#[cfg(all(not(target_os = "linux"), not(feature = "sysinfo")))]`): Returns a boxed `DummyProvider` (returns default empty values).

### 4.2 Data Flow & Sampling Loop
The main loop runs at interval of `delay` seconds, processing data flows in the following sequence:

```text
 [Start]
   │
   ▼
 [Single Instance Lock] ── (Windows: Abort if mutex is already held by another process)
   │
   ▼
 [Parse Arguments (parse_args)] ── (Extract delay and count parameters)
   │
   ▼
 [Warmup Telemetry] ── (Initialize provider and retrieve baseline CPU ticks)
   │
   ▼
 [Print Headers]
   │
   ├──◄────────────────────────────────────────────────────+
   ▼                                                       │
 [Loop Start]                                              │
   │                                                       │
   ├──► [TelemetryProvider::get_delta()]                   │
   │    Compute delta between intervals and convert to rate │
   │                                                       │
   ├──► [print_row()]                                      │
   │    Apply ANSI color thresholds and write row to stdout │
   │                                                       │
   ├──► [Check Exit Condition]                             │
   │    Break loop if iteration count matches requested `count`
   │                                                       │
   ▼                                                       │
 [thread::sleep(delay)] ───────────────────────────────────+
   │
   ▼
  [End]
```

- **Importance of Delta Rate Calculations**:
  Metrics like context switches (`cs`), interrupts (`in`), and CPU ticks are stored by the OS as cumulative values since the machine booted. To compute the accurate instant rate, `MyVMSTAT` records the previous interval's baseline, calculates the delta, and divides it by the elapsed seconds.
