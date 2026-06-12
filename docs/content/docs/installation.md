---
title: インストール
description: ソースコードからのビルドとインストール手順
weight: 10
---

LWSM は Rust 製の CLI ツールです。インストールには [Rust ツールチェーン](https://www.rust-lang.org/ja/tools/install)（`cargo`）が必要です。

## ソースからインストール

```bash
git clone https://github.com/K-Ryo-ta/LWSM.git
cd LWSM
cargo install --path .
```

インストール後、`lwsm` コマンドが使えるようになります。

```bash
lwsm -m rust
```

## ビルドのみ行う場合

```bash
cargo build --release
```

バイナリは `target/release/lwsm` に生成されます。

## 動作環境

- macOS / Linux / Windows
- 依存クレートなしの単一バイナリ
