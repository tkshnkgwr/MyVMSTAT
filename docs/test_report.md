# テスト報告書: MyVMSTAT 検証結果 (TEST_REPORT)

本ドキュメントは、**MyVMSTAT (rust-vmstat)** の自動単体テストおよび手動動作検証の結果について記録したテスト報告書です。

---

## 1. 検証結果サマリー

- **自動テストパス率**: **100% (6 / 6 ケース成功)**
- **手動動作検証結果**: **PASS (全項目正常動作確認)**
- **最終判定**: **合格 (RELEASE READY)**

---

## 2. 検証環境

| レイヤー | 環境・モジュール | バージョン / スペック |
| :--- | :--- | :--- |
| **コンパイラ** | rustc / cargo | 1.70.0+ |
| **検証OS** | Windows 11 (64-bit) | build 22631+ |
| **依存関係** | sysinfo, chrono, windows | Cargo.toml 定義通り |

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

### 3.2 実行結果
```text
running 6 tests
test tests::test_format_cpu_colored ... ok
test tests::test_format_val_normal ... ok
test tests::test_format_cpu_zero ... ok
test tests::test_format_val_colored ... ok
test tests::test_format_cpu_normal ... ok
test tests::test_format_val_zero ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

## 4. 手動検証 (Manual Verification)

本番用リリースバイナリを用いた、実機ランタイム上での機能検証結果です。

### 検証 1: 基本出力とアライメント
- **検証手順**: `.\target\release\rust-vmstat.exe 1 5` を実行。
- **期待される動作**: ヘッダー（2行）が表示された後、1秒間隔で 5回データ行が出力されて正常にプロセスが終了すること。カラムの位置ズレがないこと。
- **結果**: **PASS**.
  - ヘッダーと数値列が寸分違わず配置され、タイムスタンプが青色、0値が灰色で描画されることを確認。

### 検証 2: 引数による制御
- **検証手順**: `.\target\release\rust-vmstat.exe 5 2` を実行。
- **期待される動作**: 5秒間隔で 2回データ行が出力されること（約5秒の待機時間が1回発生し、2行目の出力直後に終了すること）。
- **結果**: **PASS**.
  - 待機時間が 5秒間であることを確認。また、指定回数（2回）で正常にループを抜けることを確認。

### 検証 3: 二重起動の抑止 (Named Mutex Lock)
- **検証手順**:
  1. 最初のターミナルで `.\target\release\rust-vmstat.exe 10 2` を起動して待機状態にする。
  2. 起動中に、別のターミナルから `.\target\release\rust-vmstat.exe 1 1` を起動する。
- **期待される動作**: 2つ目のプロセスが `ERROR_ALREADY_EXISTS` を検知し、起動を拒否して即座にエラーメッセージを出力して正常終了すること。
- **結果**: **PASS**.
  - 2つ目のプロセスで `Error: Another instance of rust-vmstat is already running.` というエラーメッセージが出力され、プロセスが即座に異常終了コードで終了することを確認。

---

## 5. 総評

すべての自動テストおよび手動シナリオテストが正常にパスしました。Windows環境における Named Mutex による排他ロック、`sysinfo` を介したメモリ・CPU 使用率の取得、および ANSI カラーコードのカラーマッピングが仕様書通りに機能していることを確認しました。
