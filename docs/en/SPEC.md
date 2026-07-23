**English** | [日本語版](../ja/SPEC.md)

# System Specification: MyVMSTAT

This document defines the internal design, operational algorithms, data parsing logic, and multi-platform architecture of **MyVMSTAT**, a colorized system resource monitoring CLI tool.

---

## 1. Architecture Overview

`MyVMSTAT` is a CLI utility that displays virtual memory statistics, process scheduling activity, CPU execution ratios, and local timestamps colorized on the terminal.

```text
       +---------------------------------------------------------+
       |                  MyVMSTAT CLI Application               |
       +----------------------------+----------------------------+
                                    |
            +-----------------------+-----------------------+
            | (Linux Platform)                              | (Cross Platform)
            v                                               v
    +-------------------+                           +-------------------+
    | /proc Filesystem  |                           | sysinfo API Layer |
    |   - /proc/stat    |                           |   - Global CPU %  |
    |   - /proc/meminfo |                           |   - Swap usage    |
    |                   |                           |   - Process list  |
    +-------------------+                           +-------------------+
```

---

## 2. Monitoring Metrics Column Specification

The output layout conforms to the standard Linux `vmstat` structure, integrated with `dstat`-like color coding and timestamps.

### Column Definitions and Data Types

| Category | Column Name | Data Type | Unit | Description |
| :--- | :--- | :--- | :--- | :--- |
| **procs** | `r` | Integer | count | Number of processes waiting for run time (Run queue) |
| | `b` | Integer | count | Number of processes in uninterruptible sleep (Blocked) |
| **memory**| `swpd` | Integer | MB | Amount of virtual memory used (Swap space) |
| | `free` | Integer | MB | Amount of idle physical memory |
| | `buff` | Integer | MB | Amount of memory used as buffers |
| | `cache`| Integer | MB | Amount of memory used as cache |
| **system**| `in` | Integer | count/s | The number of interrupts per second |
| | `cs` | Integer | count/s | The number of context switches per second |
| **cpu** | `us` | Integer | % | Time spent running non-kernel code (user + nice) |
| | `sy` | Integer | % | Time spent running kernel code (system + irq + softirq) |
| | `id` | Integer | % | Time spent idle (idle) |
| | `wa` | Integer | % | Time spent waiting for IO (iowait) |
| **time** | `timestamp`| String | - | Local absolute timestamp (`YYYY-MM-DD HH:MM:SS`) |

---

## 3. Linux Native Data Retrieval Specification

On Linux, to minimize system overhead, data is parsed directly from the `/proc` virtual filesystem without invoking external libraries or C library APIs.

### 3.1 CPU Usage Calculation (`/proc/stat`)
Read the first `cpu` line from `/proc/stat`:
```text
cpu  22538 310 11925 829532 2110 0 203 0 0 0
```
- **Fields parsed**: `user(1)`, `nice(2)`, `system(3)`, `idle(4)`, `iowait(5)`, `irq(6)`, `softirq(7)`, `steal(8)`.
- **Formulas**:
  $$\text{TotalTicks} = \text{user} + \text{nice} + \text{system} + \text{idle} + \text{iowait} + \text{irq} + \text{softirq} + \text{steal}$$
  $$\Delta\text{Total} = \text{TotalTicks}_{\text{new}} - \text{TotalTicks}_{\text{old}}$$
  $$\%us = \frac{\Delta(user + nice)}{\Delta\text{Total}} \times 100$$
  $$\%sy = \frac{\Delta(system + irq + softirq)}{\Delta\text{Total}} \times 100$$
  $$\%wa = \frac{\Delta(iowait)}{\Delta\text{Total}} \times 100$$
  $$\%id = \frac{\Delta(idle)}{\Delta\text{Total}} \times 100$$

### 3.2 Memory Parsing (`/proc/meminfo`)
Parse the following properties on each interval and convert from KB to MB (`value / 1024`):
- `MemFree` $\rightarrow$ `free` column
- `Buffers` $\rightarrow$ `buff` column
- `Cached` $\rightarrow$ `cache` column
- `SwapTotal` - `SwapFree` $\rightarrow$ `swpd` column

### 3.3 Interrupts and Context Switches (`/proc/stat`)
- Parse the first value on the `intr` line as the cumulative interrupt count.
- Parse the value on the `ctxt` line as the cumulative context switch count.
- Compute the rate per second as: $\frac{\text{Current Cumulative} - \text{Previous Cumulative}}{t}$ where $t$ is the interval in seconds.

---

## 4. Cross-Platform Specification (sysinfo Integration)

On non-Linux platforms (Windows, macOS, etc.), the `sysinfo` crate is used to acquire system metrics.

