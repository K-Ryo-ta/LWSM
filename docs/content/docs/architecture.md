---
title: 設計とテスト
description: モジュール構成とテスト戦略（S / M / L テスト）
weight: 30
---

## ディレクトリ構成

責務ごとにモジュールを分割しています。

```text
lwsm/
├─ Cargo.toml
├─ src/
│  ├─ main.rs      # CLI エントリポイント
│  ├─ lib.rs       # 各モジュールの公開
│  ├─ config.rs    # 引数解析
│  ├─ entries.rs   # ディレクトリ列挙
│  ├─ search.rs    # 検索ロジック
│  └─ output.rs    # 結果出力
└─ tests/
   ├─ integration_pipeline.rs  # 結合テスト
   └─ system_cli.rs            # システムテスト
```

## 処理の流れ

1. `config` — コマンドライン引数を解析して `Config`（モード・クエリ・パス）を生成
2. `entries` — 対象ディレクトリのエントリを列挙し、名前順にソート
3. `search` — モードに応じて単語マッチ / 文章検索でフィルタ
4. `output` — 結果を出力（ディレクトリは末尾に `/` を付与）

## テスト戦略

| 種別 | 置き場所 | 対象 |
|---|---|---|
| S（単体） | 各ソースファイル内の `#[cfg(test)]` | `normalize` / `word_match` / `parse_mode` など |
| M（結合） | `tests/integration_pipeline.rs` | ディレクトリ列挙 → 検索の連携 |
| L（システム） | `tests/system_cli.rs` | `lwsm` バイナリの CLI 全体 |

すべてのテストは以下で実行できます。

```bash
cargo test
```

## CI / カバレッジ

GitHub Actions で Ubuntu / macOS / Windows のビルドとテストを自動実行し、
[Coveralls](https://coveralls.io/github/K-Ryo-ta/LWSM?branch=main) でカバレッジを計測しています。
