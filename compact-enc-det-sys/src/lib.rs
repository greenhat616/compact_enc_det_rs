#[cxx::bridge]
#[allow(clippy::too_many_arguments)]
mod ffi {
    #[derive(Debug, Clone, PartialEq)]
    pub struct CedResult {
        mime_name: String,
        encoding: i32,
        bytes_consumed: i32,
        is_reliable: bool,
    }

    unsafe extern "C++" {
        include!("ced_wrapper.h");

        fn ced_detect_encoding(
            bytes: &[u8],
            url_hint: &str,
            http_charset_hint: &str,
            meta_charset_hint: &str,
            encoding_hint: i32,
            language_hint: i32,
            corpus_type: i32,
            ignore_7bit_mail_encodings: bool,
        ) -> CedResult;
    }
}

pub use ffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    use encoding_rs::{EUC_KR, Encoding, GB18030, SHIFT_JIS, UTF_8, WINDOWS_1251};

    const MIN_PAYLOAD_BYTES: usize = 4 * 1024 + 256;
    // CompactEncDet::TextCorpusType::QUERY_CORPUS in compact_enc_det.h
    const QUERY_CORPUS: i32 = 2;
    // Encoding ids from util/encodings/encodings.pb.h
    const ENC_UTF8: i32 = 22;
    const ENC_GB2312: i32 = 14;
    const ENC_SHIFT_JIS: i32 = 11;
    const ENC_WINDOWS_1251: i32 = 26;
    const ENC_EUC_KR: i32 = 16;

    struct TestCase {
        name: &'static str,
        encoding: &'static Encoding,
        expected_mime: &'static str,
        expected_id: i32,
        sample: &'static str,
    }

    fn build_payload(sample: &str, encoding: &'static Encoding) -> Vec<u8> {
        // Repeat the sample until the encoded payload is comfortably above 4 KiB.
        let mut repeats = (MIN_PAYLOAD_BYTES / sample.len()).max(2) + 1;
        loop {
            let mut text = String::with_capacity(sample.len() * repeats);
            for _ in 0..repeats {
                text.push_str(sample);
            }

            let (encoded, actual_encoding, had_errors) = encoding.encode(&text);
            assert!(
                !had_errors,
                "无法用 {} 编码样本文本（{}）",
                encoding.name(),
                sample
            );
            assert!(
                std::ptr::eq(actual_encoding, encoding),
                "编码检测用到的实际编码 {:?} 与预期 {:?} 不一致",
                actual_encoding.name(),
                encoding.name()
            );

            let bytes = encoded.into_owned();
            if bytes.len() >= MIN_PAYLOAD_BYTES {
                return bytes;
            }

            // 如果仍未满足长度要求，再多重复一次。
            repeats += 1;
        }
    }

    fn run_case(case: &TestCase) {
        let payload = build_payload(case.sample, case.encoding);

        let result = ced_detect_encoding(&payload, "", "", "", -1, -1, QUERY_CORPUS, true);

        println!("result: {:?}", result);

        assert!(
            result.mime_name.eq_ignore_ascii_case(case.expected_mime),
            "{} 检测结果 mime_name = {}，期望 {}",
            case.name,
            result.mime_name,
            case.expected_mime
        );
        assert_eq!(
            result.encoding, case.expected_id,
            "{} 检测出的编码枚举不符",
            case.name
        );
    }

    #[test]
    fn detects_multiple_encodings_with_large_payloads() {
        let cases = [
            TestCase {
                name: "utf8_mixed",
                encoding: UTF_8,
                expected_mime: "UTF-8",
                expected_id: ENC_UTF8,
                sample: "Rust 提供了安全与性能兼顾的编程体验，同时可以混合多种语言字符，例如中文、Русский текст、日本語、한국어，以及 emoji 😊🚀。",
            },
            TestCase {
                name: "GBK",
                encoding: GB18030,
                expected_mime: "GB2312",
                expected_id: ENC_GB2312,
                sample: "熟悉的车轴声，明媚的春日阳光沿着林荫之中的缝隙顽强地坠落在地面上，两侧森林中间的泥土路上布满了腐烂的落叶，发出难听的声音。
　　偶尔车轮会碰上一个小石头，整架廉价的运货马车马上热情响应，上下弹动，老旧的木头结构已经不太稳定，每一次弹跳都似乎令其距离宣布解体更近一分。
　　空气中弥散着一股芳草的清香，混合着泥土与树叶的气息，柔软如丝巾般的微风触摸着马车乘客的肌肤，不禁令人生出一丝惬意之感。
　　“唔！”躺在马车上的男人用力地弓起自己的腰身，好像一位破伤风病人，不受控制地抽搐着，震动着，肌肉紧绷，扭曲，他的头跟脚顶在不太结实的木板上，顶得车厢结构发出痛苦的声响。",
            },
            TestCase {
                name: "shift_jis_japanese",
                encoding: SHIFT_JIS,
                expected_mime: "Shift_JIS",
                expected_id: ENC_SHIFT_JIS,
                sample: "これは日本語で書かれた長い文章で、文字コード検出のテストを行います。東京の風景や四季の移ろい、開発に関する説明を繰り返します。",
            },
            TestCase {
                name: "windows_1251_russian",
                encoding: WINDOWS_1251,
                expected_mime: "windows-1251",
                expected_id: ENC_WINDOWS_1251,
                sample: "Это большой русский текст, который покрывает разные буквы алфавита,
                подробно описывает тестирование кодировок и многократно повторяет фразы,
                чтобы сделать данные длиннее.Это большой русский текст,
                который покрывает разные буквы алфавита,
                подробно описывает тестирование кодировок и многократно повторяет фразы,
                чтобы сделать данные длиннее.",
            },
            TestCase {
                name: "euc_kr_korean",
                encoding: EUC_KR,
                expected_mime: "EUC-KR",
                expected_id: ENC_EUC_KR,
                sample: "이 문장은 한국어 EUC-KR 인코딩으로 작성되었으며, 코드 검출 테스트를 위해 문화와 기술 이야기를 반복하여 길이를 늘립니다.",
            },
        ];

        for case in cases {
            run_case(&case);
        }
    }
}