- **Process States (`r`, `b`)**: Count processes in the process list with state `ProcessStatus::Run` for `r`, and `ProcessStatus::UninterruptibleDiskSleep` for `b`.
- **Memory Information (`swpd`, `free`)**: Derived from `total_swap() - free_swap()` and `free_memory()`. Since detailed `buff`/`cache` splitting is limited on non-Linux systems, they are displayed as `0` (shaded grey).
- **System Information (`in`, `cs`)**: Rendered as `0` as they cannot be obtained via `sysinfo`.
- **CPU Usage**: Map the global CPU usage `cpu_usage()` to `us`, and render idle percentage as `100 - cpu_usage` in the `id` column. `sy` and `wa` are rendered as `0`.

---

## 5. Terminal Color Legend & Thresholds

To maximize readability, the following ANSI escape sequences (8-bit ANSI codes) are applied based on metric thresholds:

| Column | Condition | Formatting Style | ANSI Sequence |
| :--- | :--- | :--- | :--- |
| **All** | `value == 0` | Shaded Grey (De-emphasized) | `\x1b[90m` |
| **swpd** | `swpd > 128MB` | Bold Red (High Swap Activity) | `\x1b[1;31m` |
| **free** | `free < 512MB` | Bold Red (Low Memory Warning) | `\x1b[1;31m` |
| | `free < 1536MB` | Bold Yellow (Low Memory Caution) | `\x1b[1;33m` |
| **cs** | `cs > 2000` | Bold Yellow (High Context Switches) | `\x1b[1;33m` |
| | Otherwise and `value > 0` | Green (Normal Activity) | `\x1b[32m` |
| **in** | `value > 0` | Green (Normal Activity) | `\x1b[32m` |
| **us** | `us > 80%` | Bold Red (High CPU User Load) | `\x1b[1;31m` |
| | `us > 40%` | Bold Yellow (CPU User Caution) | `\x1b[1;33m` |
| **sy** | `sy > 40%` | Bold Red (High CPU System Load) | `\x1b[1;31m` |
| | `sy > 20%` | Bold Yellow (CPU System Caution) | `\x1b[1;33m` |
| **wa** | `wa > 15%` | Bold Red (I/O Bottleneck Warning) | `\x1b[1;31m` |
| **timestamp**| Always applied | Blue (Time tracking) | `\x1b[34m` |

---

## 6. Single Instance Lock (Windows Named Mutex)

On Windows, `MyVMSTAT` prevents multiple concurrent executions using an OS Named Mutex.

- **Mutex Name**: `Local\MyVMSTAT-single-instance-mutex`
- **Execution Flow**:
  1. Call `CreateMutexW` to create the named mutex.
  2. If creation fails or `GetLastError()` returns `ERROR_ALREADY_EXISTS`, another instance is running.
  3. Print an error message to `stderr` and abort with exit code `1`.
  4. If successful, keep the mutex handle open for the lifetime of the process (the OS recovers it automatically upon termination).

---

## 7. Command Line Arguments and Options Specification

The CLI parses arguments upon invocation:

### 7.1 Synopsis
```bash
MyVMSTAT [delay [count]]
MyVMSTAT -h | --help
MyVMSTAT -v | --version
```

### 7.2 Option Descriptions
- **`-h, --help`**:
  Prints usage instructions, command options, metric descriptions (Field Descriptions), and coloring thresholds (Color Legend) to `stdout` and exits cleanly.
- **`-v, --version`**:
  Prints `MyVMSTAT version [version]` to `stdout` and exits cleanly.
  - **Version Retrieval**: Retrieved dynamically at compile time from the `version` field in `Cargo.toml` using Rust's `env!("CARGO_PKG_VERSION")`.

### 7.3 Argument Validation
- If `delay` is not a positive number, prints usage help and exits with code `1`.
- If `count` is not a positive integer, prints usage help and exits with code `1`.
- If the number of arguments exceeds 2 (i.e. more than `delay` and `count`), prints "Too many arguments.", usage details, and exits with code `1`.
- If an unsupported option (e.g. `--invalid`) is supplied, prints "Invalid option", usage details, and exits with code `1`.

---

## 8. Cargo Features Specification

`MyVMSTAT` supports conditional compilation using Cargo Features to optimize binary footprints and target platform-specific functionalities:

| Feature Name | Dependencies | Description |
| :--- | :--- | :--- |
| `default` | `sysinfo`, `windows_desktop` | The standard configuration. Enables all features by default. |
| `sysinfo` | `sysinfo` | Enables the `SysinfoProvider` for non-Linux targets. If disabled, a dummy provider is compiled instead. |
| `windows_desktop` | `common_lib` (`windows_desktop` feature) | Enables Windows Named Mutex single-instance prevention. If disabled, single-instance verification is bypassed. |
