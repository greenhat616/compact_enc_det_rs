use std::path::PathBuf;

fn main() {
    let ced = PathBuf::from("deps/compact_enc_det");

    cxx_build::bridge("src/lib.rs")
        .file("src/ced_wrapper.cc")
        .file(ced.join("compact_enc_det/compact_enc_det.cc"))
        .file(ced.join("compact_enc_det/compact_enc_det_hint_code.cc"))
        .file(ced.join("util/encodings/encodings.cc"))
        .file(ced.join("util/languages/languages.cc"))
        .include("src")
        .include(&ced)
        .std("c++17")
        .compile("compact-enc-det-sys");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ced_wrapper.h");
    println!("cargo:rerun-if-changed=src/ced_wrapper.cc");
}
