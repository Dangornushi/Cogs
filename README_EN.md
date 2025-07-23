# cogs

An AI translation CLI tool utilizing the Google Gemini API

## Overview

`cogs` is a command-line tool that translates specified text files into any language using AI (Google Gemini).
You can specify an API key, target language for translation, and output filename.

## Prerequisites

- Rust 1.74 or later (compatible with edition 2024)
- Google Gemini API Key
- Internet connection

## Installation

```sh
git clone https://github.com/Dangornushi/cogs.git
cd cogs
cargo build --release
```

## Usage

1. Create a `.env` file and set your API key as follows:

```
API_KEY=YOUR_GOOGLE_GEMINI_API_KEY
```

2. Command Execution Example

```sh
cargo run --release -- FILE_PATH [--lang LANGUAGE] [--output OUTPUT_FILE]
```

- `FILE_PATH`: Path to the text file you want to translate (required)
- `--lang`: Target language for translation (defaults to Japanese if omitted)
- `--output`: Output filename for the translation result (defaults to standard output if omitted)

### Example

```sh
cargo run --release -- input.txt --lang English --output result.txt
```

## Main Dependencies

- [`reqwest`](https://crates.io/crates/reqwest)
- [`tokio`](https://crates.io/crates/tokio)
- [`dotenv`](https://crates.io/crates/dotenv)
- [`clap`](https://crates.io/crates/clap)
- [`serde`](https://crates.io/crates/serde)
- [`serde_json`](https://crates.io/crates/serde_json)
- [`indicatif`](https://crates.io/crates/indicatif)

## License

MIT