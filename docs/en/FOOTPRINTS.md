**English** | [日本語版](../ja/FOOTPRINTS.md)

# Resource Footprint Records: FOOTPRINTS

This document records the measurement values of binary size, memory usage, and optimization profiles in the production release builds of **MyVMSTAT**.

---

## 1. Build Optimization Configuration (Cargo.toml)

To minimize the binary size and execution memory overhead, the following release compilation profile is applied:

```toml
[profile.release]
opt-level = 'z'      # Optimize for size
lto = true           # Link-Time Optimization
codegen-units = 1    # Consolidate into a single unit to maximize compiler optimization
panic = 'abort'      # Disable stack unwinding to reduce binary size
strip = true         # Completely strip symbol and debug information from the binary
```

---

## 2. Measurement Results

### 2.1 Environmental Specs
- **OS**: Windows 11 (64-bit)
- **Compiler**: rustc 1.96.0 / cargo
- **Dependencies**: `sysinfo v0.39`, `chrono v0.4`, `windows v0.62`

### 2.2 Telemetry Footprints

| Metric Item | Measured Value | Description |
| :--- | :--- | :--- |
| **Binary Size** | **206,336 Bytes (approx. 201 KB)** | Size of the standalone executable containing all statically linked dependencies |
| **Physical Memory (Working Set)** | **approx. 15.9 MB** | RSS (Resident Set Size) equivalent physical memory occupation while running |
| **Virtual Memory Size** | **approx. 440 MB** | Virtual address space size allocated by Windows 64-bit runtime startup environment |
| **CPU Usage** | **< 0.1%** | Zero CPU consumption except during the periodic 1-second interval telemetry calculations |

---

## 3. Analysis and Evaluation

- **Binary Size**: Despite linking heavy-weight system APIs like the `windows` and `sysinfo` crates, the application achieves a tiny binary footprint of 201 KB thanks to `strip = true` and `opt-level = 'z'`.
- **Memory Consumption**: By minimizing dynamic runtime allocations and reusing a single data buffer across sampling loops, memory usage remains extremely stable at 15.9 MB without any leaks. It is lightweight enough to run continuously on servers or containers as a daemon process.
