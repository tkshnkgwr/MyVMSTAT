# MyVMSTAT

[![GitHub Release](https://img.shields.io/github/v/release/tkshnkgwr/MyVMSTAT)](https://github.com/tkshnkgwr/MyVMSTAT/releases)
[![CI Build](https://github.com/tkshnkgwr/MyVMSTAT/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyVMSTAT/actions/workflows/ci.yml)
[![Platform: Windows | Linux](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-blue.svg)](https://github.com/tkshnkgwr/MyVMSTAT)
[![Rust: 1.96+](https://img.shields.io/badge/rust-1.96%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Rust で実装された、`dstat` 風のカラーハイライト表示が可能な仮想メモリ統計（`vmstat`）CLIユーティリティです。標準の `vmstat` には存在しない、動的な**タイムスタンプ表示**や視認性の高いカラー閾値遷移などの機能を備えています。

[English](README.md) | **日本語**

---

## 🎨 ターミナルカラー凡例と閾値

- **灰色のゼロ値 (`0`)**: 非活性または静的なメトリクス（灰色のANSIコード `\x1b[90m`）を表し、不要なノイズを低減します。
- **黄色・オレンジのハイライト (警告)**:
  - 空き物理メモリ量 (`free`) が 1.5GB (1536MB) を下回った時 (`\x1b[1;33m`)。
  - CPU ユーザー使用率 (`us`) が 40% を超えた時 (`\x1b[1;33m`)。
  - CPU システム使用率 (`sy`) が 20% を超えた時 (`\x1b[1;33m`)。
  - コンテキストスイッチ数 (`cs`) が 1秒あたり2,000回を超えた時 (`\x1b[1;33m`)。
- **赤色のボールドハイライト (重大)**:
  - 空き物理メモリ量 (`free`) が 512MB を下回った時 (`\x1b[1;31m`)。
  - スワップ使用量 (`swpd`) が 128MB を超えて発生している時 (`\x1b[1;31m`)。
  - CPU ユーザー使用率 (`us`) が 80% を超えた時 (`\x1b[1;31m`)。
  - CPU システム使用率 (`sy`) が 40% を超えた時 (`\x1b[1;31m`)。
  - I/Oウェイト (`wa`) が 15% を超えた時 (`\x1b[1;31m`)。
- **緑色のテキスト**: 一般的なシステムアクティビティ（割り込み回数 `in`、デフォルトのコンテキストスイッチ数 `cs`）（`\x1b[32m`）。
- **青色のタイムスタンプ**: 時系列の追跡をリソース統計データから視覚的に分離します（`\x1b[34m`）。

---

## 📁 プロジェクトフォルダ構成

```text
.
├── Cargo.toml               # Cargo パッケージ設定ファイル
├── LICENSE                  # MIT ライセンスファイル
├── CHANGELOG.md             # 変更履歴の案内インデックス
├── README.md                # 英語版メインドキュメント
├── README_JA.md             # 日本語版メインドキュメント (このファイル)
├── src/
│   └── main.rs              # エントリポイントおよびプラットフォーム別処理の実装
└── docs/                    # 各種設計ドキュメント・検証レポート
    ├── en/                  # 英語版ドキュメント
    │   ├── ARCHITECTURE.md
    │   ├── DIAGRAM.md
    │   ├── FOOTPRINTS.md
    │   ├── INSTRUCTIONS.md
    │   ├── SPEC.md
    │   └── TODO.md
    └── ja/                  # 日本語版ドキュメント
        ├── ARCHITECTURE.md
        ├── DIAGRAM.md
        ├── FOOTPRINTS.md
        ├── INSTRUCTIONS.md
        ├── SPEC.md
        └── TODO.md
```

---

## ⚙️ コンパイルと実行方法

ローカルに **Rust ツールチェーン** がインストールされている必要があります。

### コマンドライン引数・オプション仕様
```bash
MyVMSTAT [delay [count]]
MyVMSTAT -h | --help
MyVMSTAT -v | --version
```
- `delay`: 出力更新間隔（秒）。デフォルトは `1.0` です。
- `count`: 最大出力回数。デフォルトは無限ループです。
- `-h, --help`: 使用方法、引数の説明、カラー判定の閾値（凡例）のヘルプメッセージを表示して終了します。
- `-v, --version`: ツールのバージョン情報を表示して終了します。

### 開発モードでの実行
```bash
cargo run -- [delay [count]]
# ヘルプを表示する場合:
cargo run -- -h
```

### 本番用ビルド (リリースビルド)
シンボル情報の削除やサイズ最適化などを施した、極小バイナリをビルドする場合：
```bash
cargo build --release
```
最適化された実行可能バイナリは `target/release/MyVMSTAT.exe`（Unix系では `MyVMSTAT`）に生成されます。

### Cargo Features (ビルド機能フラグ) の設定
`MyVMSTAT` は機能と依存関係を切り離す Cargo Features に対応しています：
- `sysinfo`: 非Linux環境用の `SysinfoProvider` (`sysinfo` クレート) を有効化。
- `windows_desktop`: Windowsの Named Mutex による二重起動防止機能 (`common_lib`) を有効化。

```bash
# デフォルト機能を無効化して特定の feature のみでビルドする場合
cargo build --release --no-default-features --features sysinfo
```

---

## 🔒 二重起動の防止 (Windows Named Mutex)
Windows環境では、名前付きミューテックスを用いて複数プロセスの同時起動を防止します。すでに `MyVMSTAT` が起動している状態で新しく別プロセスを立ち上げようとすると、以下のエラーメッセージを出力して即座に終了します。
```text
Error: Another instance of MyVMSTAT is already running.
```

---

## 📚 補足技術ドキュメント

詳細なアーキテクチャ、仕様、検証項目等については以下を参照してください。

### 🇬🇧 英語版ドキュメント
- **[System Specification](docs/en/SPEC.md)**
- **[System Diagram](docs/en/DIAGRAM.md)**
- **[System Architecture](docs/en/ARCHITECTURE.md)**
- **[Coding Style Guidelines](docs/en/INSTRUCTIONS.md)**
- **[Performance Footprints](docs/en/FOOTPRINTS.md)**
- **[Roadmap & TODO](docs/en/TODO.md)**
- **[Testing Policy](docs/en/TESTING.md)**
- **[Release Procedures](docs/en/RELEASE.md)**
- **[Contributing Guidelines](docs/en/CONTRIBUTING.md)**
- **[Security Policy](docs/en/SECURITY.md)**
- **[Changelog](docs/en/CHANGELOG.md)**

### 🇯🇵 日本語版ドキュメント
- **[システム詳細仕様書](docs/ja/SPEC.md)**
- **[システム構成図](docs/ja/DIAGRAM.md)**
- **[システムアーキテクチャ設計書](docs/ja/ARCHITECTURE.md)**
- **[開発者向けスタイル規約](docs/ja/INSTRUCTIONS.md)**
- **[リソース使用量の記録](docs/ja/FOOTPRINTS.md)**
- **[タスク管理・TODO](docs/ja/TODO.md)**
- **[テスト・検証方針](docs/ja/TESTING.md)**
- **[リリース手順書](docs/ja/RELEASE.md)**
- **[コントリビューション規約](docs/ja/CONTRIBUTING.md)**
- **[セキュリティポリシー](docs/ja/SECURITY.md)**
- **[開発ログと変更履歴](docs/ja/CHANGELOG.md)**
