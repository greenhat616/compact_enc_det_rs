# compact-enc-det

[![Crates.io](https://img.shields.io/crates/v/compact-enc-det.svg)](https://crates.io/crates/compact-enc-det)
[![Documentation](https://docs.rs/compact-enc-det/badge.svg)](https://docs.rs/compact-enc-det)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Rust bindings for [Compact Encoding Detection (CED)](https://github.com/google/compact_enc_det) - a library for detecting character encodings in text. This library is based on Google's Compact Encoding Detection C++ library, which scans raw bytes and detects the most likely text encoding.

## Features

- Detect character encoding from raw bytes
- Support for 75+ different encodings including:
  - UTF-8, UTF-16, UTF-32
  - ISO-8859 family
  - Windows code pages (CP1250-CP1257, etc.)
  - Chinese encodings (GB2312, GBK, GB18030, Big5)
  - Japanese encodings (Shift_JIS, EUC-JP, ISO-2022-JP)
  - Korean encodings (EUC-KR)
  - Russian encodings (KOI8-R, Windows-1251)
  - And many more
- Provides reliability confidence score
- Optional hints for better detection (URL, HTTP charset, meta charset, language, corpus type)
- Safe Rust API with ergonomic types
- Optional integration with `encoding_rs` for encoding conversion

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
compact-enc-det = "0.1"
```

For the optional `encoding_rs` integration:

```toml
[dependencies]
compact-enc-det = { version = "0.1", features = ["encoding_rs"] }
```

## Usage

### Basic Detection

```rust
use compact_enc_det::{detect_encoding, DetectHints};

fn main() {
    let text = "Hello, world! 你好世界 🌍";
    let detection = detect_encoding(text.as_bytes(), DetectHints::default());

    println!("Detected encoding: {:?}", detection.encoding);
    println!("MIME name: {}", detection.mime_name);
    println!("Is reliable: {}", detection.is_reliable);
    println!("Bytes consumed: {}", detection.bytes_consumed);
}
```

### With Hints

Hints can improve detection accuracy:

```rust
use compact_enc_det::{detect_encoding, DetectHints, Language, TextCorpusType};

let hints = DetectHints {
    url_hint: "https://example.com/page.html",
    http_charset_hint: "",
    meta_charset_hint: "utf-8",
    language_hint: Some(Language::ENGLISH),
    corpus_type: TextCorpusType::WEB_CORPUS,
    ..Default::default()
};

let detection = detect_encoding(data, hints);
```

### Working with Encodings

```rust
use compact_enc_det::Encoding;

let encoding = Encoding::UTF8;
println!("Raw value: {}", encoding.as_raw());

// Convert from raw value
let enc = Encoding::try_from(22).unwrap();
assert_eq!(enc, Encoding::UTF8);
```

## Architecture

This crate is split into two packages:

- **`compact-enc-det`**: High-level Rust API with ergonomic types
- **`compact-enc-det-sys`**: Low-level FFI bindings to the C++ library using [cxx](https://cxx.rs/)

## Building

### Prerequisites

- Rust 1.70 or later
- A C++ compiler (GCC, Clang, or MSVC)
- CMake (for building the C++ library)

The C++ library is included as a git submodule and is built automatically via `cxx-build`.

### Build from Source

```bash
git clone https://github.com/greenhat616/compact_enc_det_rs
cd compact_enc_det_rs
git submodule update --init --recursive
cargo build --release
```

### Run Tests

```bash
cargo test
```

## Supported Encodings

The library supports detection of 75+ encodings including:

- **Unicode**: UTF-8, UTF-16BE, UTF-16LE, UTF-32BE, UTF-32LE, UTF-7
- **Western European**: ISO-8859-1, ISO-8859-15, Windows-1252
- **Central European**: ISO-8859-2, Windows-1250, CP852
- **Cyrillic**: ISO-8859-5, Windows-1251, KOI8-R, KOI8-RU, CP866
- **Greek**: ISO-8859-7, Windows-1253
- **Turkish**: ISO-8859-9, Windows-1254
- **Hebrew**: ISO-8859-8, ISO-8859-8-I, Windows-1255
- **Arabic**: ISO-8859-6, Windows-1256
- **Chinese**: GB2312, GBK, GB18030, Big5, Big5-HKSCS, EUC-CN, HZ-GB-2312, ISO-2022-CN
- **Japanese**: Shift_JIS, EUC-JP, ISO-2022-JP, CP932
- **Korean**: EUC-KR, ISO-2022-KR
- **Thai**: ISO-8859-11, Windows-874, TIS-620
- **Vietnamese**: Windows-1258
- **Baltic**: ISO-8859-4, ISO-8859-13, Windows-1257
- And many more...

## Performance Considerations

For best detection accuracy, provide at least 4KB of sample text when possible. The library works with smaller samples but reliability may be lower.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The underlying C++ library ([google/compact_enc_det](https://github.com/google/compact_enc_det)) is licensed under the Apache License 2.0.

## Credits

- Original C++ library by Google: [google/compact_enc_det](https://github.com/google/compact_enc_det)
- Rust bindings by the compact-enc-det-rs contributors

## Related Projects

- [chardetng](https://crates.io/crates/chardetng) - Pure Rust character encoding detection
- [encoding_rs](https://crates.io/crates/encoding_rs) - Encoding conversion library
- [charset](https://crates.io/crates/charset) - Character set detection and conversion

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
