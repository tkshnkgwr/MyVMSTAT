**English** | [日本語版](../ja/TESTING.md)

# Verification and Testing Policy: TESTING

This document defines the testing principles, validation targets, and local verification procedures that developers must run before committing code or proposing releases in the **MyVMSTAT** project.

---

## 1. Testing Principles

To guarantee extreme reliability and resource efficiency, telemetry verification focuses on three scopes:
1. **Automated Unit Testing**: Verification of isolated functions, parse algorithms, and CLI arguments helper determinations.
2. **Static Quality Analysis**: Leveraging standard compilation checks and `clippy` linters to clean code smells.
3. **Manual Execution Checks**: Validating terminal-based ANSI color styling and CLI usage constraints under real interactive environments.

---

## 2. Local Verification Checklist

Upon modifying source code, developers should run the following commands sequentially to confirm the workspace remains free of errors or compilation warnings.

### 2.1 Run Unit Tests
Verifies internal functionalities and static assertions:
```bash
cargo test
```

### 2.2 Run Linter (Clippy)
Enforces strict style standards by treating lint warnings as compilation blockers:
```bash
cargo clippy --all-targets -- -D warnings
```

### 2.3 Verify Code Formatting
Checks if files adhere to standard Rust formatting guidelines:
```bash
cargo fmt --check
```
*Note: If formatting violations occur, run `cargo fmt` to apply clean corrections automatically.*

### 2.4 Verify Documentation Compilation (Rustdoc)
Validates comments and private references to prevent documentation warnings:
```bash
cargo doc --no-deps --document-private-items
```

---

## 3. Manual Inspection Steps

For cross-platform or console-level interface details, manually inspect the following capabilities:

### 3.1 Healthy Operations
- Verify the table prints row outputs at the interval specified by `delay`.
- Check that ANSI color highlights apply correctly based on metrics magnitudes (e.g. 0 as shaded gray, high cpu load as bold red/yellow).
- Ensure the process terminates cleanly (exit code `0`) after printing the number of iterations specified by `count`.

### 3.2 Error & Constraint Validations
- Verify invalid options trigger descriptive usage alerts (e.g. "Too many arguments.", "Invalid option") and abort with exit code `1`.
- **Single Instance Verification (Windows)**:
  Launch an instance of `MyVMSTAT` and keep it running. Open another console window and run it again. Verify the secondary execution aborts with the message: "Error: Another instance of MyVMSTAT is already running."
