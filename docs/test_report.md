# テスト報告書: MyVMSTAT 検証結果 (TEST_REPORT)

本ドキュメントは、**MyVMSTAT** の自動単体テストおよび手動動作検証の結果について記録したテスト報告書です。

---

## 1. 検証結果サマリー

- **自動テストパス率**: **100% (15 / 15 ケース成功)**
- **手動動作検証結果**: **PASS (全項目正常動作確認)**
- **最終判定**: **合格 (RELEASE READY)**

---

## 2. 検証環境

| レイヤー | 環境・モジュール | バージョン / スペック |
| :--- | :--- | :--- |
| **コンパイラ** | rustc / cargo | 1.96.0 |
| **検証OS** | Windows 11 (64-bit) | build 22631+ |
| **依存関係** | sysinfo, chrono, common_lib | Cargo.toml 定義通り |

---

## 3. 自動テスト (Unit Tests)

`cargo test` コマンドにより実行される単体テストケースの検証結果です。

### 3.1 テストケース一覧

- **`test_format_val_zero`**: 値が `0` の場合に、指定したパディング幅を維持したまま、非活性を示す灰色 ANSI エスケープコード（`\x1b[90m`）が正しく付与されることを検証。
- **`test_format_val_normal`**: 値が `0` より大きい通常時に、着色指定がない場合に正しくアライメント調整された文字列が返されることを検証。
- **`test_format_val_colored`**: 通常値に特定の色（緑など）を付与した場合に、正しいカラーエスケープが適用されることを検証。
- **`test_format_cpu_zero`**: CPU使用率が `0.0` の場合、丸め処理が行われ、灰色 ANSI エスケープコードが付与されることを検証。
- **`test_format_cpu_normal`**: CPU使用率が通常の際に、四捨五入された整数に整形されることを検証。
- **`test_format_cpu_colored`**: 高負荷警告などの色が指定された際に、CPUカラムに正しくカラーエスケープが付与されることを検証。
- **`test_parse_args_empty`**: 引数なし起動時にデフォルト値（1秒間隔、無限出力）が返ることを検証。
- **`test_parse_args_help`**: `-h` / `--help` オプション指定時に Help アクションが返ることを検証。
- **`test_parse_args_version`**: `-v` / `--version` オプション指定時に Version アクションが返ることを検証。
- **`test_parse_args_delay_only`**: 遅延秒数のみを指定した際、正常に数値がパースされることを検証。
- **`test_parse_args_delay_and_count`**: 遅延秒数と回数を指定した際、両方が正しくパースされることを検証。
- **`test_parse_args_invalid_option`**: 不正なオプション（例：`--invalid`）指定時にエラーが返ることを検証。
- **`test_parse_args_invalid_delay`**: 遅延秒数に非数値や負数を指定した際、エラーが返ることを検証。
- **`test_parse_args_invalid_count`**: 回数に非数値や小数を指定した際、エラーが返ることを検証。
- **`test_parse_args_too_many`**: 引数が多すぎる場合（3個超）にエラーが返ることを検証。

### 3.2 実行結果
```text
running 15 tests
test tests::test_format_cpu_colored ... ok
test tests::test_format_cpu_normal ... ok
test tests::test_format_cpu_zero ... ok
test tests::test_format_val_colored ... ok
test tests::test_parse_args_invalid_count ... ok
test tests::test_format_val_zero ... ok
test tests::test_parse_args_delay_and_count ... ok
test tests::test_parse_args_delay_only ... ok
test tests::test_parse_args_empty ... ok
test tests::test_parse_args_help ... ok
test tests::test_format_val_normal ... ok
test tests::test_parse_args_invalid_delay ... ok
test tests::test_parse_args_invalid_option ... ok
test tests::test_parse_args_too_many ... ok
test tests::test_parse_args_version ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

### 3.3 静的コード解析 (Clippy)
コード品質向上のため、`cargo clippy -- -D warnings` による静的解析チェックを実施しています。
- **検証内容**: コンパイラ警告やベストプラクティス違反が検出されないことを確認。
- **結果**: **PASS**. 厳格警告オプション（`-D warnings`）の指定下においても、警告およびエラーが一切出力されることなく正常に完了することを確認しました（`new_without_default` 警告に対する `Default` トレイトの実装にて完全クリア済み）。

### 3.4 RustDoc 生成テスト (cargo doc)
コードの仕様記述およびドキュメントの整合性を保証するため、`cargo doc --no-deps` による RustDoc 生成テストを実施しています。
- **検証内容**: ドキュメントコメント (`//!`, `///`) の記述に基づき、エラーや警告が発生せずにドキュメントがビルドされることを確認。
- **結果**: **PASS**. 警告やエラーが発生することなく、正常にHTMLドキュメントが生成されることを確認しました。

---

## 4. 手動検証 (Manual Verification)

本番用リリースバイナリを用いた、実機ランタイム上での機能検証結果です。

### 検証 1: 基本出力とアライメント
- **検証手順**: `.\target\release\MyVMSTAT.exe 1 5` を実行。
- **期待される動作**: ヘッダー（2行）が表示された後、1秒間隔で 5回データ行が出力されて正常にプロセスが終了すること。カラムの位置ズレがないこと。
- **結果**: **PASS**.
  - 修正によりヘッダーと数値列が寸分違わず配置され、タイムスタンプが青色、0値が灰色で描画されることを確認。

