[English](../en/INSTRUCTIONS.md) | **日本語版**

# AI・開発者向けコーディングスタイル規約: INSTRUCTIONS

本ドキュメントは、**MyVMSTAT** プロジェクトにおいて今後ソースコードの修正・機能追加を行う際、開発者およびAIアシスタントが遵守すべきコーディングスタイル、エラーハンドリング方針、コンポーネント設計、および出力フォーマットについて規定します。

---

## 1. 命名規則
Rust の標準的なネーミングコンベンション（RFC 430）に準拠します。

| 対象項目 | 表記法 (Casing) | 具体例 | 備考 |
| :--- | :--- | :--- | :--- |
| **変数 / 関数 / パラメータ** | `lower_snake_case` | `delay`, `parse_args`, `loop_count` | 簡潔で分かりやすい名称を選択する |
| **構造体 / 列挙型 / トレイト** | `PascalCase` | `VmstatData`, `TelemetryProvider` | 略称は全て大文字にする (例: `CliAction`) |
| **定数 / マクロ** | `UPPER_SNAKE_CASE` | `CARGO_PKG_NAME`, `Local\MyVMSTAT-...` | 視認性を高めるために大文字統一 |
| **ソースファイル名** | `lower_snake_case` | `main.rs` | モジュール名と一致させる |
| **ドキュメントファイル名** | `UPPER_SNAKE_CASE.md` | `ARCHITECTURE.md`, `TODO.md` | 本プロジェクト独自の統一ルール |

---

## 2. エラーハンドリングの方針
- **例外（パニック）の回避**: `unwrap()` や `expect()` の使用は、テストコード、または「絶対に失敗しないことが保証されている極めて限定的な文脈」を除き、原則禁止します。
- **Result/Option を用いた戻り値での伝搬**: ユーザー入力に起因するエラーは `Result<T, String>` などの戻り値として表現し、メイン関数で受け取って処理します。
- **致命的エラー時のプロセス終了**: `eprintln!` に書き出し、`std::process::exit(1)` によって終了コード `1` で速やかに終了させます。

---

## 3. コンポーネントおよびモジュールの分割基準
- **単一ファイルの維持と1,000行基準**: 単一ファイルが **1,000 行を超えた場合**、機能・モジュール単位への分割やリファクタリングの実施・提案を推進します。
- **外部共通クレートへの切り出し (`common_lib`)**: 共通のプラットフォーム固有機能は `common_lib` からインポートします。

---

## 4. 自動ドキュメンテーションと更新ルール (モジュール化退避)
コードの変更、リファクタリング、機能追加、バグ修正、または依存ライブラリ更新時、英語版 (`docs/en/`) と日本語版 (`docs/ja/`) の両ドキュメントを完全同期して更新します。
※**`.md` ファイルのみの変更時は、自動ドキュメント更新および事前検証プロセスはすべてスキップ可**。

