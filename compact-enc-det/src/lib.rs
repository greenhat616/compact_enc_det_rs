#![allow(non_camel_case_types)]

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidEnumValue {
    pub raw: i32,
}

impl fmt::Display for InvalidEnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid enum value: {}", self.raw)
    }
}

impl std::error::Error for InvalidEnumValue {}

macro_rules! define_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $variant:ident = $value:expr,
            )+
        }
    ) => {
        $(#[$meta])*
        #[repr(i32)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        $vis enum $name {
            $(
                $variant = $value,
            )+
        }

        impl $name {
            pub const fn as_raw(self) -> i32 {
                self as i32
            }
        }

        impl TryFrom<i32> for $name {
            type Error = InvalidEnumValue;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $value => Ok(Self::$variant),
                    )+
                    _ => Err(InvalidEnumValue { raw: value }),
                }
            }
        }
    };
}

define_enum! {
    /// `util/encodings/encodings.pb.h`
    pub enum Encoding {
        ISO_8859_1 = 0,
        ISO_8859_2 = 1,
        ISO_8859_3 = 2,
        ISO_8859_4 = 3,
        ISO_8859_5 = 4,
        ISO_8859_6 = 5,
        ISO_8859_7 = 6,
        ISO_8859_8 = 7,
        ISO_8859_9 = 8,
        ISO_8859_10 = 9,
        JAPANESE_EUC_JP = 10,
        JAPANESE_SHIFT_JIS = 11,
        JAPANESE_JIS = 12,
        CHINESE_BIG5 = 13,
        CHINESE_GB = 14,
        CHINESE_EUC_CN = 15,
        KOREAN_EUC_KR = 16,
        UNICODE = 17,
        CHINESE_EUC_DEC = 18,
        CHINESE_CNS = 19,
        CHINESE_BIG5_CP950 = 20,
        JAPANESE_CP932 = 21,
        UTF8 = 22,
        UNKNOWN_ENCODING = 23,
        ASCII_7BIT = 24,
        RUSSIAN_KOI8_R = 25,
        RUSSIAN_CP1251 = 26,
        MSFT_CP1252 = 27,
        RUSSIAN_KOI8_RU = 28,
        MSFT_CP1250 = 29,
        ISO_8859_15 = 30,
        MSFT_CP1254 = 31,
        MSFT_CP1257 = 32,
        ISO_8859_11 = 33,
        MSFT_CP874 = 34,
        MSFT_CP1256 = 35,
        MSFT_CP1255 = 36,
        ISO_8859_8_I = 37,
        HEBREW_VISUAL = 38,
        CZECH_CP852 = 39,
        CZECH_CSN_369103 = 40,
        MSFT_CP1253 = 41,
        RUSSIAN_CP866 = 42,
        ISO_8859_13 = 43,
        ISO_2022_KR = 44,
        GBK = 45,
        GB18030 = 46,
        BIG5_HKSCS = 47,
        ISO_2022_CN = 48,
        TSCII = 49,
        TAMIL_MONO = 50,
        TAMIL_BI = 51,
        JAGRAN = 52,
        MACINTOSH_ROMAN = 53,
        UTF7 = 54,
        BHASKAR = 55,
        HTCHANAKYA = 56,
        UTF16BE = 57,
        UTF16LE = 58,
        UTF32BE = 59,
        UTF32LE = 60,
        BINARYENC = 61,
        HZ_GB_2312 = 62,
        UTF8UTF8 = 63,
        TAM_ELANGO = 64,
        TAM_LTTMBARANI = 65,
        TAM_SHREE = 66,
        TAM_TBOOMIS = 67,
        TAM_TMNEWS = 68,
        TAM_WEBTAMIL = 69,
        KDDI_SHIFT_JIS = 70,
        DOCOMO_SHIFT_JIS = 71,
        SOFTBANK_SHIFT_JIS = 72,
        KDDI_ISO_2022_JP = 73,
        SOFTBANK_ISO_2022_JP = 74,
        NUM_ENCODINGS = 75,
    }
}

