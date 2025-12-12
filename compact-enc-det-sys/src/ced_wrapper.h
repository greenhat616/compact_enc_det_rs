#pragma once
#include <cstdint>
#include "rust/cxx.h"

struct CedResult;

CedResult ced_detect_encoding(
    rust::Slice<const std::uint8_t> bytes,
    rust::Str url_hint,
    rust::Str http_charset_hint,
    rust::Str meta_charset_hint,
    std::int32_t encoding_hint,
    std::int32_t language_hint,
    std::int32_t corpus_type,
    bool ignore_7bit_mail_encodings);