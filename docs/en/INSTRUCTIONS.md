**English** | [日本語版](../ja/INSTRUCTIONS.md)

# Coding Style Guidelines for Developers & AI: INSTRUCTIONS

This document prescribes the coding styles, error handling policies, component designs, and output formats that developers and AI assistants must adhere to when modifying source code or adding features in the **MyVMSTAT** project.

---

## 1. Naming Conventions

Adhere to Rust's standard naming conventions (RFC 430).

| Target | Casing | Example | Remarks |
| :--- | :--- | :--- | :--- |
| **Variables / Functions / Parameters** | `lower_snake_case` | `delay`, `parse_args`, `loop_count` | Select concise and clear names |
| **Structs / Enums / Traits** | `PascalCase` | `VmstatData`, `TelemetryProvider` | Acronyms must be all uppercase (e.g. `CliAction`) |
| **Constants / Macros** | `UPPER_SNAKE_CASE` | `CARGO_PKG_NAME`, `Local\MyVMSTAT-...` | Uppercase for better visibility |
| **Source File Names** | `lower_snake_case` | `main.rs` | Must match module names |
| **Documentation File Names** | `UPPER_SNAKE_CASE.md` | `ARCHITECTURE.md`, `TODO.md` | Unified rule specific to this project |

---

## 2. Error Handling Policy

Implement robust error handling to build a safe, crash-free CLI utility.

- **Avoid Panics**:
  - The use of `unwrap()` or `expect()` is prohibited except in test code or extremely specific contexts where success is guaranteed.
  - When parsing might fail (such as converting string to integer), apply a safe fallback value using `parse().unwrap_or(0)` or propagate a `Result` back to the caller.
- **Propagation via Result/Option**:
  - Errors caused by user input (such as parsing command line arguments) should be returned as `Result<T, String>` and processed in the `main` function.
  - When a fatal error occurs and continuation is impossible (such as failing to acquire the single-instance named mutex or parsing critical arguments), write the error to standard error (`eprintln!`) and terminate immediately with exit code `1` using `std::process::exit(1)`.

---

## 3. Component & Module Splitting Criteria

- **Single File Maintenance and 1,000 Lines Threshold**:
  - This tool has a simple responsibility: fetch system statistics, format, and print them row by row.
  - If the size of the single source file (`src/main.rs`) exceeds **1,000 lines**, the AI assistant/developer should refactor and propose modularizing the code (e.g. splitting into `src/provider.rs`, `src/cli.rs`, etc.) to maintain read clarity and ease of maintenance.
  - If the size is less than 1,000 lines, keep it as a single source file in `src/main.rs` to prevent over-complicating directory layout.
- **Decoupling to Common Shared Library (`common_lib`)**:
  - Reusable platform-specific logic that can be shared among multiple desktop/CLI applications (such as Windows Named Mutex single-instance lock) should be implemented in `common_lib` instead of inside this repository, and imported as a dependency.

---

## 4. AI Assistant Output Formats and Behaviors

When the AI assistant conducts tasks in this repository, it must strictly comply with the following instructions:

- **Minimal Explanations**:
  - When proposing code changes, skip verbose greetings and explanations. Focus on providing executable code blocks or `diff` blocks to keep responses concise.
- **Automatic Documentation**:
  - Whenever making changes to the source code, inspect and automatically update the corresponding markdown files (`CHANGELOG.md`, `SPEC.md`, `ARCHITECTURE.md`, `TODO.md`) before finishing the turn, in accordance with the guidelines in `.agents/AGENTS.md`.
- **Run Verification Tests**:
  - Run `cargo test` after modifying code to verify that all automated unit test cases pass.
- **Strict Relative Path Links**:
  - When linking documents inside the repository, never use absolute file paths (`file:///`). Always use relative paths (e.g., `docs/SPEC.md`) to maintain portability across environments.