define_enum! {
    /// `util/languages/languages.pb.h`
    pub enum Language {
        ENGLISH = 0,
        DANISH = 1,
        DUTCH = 2,
        FINNISH = 3,
        FRENCH = 4,
        GERMAN = 5,
        HEBREW = 6,
        ITALIAN = 7,
        JAPANESE = 8,
        KOREAN = 9,
        NORWEGIAN = 10,
        POLISH = 11,
        PORTUGUESE = 12,
        RUSSIAN = 13,
        SPANISH = 14,
        SWEDISH = 15,
        CHINESE = 16,
        CZECH = 17,
        GREEK = 18,
        ICELANDIC = 19,
        LATVIAN = 20,
        LITHUANIAN = 21,
        ROMANIAN = 22,
        HUNGARIAN = 23,
        ESTONIAN = 24,
        TG_UNKNOWN_LANGUAGE = 25,
        UNKNOWN_LANGUAGE = 26,
        BULGARIAN = 27,
        CROATIAN = 28,
        SERBIAN = 29,
        IRISH = 30,
        GALICIAN = 31,
        TAGALOG = 32,
        TURKISH = 33,
        UKRAINIAN = 34,
        HINDI = 35,
        MACEDONIAN = 36,
        BENGALI = 37,
        INDONESIAN = 38,
        LATIN = 39,
        MALAY = 40,
        MALAYALAM = 41,
        WELSH = 42,
        NEPALI = 43,
        TELUGU = 44,
        ALBANIAN = 45,
        TAMIL = 46,
        BELARUSIAN = 47,
        JAVANESE = 48,
        OCCITAN = 49,
        URDU = 50,
        BIHARI = 51,
        GUJARATI = 52,
        THAI = 53,
        ARABIC = 54,
        CATALAN = 55,
        ESPERANTO = 56,
        BASQUE = 57,
        INTERLINGUA = 58,
        KANNADA = 59,
        PUNJABI = 60,
        SCOTS_GAELIC = 61,
        SWAHILI = 62,
        SLOVENIAN = 63,
        MARATHI = 64,
        MALTESE = 65,
        VIETNAMESE = 66,
        FRISIAN = 67,
        SLOVAK = 68,
        CHINESE_T = 69,
        FAROESE = 70,
        SUNDANESE = 71,
        UZBEK = 72,
        AMHARIC = 73,
        AZERBAIJANI = 74,
        GEORGIAN = 75,
        TIGRINYA = 76,
        PERSIAN = 77,
        BOSNIAN = 78,
        SINHALESE = 79,
        NORWEGIAN_N = 80,
        PORTUGUESE_P = 81,
        PORTUGUESE_B = 82,
        XHOSA = 83,
        ZULU = 84,
        GUARANI = 85,
        SESOTHO = 86,
        TURKMEN = 87,
        KYRGYZ = 88,
        BRETON = 89,
        TWI = 90,
        YIDDISH = 91,
        SERBO_CROATIAN = 92,
        SOMALI = 93,
        UIGHUR = 94,
        KURDISH = 95,
        MONGOLIAN = 96,
        ARMENIAN = 97,
        LAOTHIAN = 98,
        SINDHI = 99,
        RHAETO_ROMANCE = 100,
        AFRIKAANS = 101,
        LUXEMBOURGISH = 102,
        BURMESE = 103,
        KHMER = 104,
        TIBETAN = 105,
        DHIVEHI = 106,
        CHEROKEE = 107,
        SYRIAC = 108,
        LIMBU = 109,
        ORIYA = 110,
        ASSAMESE = 111,
        CORSICAN = 112,
        INTERLINGUE = 113,
        KAZAKH = 114,
        LINGALA = 115,
        MOLDAVIAN = 116,
        PASHTO = 117,
        QUECHUA = 118,
        SHONA = 119,
        TAJIK = 120,
        TATAR = 121,
        TONGA = 122,
        YORUBA = 123,
        CREOLES_AND_PIDGINS_ENGLISH_BASED = 124,
        CREOLES_AND_PIDGINS_FRENCH_BASED = 125,
        CREOLES_AND_PIDGINS_PORTUGUESE_BASED = 126,
        CREOLES_AND_PIDGINS_OTHER = 127,
        MAORI = 128,
        WOLOF = 129,
        ABKHAZIAN = 130,
        AFAR = 131,
        AYMARA = 132,
        BASHKIR = 133,
        BISLAMA = 134,
        DZONGKHA = 135,
        FIJIAN = 136,
        GREENLANDIC = 137,
        HAUSA = 138,
        HAITIAN_CREOLE = 139,
        INUPIAK = 140,
        INUKTITUT = 141,
        KASHMIRI = 142,
        KINYARWANDA = 143,
        MALAGASY = 144,
        NAURU = 145,
        OROMO = 146,
        RUNDI = 147,
        SAMOAN = 148,
        SANGO = 149,
        SANSKRIT = 150,
        SISWANT = 151,
        TSONGA = 152,
        TSWANA = 153,
        VOLAPUK = 154,
        ZHUANG = 155,
        KHASI = 156,
        SCOTS = 157,
        GANDA = 158,
        MANX = 159,
        MONTENEGRIN = 160,
        NUM_LANGUAGES = 161,
    }
}

