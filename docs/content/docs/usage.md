---
title: 使い方
description: コマンドの形式、オプション、検索モードの違い
weight: 20
---

## コマンド形式

```bash
lwsm -m <word> [path]
lwsm -s <sentence> [path]
```

`path` を省略するとカレントディレクトリ（`.`）が対象になります。

## オプション

| オプション | エイリアス | 説明 |
|---|---|---|
| `-m` | `--match` | 単語単位の完全一致でファイル名・ディレクトリ名を検索 |
| `-s` | `--sentence` | 部分一致（大文字小文字を区別しない）で検索 |

## 検索モードの違い

### 単語マッチ検索（`-m`）

ファイル名を `_` / `-` / `.` / `/` / `\` で単語に分割し、単語単位で完全一致するものを表示します。

```bash
$ lwsm -m rust
gamma-rust.txt    # "rust" という単語を含むのでヒット
```

`readme.md` は「readme」ではヒットしますが、「read」ではヒットしません。

### 文章検索（`-s`）

正規化（小文字化・区切り文字の空白化）したファイル名に対する部分一致検索です。

```bash
$ lwsm -s "hello world"
hello_world.txt   # "hello_world" → "hello world" に正規化されてヒット
```

## 実行例

```bash
lwsm -m rust            # カレントディレクトリから "rust" を単語検索
lwsm -s "hello world"   # 文章検索
lwsm -m test ./src      # ./src を対象に検索
```

ディレクトリは末尾に `/` が付いて表示されます。
