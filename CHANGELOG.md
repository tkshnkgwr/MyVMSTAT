# Changelog: MyVMSTAT & Sandbox Dashboard

All notable changes to this project are documented in this file. This project adheres to Semantic Versioning.

## [1.2.4] - 2026-07-06

### Changed
- **CI/CDワークフローの正常化**:
  - 各ワークフロー定義ファイル (`ci.yml`, `release.yml`) 内の `actions/checkout` のバージョンを非推奨の `v7` から安定版 `v4` へ、`softprops/action-gh-release` を `v2` へ更新。
  - 依存ライブラリ `common_lib` の相対パス依存を解決するため、チェックアウト後に `tkshnkgwr/common_lib` を同じ階層の `common_lib` パスへクローンするステップを追加（`PAT` 未設定時のフォールバックとして `token: ${{ secrets.PAT || github.token }}` を設定）。
  - 各ジョブに `defaults.run.working-directory: MyVMSTAT` を設定し、`run` コマンドがプロジェクトのディレクトリ内で実行されるように調整。
  - `release.yml` におけるリリースアセット指定パスを、実際の出力先である `MyVMSTAT/${{ matrix.release_name }}` に修正。また、Linuxアセットのパッケージング処理を `MyVMSTAT` ディレクトリ基準で動作するようシンプルかつ堅牢な記述に改善。
  - `release.yml` 内の `Upload Release Asset` ステップに `generate_release_notes: true`, `draft: false`, `prerelease: false` オプションを追加し、リリース自動作成と変更ログ自動生成を統合。
  - `ci.yml` に自動タグ生成ステップ（`Auto Tagging`）を導入。`Cargo.toml` のバージョン更新時に、対応するGitタグ（例: `v1.2.4`）を自動的に作成してプッシュし、手動のタグ操作なしで自動的にリリースビルドが走るよう改善。
  - `ci.yml` の `Checkout code` ステップに `token: ${{ secrets.PAT || github.token }}` の指定を追加し、PAT経由でのタグプッシュによりリリースビルドの自動トリガー制限（GITHUB_TOKENのトリガー制限）を回避。
- **コードフォーマットの適用**:
  - `cargo fmt` によるコード自動整形を適用し、静的検証エラーを解消。
- **ドキュメントの更新**:
  - `README.md` および `README.ja.md` のヘッダー部に GitHub Release バッジを追加。

---

## [1.2.3] - 2026-07-03

### Changed
- **共有ライブラリの活用と二重起動防止ロジックの移行**:
  - `MyVMSTAT` 内の Windows 向け二重起動防止ロジックを、プロジェクト間共有ライブラリ `common_lib` の `desktop::acquire_single_instance` を使うように移行。
  - `MyVMSTAT/Cargo.toml` から `windows` クレートへの直接依存を排除し、`common_lib`（`windows_desktop` フィーチャー有効化）に依存するように集約。
  - `MyVMSTAT/src/main.rs` から unsafe な Win32 API 依存の `check_single_instance` 関数定義を削除し、コードの安全性と保守性を向上。

---

## [1.2.2] - 2026-06-30

### Changed
- **依存ライブラリの更新**:
  - `windows` クレートを `0.52` から `0.62` へアップデート。
  - `sysinfo` クレートを `0.30` から `0.39` へアップデート。
- **ソースコードのAPI仕様変更追従**:
  - `windows 0.62` での `GetLastError()` の戻り値の型変更に対応し、`WIN32_ERROR` 構造体を用いた型安全な比較に修正。
  - `sysinfo 0.39` での CPU 使用率取得方法の変更に対応し、`global_cpu_usage()` に移行。
- **GitHub Actions ワークフローの改善**:
  - ワークフローファイル (`ci.yml`, `release.yml`) 内の `actions/checkout` アクションのバージョン指定を `v7` から安定版 `v4` に修正。
  - `release.yml` 内の実行ファイル・アーカイブファイル名における他プロジェクト名（`MyNKF`）のコピペミスを `MyVMSTAT` に修正。
- **Rust 1.96.0 環境への対応**:
  - 新環境でのビルドおよび全単体テスト（6ケース）が正常にパスすることを確認。
  - 実行ファイルサイズ（約 201 KB）および物理メモリ使用量（約 15.9 MB）の再計測を行い、動作検証レポートおよびフットプリントドキュメントを更新。
- **READMEバッジと多言語相互リンクの追加・拡充**:
  - READMEにCIビルドステータス、プラットフォーム（Windows/Linux）、Rustバージョン（1.96.0+）、およびライセンス（MIT）の各バッジを導入・拡充。日英READMEの相互リンク関係を保証。
- **ガイドラインの更新 (`AGENTS.md`)**:
  - ドキュメント自動更新ルールにREADMEバッジおよび相互リンク維持のガイドラインを追記。
- **引数パースのリファクタリングと単体テスト追加**:
  - 引数パース処理を `parse_args` 関数へ分離し、新たに 9 つの引数解析用単体テストを追加（全15テストがPASS）。
- **CI/CD ワークフローの新規構築**:
  - `.github/workflows/ci.yml` (自動ビルド/テスト) および `release.yml` (自動リリースアセットパッケージ) を新規追加。
- **CIビルド警告エラーの修正**:
  - `SysinfoProvider` および `LinuxProvider` に対して `Default` トレイトを実装し、Clippy の `new-without-default` 警告を解消（GitHub Actions の CI ビルドエラーを修正）。

---

## [1.2.1] - 2026-06-30

### Changed
- **プログラム名の変更**: プログラムおよび実行ファイル名を `rust-vmstat` から `MyVMSTAT` に変更。
- **ハードコーディングの排除**: ヘルプやエラー出力等で使用されていたプログラム名とバージョンの出力を、`env!("CARGO_PKG_NAME")` および `env!("CARGO_PKG_VERSION")` を使用して Cargo.toml の情報から動的に取得するように変更。
- **ドキュメント更新**: すべての関連ドキュメントおよびシステム仕様書内のプログラム名・二重起動ミューテックス名の記述を更新。

---

## [1.2.0] - 2026-06-30

### Added
- **本番用 CLI 実装の完了**: `sysinfo` を用いたクロスプラットフォーム監視と、Linux専用 `/proc` 解析を `src/main.rs` に完全実装。
- **二重起動防止 (Windows)**: Windows環境下で名前付きミューテックス (`CreateMutexW`) を用いた多重起動防止機構を導入。
- **ヘルプ・バージョンオプションの追加**: `-h`/`--help` および `-v`/`--version` オプションを実装。バージョン情報は `env!("CARGO_PKG_VERSION")` により `Cargo.toml` と連動させて非ハードコード化。
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
