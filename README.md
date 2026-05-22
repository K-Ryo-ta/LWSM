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
lwsm -cでファイルの中身から該当のファイルを探すコマンド

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
   └─ output.rs
```
