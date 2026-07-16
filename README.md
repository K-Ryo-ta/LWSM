# LWSM

![License](https://img.shields.io/github/license/K-Ryo-ta/LWSM)
![Version](https://img.shields.io/badge/version-1.0.0-blue)
[![Coverage Status](https://coveralls.io/repos/github/K-Ryo-ta/LWSM/badge.svg?branch=main)](https://coveralls.io/github/K-Ryo-ta/LWSM?branch=main)
[![DOI](https://zenodo.org/badge/1206543697.svg)](https://doi.org/10.5281/zenodo.20195247)

lsコマンドでフォルダの中身を見るときに、中身が多すぎる場合に単語検索や文章検索で調べられるコマンド

## Description

lsコマンドでフォルダの中身を見るときに、中身が多すぎる場合に単語検索や文章検索で調べられるコマンド

lwsm -mでディレクトリ名・ファイル名の単語マッチ検索
lwsm -sでディレクトリ名・ファイル名の文章検索
lwsm -cでファイルの中身からキーワードを含むファイルを探す
lwsm -tでファイルの文字数を調べる

```bash
lwsm -c "キーワード" [path]
```

指定したディレクトリ（省略時はカレントディレクトリ）内の各ファイルの中身を読み込み、キーワードを含むファイルだけを表示します。大文字・小文字は区別しません。ディレクトリや、テキストとして読み込めないファイルはスキップされます。

### 文字数を数える（-t）

```bash
lwsm -t [path]
```

`path` にファイルを指定するとそのファイルの文字数を、ディレクトリを指定（省略時はカレントディレクトリ）すると直下の各ファイルの文字数と合計（`total`）を表示します。文字数は空白・改行・タブを除いて数え、日本語などのマルチバイト文字も1文字として数えます。テキストとして読み込めないファイル（バイナリなど）はスキップされます。

```bash
$ lwsm -t src
    1458  config.rs
     921  entries.rs
     879  gencomp.rs
    1526  lib.rs
      96  main.rs
     371  output.rs
    2172  search.rs
    1647  wordcount.rs
    9070  total
```

## ディレクトリ構成

```
lwsm/
├─ Cargo.toml
└─ src/
   ├─ main.rs
   ├─ lib.rs
   ├─ config.rs
   ├─ entries.rs
   ├─ search.rs
   ├─ wordcount.rs
   └─ output.rs
```

## インストール（Homebrew）

`homebrew-tap` 経由でインストールできます。

```bash
brew tap K-Ryo-ta/tap
brew install lwsm
```

または1行で:

```bash
brew install K-Ryo-ta/tap/lwsm
```

インストール時に `lwsm --completions` で生成したシェル補完（bash / zsh / fish）も同時に導入されます。

## Docker配布手順（マルチプラットフォーム）

`linux/amd64` と `linux/arm64` のコンテナイメージを、同じタグで `ghcr.io` に配布できます。

### 1. 事前準備（初回のみ）

- GitHub Personal Access Token を作成する（`write:packages` 権限を付与）
- ローカルで `ghcr.io` にログインする

```bash
export ACCESS_TOKEN=<your_github_token>
echo "$ACCESS_TOKEN" | docker login ghcr.io -u <your_github_username> --password-stdin
```

### 2. Buildxビルダー作成（初回のみ）

```bash
docker buildx create --use
docker buildx ls
```

### 3. 手元で簡単にビルドする（Just）

```bash
just docker-image 0.1.0
```

`Justfile` は `.github/scripts/build_docker.sh` を呼び出します。  
毎回長い `docker buildx build ...` を直接書かなくて済みます。

### 4. 手元からGHCRへpushする（Just）

```bash
just docker-image-push 0.1.0
```

任意のイメージ名を使う場合:

```bash
just docker-image-push-to ghcr.io/<owner>/<repo> 0.1.0
```

### 5. 実行確認

```bash
docker run --rm ghcr.io/<owner>/lwsm:latest --help
```

## GitHub Actionsでの自動配布

`publish` ワークフロー（`.github/workflows/publish.yaml`）に `container_image` ジョブを組み込み、`finalize` の前に自動実行されるようにしています。

- `docker buildx` で `linux/amd64,linux/arm64` をビルド
- `ghcr.io/<owner>/<repo>` に push
- OCIメタデータ（`title`, `description`, `licenses`, `authors`, `version`, `source` など）を付与
- タグ `<version>` と `latest` を付与

また `homebrew` ジョブで、リリース資産の SHA256 を算出して Formula（`.github/templates/lwsm.rb` を埋めたもの）を生成し、`K-Ryo-ta/homebrew-tap` の `Formula/lwsm.rb` に自動 push します。

### Homebrew自動配布に必要な準備（初回のみ）

- GitHub に `homebrew-tap` リポジトリ（`K-Ryo-ta/homebrew-tap`）を作成する
- `homebrew-tap` に push できる PAT（`repo` 権限）を発行する
- このリポジトリの Secrets に `TAP_GITHUB_TOKEN` として登録する
