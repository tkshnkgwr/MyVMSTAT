# MyVMSTAT

A `dstat`-like colorized virtual memory statistics (`vmstat`) CLI utility written in Rust. It introduces highly requested features missing from standard `vmstat`, such as dynamic **Timestamps** and high-visibility color transitions.

🌐 **[日本語版のREADMEはこちら (README.ja.md)](README.ja.md)**

---

## 🎨 Terminal Color Legend & Thresholds

- **Grey Numbers (`0`)**: Indicates inactive or silent metrics to reduce visual noise (gray ANSI code `\x1b[90m`).
- **Yellow / Amber Highlights**: Warning states:
  - Free memory (`free`) dropping below 1.5GB (1536MB) (`\x1b[1;33m`).
  - CPU User space usage (`us`) > 40% (`\x1b[1;33m`).
  - CPU System space usage (`sy`) > 20% (`\x1b[1;33m`).
  - Context switch rate (`cs`) > 2000/s (`\x1b[1;33m`).
- **Red Bold Highlights**: High-criticality warnings:
  - Free memory (`free`) dropping below 512MB (`\x1b[1;31m`).
  - Active Swap usage (`swpd`) > 128MB (`\x1b[1;31m`).
  - CPU User space usage (`us`) > 80% (`\x1b[1;31m`).
  - CPU System space usage (`sy`) > 40% (`\x1b[1;31m`).
  - CPU I/O Wait (`wa`) > 15% (`\x1b[1;31m`).
- **Green Text**: Standard system activity telemetry (interrupts `in`, default context switches `cs`) (`\x1b[32m`).
- **Blue Timestamps**: Distinguishes temporal tracking from numeric machine statistics (`\x1b[34m`).

---

## 📁 Project Directory Layout

```text
.
├── Cargo.toml               # Cargo package configuration
├── LICENSE                  # MIT License
├── README.md                # English documentation (This file)
├── README.ja.md             # Japanese translation
├── src/
│   └── main.rs              # Main entry point (platform-specific implementations)
└── docs/                    # Architecture documents and reports
    ├── SPEC.md              # Technical specification & architectural design (Japanese)
    ├── DIAGRAM.md           # System architecture diagram (Mermaid)
    ├── FOOTPRINTS.md        # Binary footprint & performance statistics
    └── TEST_REPORT.md       # Validation logs & test cases
```

---

## ⚙️ Compilation & Running

Ensure you have the Rust toolchain installed.

### Command Line Usage
```bash
MyVMSTAT [delay [count]]
MyVMSTAT -h | --help
MyVMSTAT -v | --version
```
- `delay`: Interval in seconds (default: `1.0`).
- `count`: Maximum number of updates (default: infinite loop).
- `-h, --help`: Show usage help message, color thresholds, and exit.
- `-v, --version`: Show tool version and exit.

### Build and Run in Development Mode
```bash
cargo run -- [delay [count]]
# Or to show help:
cargo run -- -h
```

### Build for Production
To generate a highly optimized, stripped binary with minimal size:
```bash
cargo build --release
```
The optimized binary will be created at `target/release/MyVMSTAT.exe` (or `MyVMSTAT` on Unix).


---

## 🔒 Single Instance Lock (Windows Named Mutex)
On Windows platforms, `MyVMSTAT` prevents concurrent executions of the utility using a named OS mutex. If you try to run multiple instances concurrently, secondary executions will abort with:
```text
Error: Another instance of MyVMSTAT is already running.
```

---

## 📚 Technical Documentation (Japanese)
For further architectural blueprints and verification logs, check:
- **[System Specification](docs/SPEC.md)**
- **[System Diagram](docs/DIAGRAM.md)**
- **[Performance Footprints](docs/FOOTPRINTS.md)**
- **[Test Report](docs/TEST_REPORT.md)**
- **[Changelog](CHANGELOG.md)**
