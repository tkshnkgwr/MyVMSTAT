**English** | [日本語版](../ja/DIAGRAM.md)

# System Diagrams: MyVMSTAT

This document visually defines the control flow, thread lifecycle, and data retrieval paths of **MyVMSTAT**.

---

## 1. Overall Control Flow

The diagram below outlines the runtime execution flow from startup to clean exit.

```mermaid
graph TD
    Start([Program Start]) --> InitLock{OS Check & Single Instance Prevention}
    
    subgraph Windows Processing
        InitLock -->|Windows| WinLock[Create Named Mutex]
        WinLock --> WinLockCheck{Already Running?}
        WinLockCheck -->|Yes| ExitError[Print Error & Abort]
        WinLockCheck -->|No| InitTelemetry[Initialize Telemetry Provider]
    end
    
    subgraph Non-Windows OS Processing
        InitLock -->|Linux / macOS / Other| InitTelemetry
    end
    
    InitTelemetry --> ParseArgs[Parse Args: delay, count]
    ParseArgs --> ShowHeader[Print Headers]
    ShowHeader --> LoopStart[Start Sampling Loop]
    
    LoopStart --> ReadTelemetry[Retrieve Telemetry Data]
    ReadTelemetry --> CalcDelta[Calculate Delta Rates: cpu, cs, in]
    CalcDelta --> PrintRow[Print Row with ANSI Colors]
    PrintRow --> CheckCount{Iteration Count Reached?}
    
    CheckCount -->|Yes| End([Clean Termination])
    CheckCount -->|No| Sleep[Sleep for delay seconds]
    Sleep --> LoopStart
```

---

## 2. Telemetry Retrieval Structure and Platform Abstraction

This class diagram represents how the different telemetry data sources are abstracted based on the compilation target.

```mermaid
classDiagram
    class TelemetryProvider {
        <<interface>>
        +get_data() VmstatData
        +get_delta(current, duration) VmstatData
    }
    
    class LinuxProvider {
        -prev_ticks: CpuTicks
        -prev_intr: u64
        -prev_ctxt: u64
        +get_data() VmstatData
        +get_delta() VmstatData
    }
    
    class SysinfoProvider {
        -sys: sysinfo::System
        +get_data() VmstatData
        +get_delta() VmstatData
    }
    
    class VmstatData {
        +r: u64
        +b: u64
        +swpd: u64
        +free: u64
        +buff: u64
        +cache: u64
        +intr: u64
        +ctxt: u64
        +cpu_us: f64
        +cpu_sy: f64
        +cpu_id: f64
        +cpu_wa: f64
    }

    TelemetryProvider <|.. LinuxProvider : Implements (target_os = "linux")
    TelemetryProvider <|.. SysinfoProvider : Implements (Other OS)
    LinuxProvider --> VmstatData : Instantiates
    SysinfoProvider --> VmstatData : Instantiates
```

---

## 3. Thread and Waiting Lifecycle

The application operates entirely on a single thread. Instead of being event-driven, it manages polling loops with periodic blocking waits using `thread::sleep`.

```mermaid
sequenceDiagram
    autonumber
    actor OS as Operating System
    participant App as MyVMSTAT (Main Thread)
    
    App->>OS: Mutex Creation Request (Windows Only)
    OS-->>App: Return Mutex Handle (Abort if already running)
    
    Note over App: Initial Data Fetch (Warmup)
    App->>OS: Read /proc or query sysinfo
    OS-->>App: System Telemetry
    
    App->>App: Wait 0.2s (Stabilize initial CPU ticks)
    
    loop Sampling Cycle (Every delay seconds)
        App->>OS: Query current telemetry
        OS-->>App: Raw Data
        App->>App: Compute delta from previous values (cpu_us, cpu_sy, cs, etc.)
        App->>App: Format row and apply ANSI codes
        App->>App: Write to standard output
        App->>OS: Block thread (thread::sleep)
        OS-->>App: Sleep Wakeup
    end
```