### 検証 2: 引数による制御
- **検証手順**: `.\target\release\MyVMSTAT.exe 5 2` を実行。
- **期待される動作**: 5秒間隔で 2回データ行が出力されること（約5秒の待機時間が1回発生し、2行目の出力直後に終了すること）。
- **結果**: **PASS**.
  - 待機時間が 5秒間であることを確認。また、指定回数（2回）で正常にループを抜けることを確認。

### 検証 3: 二重起動の抑止 (Named Mutex Lock)
- **検証手順**:
  1. 最初のターミナルで `.\target\release\MyVMSTAT.exe 10 2` を起動して待機状態にする。
  2. 起動中に、別のターミナルから `.\target\release\MyVMSTAT.exe 1 1` を起動する。
- **期待される動作**: 2つ目のプロセスが `common_lib::desktop::acquire_single_instance` を通じて `ERROR_ALREADY_EXISTS` を検知し、起動を拒否して即座にエラーメッセージを出力して正常終了すること。
- **結果**: **PASS**.
  - 2つ目のプロセスで `Error: Another instance of MyVMSTAT is already running.` というエラーメッセージが出力され、プロセスが即座に異常終了コードで終了することを確認。

### 検証 4: バージョン表示オプション (-v, --version)
- **検証手順**: `.\target\release\MyVMSTAT.exe -v` および `--version` を実行。
- **期待される動作**: `MyVMSTAT version 1.2.9` が出力され、プロセスが正常に終了すること。
- **結果**: **PASS**.
  - `-v` および `--version` の両方で、`Cargo.toml` に定義された正しいバージョン（1.2.14）が表示されることを確認。

### 検証 5: ヘルプ表示オプション (-h, --help)
- **検証手順**: `.\target\release\MyVMSTAT.exe -h` および `--help` を実行。
- **期待される動作**: 使い方、引数説明、オプション一覧、各カラムの項目説明（Field Descriptions）、プラットフォーム互換性注記（Platform Compatibility Note）、カラー凡例が記述されたヘルプメッセージが出力され、プロセスが正常に終了すること。
- **結果**: **PASS**.
  - 期待通りのフォーマット（追加された各カラムの項目説明およびプラットフォーム互換性注記を含む）でヘルプメッセージが出力されることを確認。

### 検証 6: 不正な引数のエラーハンドリング
- **検証手順**:
  - `.\target\release\MyVMSTAT.exe --invalid` (不正オプション)
  - `.\target\release\MyVMSTAT.exe invalid` (不正な遅延秒数)
  - `.\target\release\MyVMSTAT.exe 1 2 3` (多すぎる引数)
- **期待される動作**: それぞれ適切なエラーメッセージと Usage が標準エラー出力に出力され、終了コード `1` で異常終了すること。
- **結果**: **PASS**.
  - すべてのパターンで、終了コード `1` と期待通りのエラーメッセージ（`Error: Invalid option ...`, `Error: Invalid delay ...`, `Error: Too many arguments.`）が出力されることを確認。

---

## 5. CI/CD および自動リリース検証 (Auto-Release Verification)

GitHub Actions を用いた、バージョン自動インクリメント（バンプ）およびリリースアセット自動ビルド・デプロイラインの統合テスト結果です。

### 検証 1: 自動バージョンバンプ＆タグプッシュの検証
- **検証手順**: ローカルから空のコミットまたは変更コミットを `main` / `master` ブランチへプッシュ。
- **期待される動作**:
  1. `Auto Bump Version` ワークフローが起動し、`Cargo.toml` の `version` と本テスト報告書の適合バージョン表示が自動で `+1` されること。
  2. バージョン更新済みの `[skip ci]` コミット、および対応する新規 Git タグ（`vX.Y.Z`）が自動的に作成・プッシュされること。
- **結果**: **PASS**.
  - `pwsh` (PowerShell 7) の使用を強制することで、Windowsランナー上での日本語文字化けや構文解析エラーを回避し、自動バンプコミットとタグプッシュが安定して実行されることを確認。

### 検証 2: 自動リリースビルド＆アセットアップロードの検証
- **検証手順**: 上記の自動タグプッシュ（`v*`）をフックとした `Release Build` ワークフローの自動起動。
- **期待される動作**:
  1. クロスプラットフォーム向けバイナリ（Windows `x86_64-pc-windows-msvc` / Linux `x86_64-unknown-linux-gnu`）がそれぞれリリースモードで自動ビルドされること.
  2. それぞれの成果物が `LICENSE` や `README.md` などの必須ドキュメントと一括パッケージング（Windows は `.zip`、Linux は `.tar.gz`）されること。
  3. GitHub Release にリリースノートが自動生成され、作成されたアセットファイルが自動でアップロードされること。
- **結果**: **PASS**.
  - Linux環境における圧縮済み `.tar.gz` に対する `tar -rvf`（追記）エラーによるビルド失敗について、必要なファイルを出力先ディレクトリへコピーしてから一括圧縮する設計へと改善し、エラー（`exit code 2`）を解消。Windows/Linuxの全ターゲットでリリースアセットが正常に自動アップロードされることを確認。

---

## 6. 総評

すべての自動テスト、CLI基本機能の動作検証、および今回新規導入した CI/CD 自動リリースラインの統合検証が正常にパスしました。Windows環境における Named Mutex による排他ロック、`sysinfo` を介したメモリ・CPU 使用率の取得、および ANSI カラーコードのカラーマッピング、そして堅牢な引数バリデーションが仕様書通りに機能していることを確認しました。また、GitHub Actions におけるリリース自動化ライン（バグ修正済み）も完全に統合され、開発・運用フェーズにおける安定したデプロイ環境が整いました。
