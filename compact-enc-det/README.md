# compact-enc-det

[![Crates.io](https://img.shields.io/crates/v/compact-enc-det.svg)](https://crates.io/crates/compact-enc-det)
[![Documentation](https://docs.rs/compact-enc-det/badge.svg)](https://docs.rs/compact-enc-det)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

High-level Rust bindings for [Compact Encoding Detection (CED)](https://github.com/google/compact_enc_det) - a library for detecting character encodings in text.

## Features

- Safe, ergonomic Rust API
- Type-safe enums for encodings, languages, and corpus types
- Support for 75+ character encodings
- Provides reliability confidence scores
- Optional hints for improved detection accuracy

## Installation

```toml
[dependencies]
compact-enc-det = "0.1"
```

## Usage

```rust
use compact_enc_det::{detect_encoding, DetectHints, Encoding};

fn main() {
    let text = "Hello, world! 你好世界";
    let detection = detect_encoding(text.as_bytes(), DetectHints::default());

    println!("Detected: {:?}", detection.encoding);
    println!("MIME name: {}", detection.mime_name);
    println!("Reliable: {}", detection.is_reliable);

    assert_eq!(detection.encoding, Encoding::UTF8);
}
```

### Detection with Hints

```rust
use compact_enc_det::{detect_encoding, DetectHints, Language, TextCorpusType};

let hints = DetectHints {
    url_hint: "https://example.jp/page.html",
    language_hint: Some(Language::JAPANESE),
    corpus_type: TextCorpusType::WEB_CORPUS,
    ..Default::default()
};

let detection = detect_encoding(japanese_text, hints);
```

## Documentation

For complete documentation, see [docs.rs/compact-enc-det](https://docs.rs/compact-enc-det).

## License

MIT License - see [LICENSE](../LICENSE) file for details.

The underlying C++ library is licensed under Apache License 2.0.
