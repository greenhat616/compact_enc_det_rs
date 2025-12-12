# compact-enc-det-sys

[![Crates.io](https://img.shields.io/crates/v/compact-enc-det-sys.svg)](https://crates.io/crates/compact-enc-det-sys)
[![Documentation](https://docs.rs/compact-enc-det-sys/badge.svg)](https://docs.rs/compact-enc-det-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../LICENSE)

Low-level FFI bindings to the [Compact Encoding Detection (CED)](https://github.com/google/compact_enc_det) C++ library.

## Overview

This crate provides unsafe FFI bindings to Google's Compact Encoding Detection library using [cxx](https://cxx.rs/). For most use cases, you should use the high-level [`compact-enc-det`](https://crates.io/crates/compact-enc-det) crate instead, which provides a safe and ergonomic Rust API.

## When to Use This Crate

Use `compact-enc-det-sys` directly only if you need:

- Direct access to the C++ API
- Custom build configurations
- To build your own higher-level wrapper

For normal encoding detection tasks, use the [`compact-enc-det`](https://crates.io/crates/compact-enc-det) crate.

## Building

This crate uses `cxx` and `cxx-build` to generate bindings and compile the C++ library. The C++ source code is included as a git submodule.

### Prerequisites

- Rust 1.70+
- A C++ compiler (GCC, Clang, or MSVC)
- CMake (optional, for standalone C++ builds)

### Build Process

The build happens automatically via `build.rs`:

1. The C++ library is compiled from source in `libs/compact_enc_det`
2. CXX generates the Rust-C++ bridge code
3. Everything is linked together

## Usage

```rust
use compact_enc_det_sys::ced_detect_encoding;

let text = b"Hello, world!";
let result = ced_detect_encoding(
    text,
    "",    // url_hint
    "",    // http_charset_hint
    "",    // meta_charset_hint
    -1,    // encoding_hint
    -1,    // language_hint
    2,     // corpus_type (QUERY_CORPUS)
    true,  // ignore_7bit_mail_encodings
);

println!("Encoding: {}", result.encoding);
println!("MIME name: {}", result.mime_name);
println!("Is reliable: {}", result.is_reliable);
```

## API

### `ced_detect_encoding`

```rust
pub fn ced_detect_encoding(
    bytes: &[u8],
    url_hint: &str,
    http_charset_hint: &str,
    meta_charset_hint: &str,
    encoding_hint: i32,
    language_hint: i32,
    corpus_type: i32,
    ignore_7bit_mail_encodings: bool,
) -> CedResult
```

### `CedResult`

```rust
pub struct CedResult {
    pub mime_name: String,
    pub encoding: i32,
    pub bytes_consumed: i32,
    pub is_reliable: bool,
}
```

## License

MIT License - see [LICENSE](../LICENSE) file for details.

The underlying C++ library ([google/compact_enc_det](https://github.com/google/compact_enc_det)) is licensed under the Apache License 2.0.

## See Also

- [`compact-enc-det`](https://github.com/google/compact_enc_det) - Compact Encoding Detection (CED) C++ library
- [CXX](https://cxx.rs/) - Safe interop between Rust and C++
