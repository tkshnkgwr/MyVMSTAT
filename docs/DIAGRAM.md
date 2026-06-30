# システム構成図: MyVMSTAT

本ドキュメントは、**MyVMSTAT** の制御フロー、スレッド、およびデータ取得経路について視覚的に定義します。

---

## 1. 全体制御フロー

アプリケーションの起動から終了までの制御フロー図です。

```mermaid
graph TD
    Start([プログラム開始]) --> InitLock{OS判別 & 二重起動防止}
    
    subgraph Windows処理
        InitLock -->|Windows| WinLock[Named Mutex 作成]
        WinLock --> WinLockCheck{既に起動中?}
        WinLockCheck -->|Yes| ExitError[エラー出力して終了]
        WinLockCheck -->|No| InitTelemetry[テレメトリプロバイダの初期化]
    end
    
    subgraph 他のOS処理
        InitLock -->|Linux / macOS / 他| InitTelemetry
    end
    
    InitTelemetry --> ParseArgs[引数解析: delay, count]
    ParseArgs --> ShowHeader[ヘッダー表示]
    ShowHeader --> LoopStart[サンプリングループ開始]
    
    LoopStart --> ReadTelemetry[テレメトリデータの読み込み]
    ReadTelemetry --> CalcDelta[差分レートの計算 cpu, cs, in]
    CalcDelta --> PrintRow[ANSIカラーでの一行出力]
    PrintRow --> CheckCount{指定回数に達したか?}
    
    CheckCount -->|Yes| End([プログラム正常終了])
    CheckCount -->|No| Sleep[delay秒間 スリープ待機]
    Sleep --> LoopStart
```

---

## 2. データ取得構造とプラットフォーム抽象化

プログラム内部でのプラットフォーム別のデータソース抽象化を表したクラス/トレイト関連図です。

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

    TelemetryProvider <|.. LinuxProvider : 実装 (target_os = "linux")
    TelemetryProvider <|.. SysinfoProvider : 実装 (その他のOS)
    LinuxProvider --> VmstatData : 生成
    SysinfoProvider --> VmstatData : 生成
```

---

## 3. スレッド・待機サイクル

本ツールはメインスレッド単一で動作します。イベント駆動ではなく、周期的なブロッキング待機（`thread::sleep`）によるポーリング制御を行います。

```mermaid
sequenceDiagram
    autonumber
    actor OS as オペレーティングシステム
    participant App as MyVMSTAT (メインスレッド)
    
    App->>OS: ミューテックス作成要求 (Windowsのみ)
    OS-->>App: ハンドル返却 (多重起動時は即座に終了)
    
    Note over App: 初回データ取得 (ウォームアップ)
    App->>OS: /proc 読込 または sysinfo 取得
    OS-->>App: システム情報
    
    App->>App: 0.2秒待機 (初期CPUチック安定化のため)
    
    loop サンプリングサイクル (delay秒毎)
        App->>OS: 最新テレメトリデータの取得
        OS-->>App: 生データ
        App->>App: 前回値との差分算出 (cpu_us, cpu_sy, cs 等)
        App->>App: ANSIエスケープコード適用とフォーマット
        App->>App: 標準出力 (stdout) へ書き込み
        App->>OS: 指定時間スリープ (thread::sleep)
        OS-->>App: スリープ復帰
    end
```
