#include "compact-enc-det-sys/src/lib.rs.h"

#include <string>

#include "compact_enc_det/compact_enc_det.h"
#include "util/encodings/encodings.h"
#include "util/languages/languages.pb.h" // UNKNOWN_LANGUAGE / enum Language
#include "util/encodings/encodings.pb.h" // UNKNOWN_ENCODING / enum Encoding

CedResult ced_detect_encoding(
    rust::Slice<const std::uint8_t> bytes,
    rust::Str url_hint,
    rust::Str http_charset_hint,
    rust::Str meta_charset_hint,
    std::int32_t encoding_hint,
    std::int32_t language_hint,
    std::int32_t corpus_type,
    bool ignore_7bit_mail_encodings)
{

    std::string url(url_hint.data(), url_hint.size());
    std::string http(http_charset_hint.data(), http_charset_hint.size());
    std::string meta(meta_charset_hint.data(), meta_charset_hint.size());

    const char *url_ptr = url.empty() ? nullptr : url.c_str();
    const char *http_ptr = http.empty() ? nullptr : http.c_str();
    const char *meta_ptr = meta.empty() ? nullptr : meta.c_str();

    if (encoding_hint < 0)
        encoding_hint = UNKNOWN_ENCODING;
    if (language_hint < 0)
        language_hint = UNKNOWN_LANGUAGE;

    int bytes_consumed = 0;
    bool is_reliable = false;

    auto enc = CompactEncDet::DetectEncoding(
        reinterpret_cast<const char *>(bytes.data()),
        static_cast<int>(bytes.size()),
        url_ptr,
        http_ptr,
        meta_ptr,
        static_cast<int>(encoding_hint),
        static_cast<Language>(language_hint),
        static_cast<CompactEncDet::TextCorpusType>(corpus_type),
        ignore_7bit_mail_encodings,
        &bytes_consumed,
        &is_reliable);

    const char *mime = MimeEncodingName(enc);

    CedResult out;
    out.mime_name = rust::String(mime ? mime : "");
    out.encoding = static_cast<std::int32_t>(enc);
    out.bytes_consumed = bytes_consumed;
    out.is_reliable = is_reliable;
    return out;
}