define_enum! {
    /// `CompactEncDet::TextCorpusType` 
    pub enum TextCorpusType {
        WEB_CORPUS = 0,
        XML_CORPUS = 1,
        QUERY_CORPUS = 2,
        EMAIL_CORPUS = 3,
        NUM_CORPA = 4,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    pub mime_name: String,
    pub encoding: Encoding,
    pub bytes_consumed: usize,
    pub is_reliable: bool,
}

#[derive(Debug, Clone)]
pub struct DetectHints<'a> {
    pub url_hint: &'a str,
    pub http_charset_hint: &'a str,
    pub meta_charset_hint: &'a str,
    pub encoding_hint: Option<Encoding>,
    pub language_hint: Option<Language>,
    pub corpus_type: TextCorpusType,
    pub ignore_7bit_mail_encodings: bool,
}

impl<'a> Default for DetectHints<'a> {
    fn default() -> Self {
        Self {
            url_hint: "",
            http_charset_hint: "",
            meta_charset_hint: "",
            encoding_hint: None,
            language_hint: None,
            corpus_type: TextCorpusType::QUERY_CORPUS,
            ignore_7bit_mail_encodings: true,
        }
    }
}

pub fn detect_encoding(bytes: &[u8], hints: DetectHints<'_>) -> Detection {
    let encoding_hint = hints.encoding_hint.map(|e| e.as_raw()).unwrap_or(-1);
    let language_hint = hints.language_hint.map(|l| l.as_raw()).unwrap_or(-1);

    let result = compact_enc_det_sys::ced_detect_encoding(
        bytes,
        hints.url_hint,
        hints.http_charset_hint,
        hints.meta_charset_hint,
        encoding_hint,
        language_hint,
        hints.corpus_type.as_raw(),
        hints.ignore_7bit_mail_encodings,
    );

    Detection {
        mime_name: result.mime_name,
        encoding: Encoding::try_from(result.encoding).unwrap_or(Encoding::UNKNOWN_ENCODING),
        bytes_consumed: result.bytes_consumed as usize,
        is_reliable: result.is_reliable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_utf8_plain_text() {
        let payload = "Rust makes FFI safer. 编码检测 UTF-8 mixed text.".repeat(50);
        let detection = detect_encoding(payload.as_bytes(), DetectHints::default());

        assert_eq!(detection.encoding, Encoding::UTF8);
        assert!(detection.is_reliable);
        assert!(detection.bytes_consumed > 0);
    }
}