| ドキュメント | 役割 | 更新タイミング・詳細規則 |
|---|---|---|
| [CHANGELOG.md (EN)](../docs/en/CHANGELOG.md)<br>[CHANGELOG.md (JA)](../docs/ja/CHANGELOG.md) | 変更履歴の記録 | 実装完了時に日付 `## [YYYY-MM-DD]` 単位で追記。<br>カテゴリ: `Added`, `Fixed`, `Optimized`, `Removed` |
| [SPEC.md (EN)](../docs/en/SPEC.md)<br>[SPEC.md (JA)](../docs/ja/SPEC.md) | アプリ機能仕様・技術スタック | コマンドライン引数、フィルタリング規則、表示フォーマット、動作OSなどの仕様変更時 |
| [DIAGRAM.md (EN)](../docs/en/DIAGRAM.md)<br>[DIAGRAM.md (JA)](../docs/ja/DIAGRAM.md) | システム構成・実行フローの可視化 | データ取得フロー、スレッド構造、OS判定分岐などの変更時（Mermaid更新） |
| [README.md](../README.md)<br>[README_JA.md](../README_JA.md) | プロジェクト概要・ビルド手順・解説 | 起動オプション、ビルド手順、動作要件、ステータスバッジ、ドキュメント一覧の変更時 |
| [FOOTPRINTS.md (EN)](../docs/en/FOOTPRINTS.md)<br>[FOOTPRINTS.md (JA)](../docs/ja/FOOTPRINTS.md) | パフォーマンス・サイズ・リソース記録 | リリースビルドのバイナリサイズ変更、最適化設定変更、新規計測結果の取得時 |
| [ARCHITECTURE.md (EN)](../docs/en/ARCHITECTURE.md)<br>[ARCHITECTURE.md (JA)](../docs/ja/ARCHITECTURE.md) | 設計概要・std依存ゼロ方針・データフロー | 内部構造刷新、モジュールの新規作成/分割、データフロー構造の変更時 |
| [INSTRUCTIONS.md (EN)](../docs/en/INSTRUCTIONS.md)<br>[INSTRUCTIONS.md (JA)](../docs/ja/INSTRUCTIONS.md) | AI向けコーディング規約 | 命名規則、エラーハンドリング、モジュール分割、回答フォーマットの管理。 |
| [TODO.md (EN)](../docs/en/TODO.md)<br>[TODO.md (JA)](../docs/ja/TODO.md) | タスク・バックログ管理 | タスク完了（Done）、直近タスク追加（In Progress/Todo）、拡張提案追加（Backlog）時 |
| [TESTING.md (EN)](../docs/en/TESTING.md)<br>[TESTING.md (JA)](../docs/ja/TESTING.md) | テスト方針・検証項目・手順 | テスト観点の変更や新たな検証コマンドの導入時 |
| [RELEASE.md (EN)](../docs/en/RELEASE.md)<br>[RELEASE.md (JA)](../docs/ja/RELEASE.md) | バージョン更新・リリース手順書 | リリースプロセス、タグ打ち、PowerShellスクリプト (`scripts/bump-version.ps1`) 変更時 |
| [CONTRIBUTING.md (EN)](../docs/en/CONTRIBUTING.md)<br>[CONTRIBUTING.md (JA)](../docs/ja/CONTRIBUTING.md) | 貢献者向けガイドライン | コミット規約、PR提出手順、環境構築方法の変更時 |
| [SECURITY.md (EN)](../docs/en/SECURITY.md)<br>[SECURITY.md (JA)](../docs/ja/SECURITY.md) | セキュリティ原則・脆弱性報告定義 | セキュリティ方針、サポート対象バージョン、報告先の変更時 |
| Rustdoc (ソースコード内) | 構造体・関数・モジュールの仕様記述 | プログラム仕様変更・機能追加/修正時に Rustdoc (`///`, `//!`) を最新化 |

---

## 5. 品質管理・自動テストと事前検証ルール (モジュール化退避)
- **ユニットテストの追加・拡充**: 新機能追加やコアロジック変更時は、可能な限り関連するユニットテストを追加・拡張すること。
- **ローカル事前検証プロセス** (※ `.md` のみ変更時は全スキップ可):
  1. `cargo test` （テスト合格）
  2. `cargo clippy --all-targets -- -D warnings` （Clippy警告・エラーゼロ）
  3. `cargo fmt --check` （コードフォーマット完全準拠）
  4. `cargo doc --no-deps --document-private-items` （Rustdocビルドエラー・警告ゼロ）
- **二重起動防止 (Windows Named Mutex)**: Windows環境での多重起動を防ぐため、`common_lib` の Named Mutexを `fn main()` で実行し、重複時は即座に正常終了すること。
- **リリース時のサイズ最適化**:
  バイナリサイズ最小化のため、`Cargo.toml` の `[profile.release]` に `opt-level = 'z'`, `lto = true`, `codegen-units = 1`, `panic = 'abort'`, `strip = true` を維持すること。
