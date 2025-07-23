# cogs

Google Gemini APIを利用したAI翻訳CLIツール

## 概要

`cogs`は、指定したテキストファイルをAI（Google Gemini）で任意の言語に翻訳するコマンドラインツールです。  
APIキーや翻訳先言語、出力ファイル名を指定できます。

## 必要要件

- Rust 1.74以上（edition 2024対応）
- Google Gemini APIキー
- インターネット接続

## インストール

```sh
git clone https://github.com/Dangornushi/cogs.git
cd cogs
cargo build --release
```

## 使い方

1. `.env`ファイルを作成し、以下のようにAPIキーを設定してください。

```
API_KEY=あなたのGoogle_Gemini_APIキー
```

2. コマンド実行例

```sh
cargo run --release -- ファイルパス [--lang 言語] [--output 出力ファイル]
```

- `ファイルパス`: 翻訳したいテキストファイルのパス（必須）
- `--lang`: 翻訳先言語（省略時は日本語）
- `--output`: 翻訳結果の出力ファイル名（省略時は標準出力）

### 例

```sh
cargo run --release -- input.txt --lang 英語 --output result.txt
```

## 主な依存クレート

- [`reqwest`](https://crates.io/crates/reqwest)
- [`tokio`](https://crates.io/crates/tokio)
- [`dotenv`](https://crates.io/crates/dotenv)
- [`clap`](https://crates.io/crates/clap)
- [`serde`](https://crates.io/crates/serde)
- [`serde_json`](https://crates.io/crates/serde_json)
- [`indicatif`](https://crates.io/crates/indicatif)

## ライセンス

MIT# Cogs
