# Changelog: rust-vmstat & Sandbox Dashboard

All notable changes to this project are documented in this file. This project adheres to Semantic Versioning.

---

## [1.2.0] - 2026-06-30

### Added
- **本番用 CLI 実装の完了**: `sysinfo` を用いたクロスプラットフォーム監視と、Linux専用 `/proc` 解析を `src/main.rs` に完全実装。
- **二重起動防止 (Windows)**: Windows環境下で名前付きミューテックス (`CreateMutexW`) を用いた多重起動防止機構を導入。
- **リリース最適化**: `Cargo.toml` の `profile.release` にサイズ削減およびパフォーマンス最適化設定を追加（最適化後のバイナリサイズは約 254 KB）。
- **MITライセンス**: リポジトリルートに `LICENSE` ファイルを追加。
- **ドキュメントの再構築および多言語対応**:
  - `README.md` (英語版) を Rust CUI ツール専用に最新化。
  - `README.ja.md` (日本語版) を新規作成・最新化。
  - `docs/SPEC.md` (仕様書) を日本語に翻訳・更新。
  - `docs/DIAGRAM.md` (システム構成図) を Mermaid で新規作成。
  - `docs/FOOTPRINTS.md` (リソースフットプリント記録) を新規作成し、バイナリサイズやメモリ使用量を記録。
  - `docs/TEST_REPORT.md` (検証レポート) を新規作成（古い `test_report.md` は削除）。

---

## [1.1.0] - 2026-06-29

### Added
- **Production Rust CLI Executable**: Bootstrapped complete Rust sources (`rust-vmstat/`) supporting Linux-Native procfs reading and Cross-Platform `sysinfo` runtime.
- **Root Level Multilingual Documentation**:
  - Moved and upgraded English (`README.md`) and Japanese (`README.ja.md`) documentation to the workspace root for convenient initial access.
- **Architecture Specification (`docs/SPEC.md`)**: Drafted complete technical blueprints and data parsing designs of the Rust systems code.
- **Verification Logs (`docs/test_report.md`)**: Completed full test execution summaries across 6 test case categories verifying colorizer boundaries and simulated scenarios.
- **Timestamp Column**: Merged dynamic high-visibility `timestamp` columns in the simulator, standard logging exports, and Rust compilations to prevent chronological drift during diagnostic reviews.

### Changed
- **Icon Dependency Cleanup**: Removed the obsolete `Windows` icon from React layouts and substituted lightweight lucide vectors to prevent compilation blockages.
- **Unified Dev Configuration**: Restarted the container runtime using streamlined `tsc` builds to ensure modular React component bindings function flawlessly.

---

## [1.0.0] - 2026-06-28

### Added
- **Initial Interactive Sandbox**: Developed the React visual terminal simulator mirroring the layout and output behaviors of standard Linux vmstat.
- **Interactive Load Controls**: Integrated slide throttles and scenario macro presets (Idle, CPU Spike, Memory Leak, IO Bottleneck) allowing developers to trigger simulated performance strain.
- **AI-Powered Feature Extension**: Provided a smart assistant panel enabling users to add customized runtime telemetry columns (e.g., thermal bounds, physical network interfaces) using on-the-fly Gemini custom bindings.
- **Local Logs Exporter**: Integrated text-formatted exporter routines generating clean timestamp-delimited metric rows compatible with Linux log-parsers.
