// This is a part of rust-encoding.
// Copyright (c) 2013, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! An interface for retrieving an encoding (or a set of encodings) from a string label.

use std::ascii::StrAsciiExt;
use all;
use types;

/// Returns an encoding from given label, defined in the WHATWG Encoding standard, if any.
/// Implements "get an encoding" algorithm: http://encoding.spec.whatwg.org/#decode
pub fn encoding_from_whatwg_label(label: &str) -> Option<&'static types::Encoding> {
    match label.trim_chars(& &[' ', '\n', '\r', '\t', '\x0C']).to_ascii_lower().as_slice() {
        "unicode-1-1-utf-8" |
        "utf-8" |
        "utf8" =>
            Some(all::UTF_8 as &'static types::Encoding),
        "866" |
        "cp866" |
        "csibm866" |
        "ibm866" =>
            Some(all::IBM866 as &'static types::Encoding),
        "csisolatin2" |
        "iso-8859-2" |
        "iso-ir-101" |
        "iso8859-2" |
        "iso88592" |
        "iso_8859-2" |
        "iso_8859-2:1987" |
        "l2" |
        "latin2" =>
            Some(all::ISO_8859_2 as &'static types::Encoding),
        "csisolatin3" |
        "iso-8859-3" |
        "iso-ir-109" |
        "iso8859-3" |
        "iso88593" |
        "iso_8859-3" |
        "iso_8859-3:1988" |
        "l3" |
        "latin3" =>
            Some(all::ISO_8859_3 as &'static types::Encoding),
        "csisolatin4" |
        "iso-8859-4" |
        "iso-ir-110" |
        "iso8859-4" |
        "iso88594" |
        "iso_8859-4" |
        "iso_8859-4:1988" |
        "l4" |
        "latin4" =>
            Some(all::ISO_8859_4 as &'static types::Encoding),
        "csisolatincyrillic" |
        "cyrillic" |
        "iso-8859-5" |
        "iso-ir-144" |
        "iso8859-5" |
        "iso88595" |
        "iso_8859-5" |
        "iso_8859-5:1988" =>
            Some(all::ISO_8859_5 as &'static types::Encoding),
        "arabic" |
        "asmo-708" |
        "csiso88596e" |
        "csiso88596i" |
        "csisolatinarabic" |
        "ecma-114" |
        "iso-8859-6" |
        "iso-8859-6-e" |
        "iso-8859-6-i" |
        "iso-ir-127" |
        "iso8859-6" |
        "iso88596" |
        "iso_8859-6" |
        "iso_8859-6:1987" =>
            Some(all::ISO_8859_6 as &'static types::Encoding),
        "csisolatingreek" |
        "ecma-118" |
        "elot_928" |
        "greek" |
        "greek8" |
        "iso-8859-7" |
        "iso-ir-126" |
        "iso8859-7" |
        "iso88597" |
        "iso_8859-7" |
        "iso_8859-7:1987" |
        "sun_eu_greek" =>
            Some(all::ISO_8859_7 as &'static types::Encoding),
        "csiso88598e" |
        "csisolatinhebrew" |
        "hebrew" |
        "iso-8859-8" |
        "iso-8859-8-e" |
        "iso-ir-138" |
        "iso8859-8" |
        "iso88598" |
        "iso_8859-8" |
        "iso_8859-8:1988" |
        "visual" =>
            Some(all::ISO_8859_8 as &'static types::Encoding),
        "csiso88598i" |
        "iso-8859-8-i" |
        "logical" =>
            Some(all::whatwg::ISO_8859_8_I as &'static types::Encoding),
        "csisolatin6" |
        "iso-8859-10" |
        "iso-ir-157" |
        "iso8859-10" |
        "iso885910" |
        "l6" |
        "latin6" =>
            Some(all::ISO_8859_10 as &'static types::Encoding),
        "iso-8859-13" |
        "iso8859-13" |
        "iso885913" =>
            Some(all::ISO_8859_13 as &'static types::Encoding),
        "iso-8859-14" |
        "iso8859-14" |
        "iso885914" =>
            Some(all::ISO_8859_14 as &'static types::Encoding),
        "csisolatin9" |
        "iso-8859-15" |
        "iso8859-15" |
        "iso885915" |
        "iso_8859-15" |
        "l9" =>
            Some(all::ISO_8859_15 as &'static types::Encoding),
        "iso-8859-16" =>
            Some(all::ISO_8859_16 as &'static types::Encoding),
        "cskoi8r" |
        "koi" |
        "koi8" |
        "koi8-r" |
        "koi8_r" =>
            Some(all::KOI8_R as &'static types::Encoding),
        "koi8-u" =>
            Some(all::KOI8_U as &'static types::Encoding),
        "csmacintosh" |
        "mac" |
        "macintosh" |
        "x-mac-roman" =>
            Some(all::MACINTOSH as &'static types::Encoding),
        "dos-874" |
        "iso-8859-11" |
        "iso8859-11" |
        "iso885911" |
        "tis-620" |
        "windows-874" =>
            Some(all::WINDOWS_874 as &'static types::Encoding),
        "cp1250" |
        "windows-1250" |
        "x-cp1250" =>
            Some(all::WINDOWS_1250 as &'static types::Encoding),
        "cp1251" |
        "windows-1251" |
        "x-cp1251" =>
            Some(all::WINDOWS_1251 as &'static types::Encoding),
        "ansi_x3.4-1968" |
        "ascii" |
        "cp1252" |
        "cp819" |
        "csisolatin1" |
        "ibm819" |
        "iso-8859-1" |
        "iso-ir-100" |
        "iso8859-1" |
        "iso88591" |
        "iso_8859-1" |
        "iso_8859-1:1987" |
        "l1" |
        "latin1" |
        "us-ascii" |
        "windows-1252" |
        "x-cp1252" =>
            Some(all::WINDOWS_1252 as &'static types::Encoding),
        "cp1253" |
        "windows-1253" |
        "x-cp1253" =>
            Some(all::WINDOWS_1253 as &'static types::Encoding),
        "cp1254" |
        "csisolatin5" |
        "iso-8859-9" |
        "iso-ir-148" |
        "iso8859-9" |
        "iso88599" |
        "iso_8859-9" |
        "iso_8859-9:1989" |
        "l5" |
        "latin5" |
        "windows-1254" |
        "x-cp1254" =>
            Some(all::WINDOWS_1254 as &'static types::Encoding),
        "cp1255" |
        "windows-1255" |
        "x-cp1255" =>
            Some(all::WINDOWS_1255 as &'static types::Encoding),
        "cp1256" |
        "windows-1256" |
        "x-cp1256" =>
            Some(all::WINDOWS_1256 as &'static types::Encoding),
        "cp1257" |
        "windows-1257" |
        "x-cp1257" =>
            Some(all::WINDOWS_1257 as &'static types::Encoding),
        "cp1258" |
        "windows-1258" |
        "x-cp1258" =>
            Some(all::WINDOWS_1258 as &'static types::Encoding),
        "x-mac-cyrillic" |
        "x-mac-ukrainian" =>
            Some(all::X_MAC_CYRILLIC as &'static types::Encoding),
        "chinese" |
        "csgb2312" |
        "csiso58gb231280" |
        "gb2312" |
        "gb_2312" |
        "gb_2312-80" |
        "gbk" |
        "iso-ir-58" |
        "x-gbk" =>
            Some(all::GBK18030 as &'static types::Encoding),
        "gb18030" =>
            Some(all::GB18030 as &'static types::Encoding),
        /*
        "hz-gb-2312" =>
            Some(all::HZ_GB_2312 as &'static types::Encoding),
        "big5" |
        "big5-hkscs" |
        "cn-big5" |
        "csbig5" |
        "x-x-big5" =>
            Some(all::BIG5 as &'static types::Encoding),
        */
        "cseucpkdfmtjapanese" |
        "euc-jp" |
        "x-euc-jp" =>
            Some(all::EUC_JP as &'static types::Encoding),
        /*
        "csiso2022jp" |
        "iso-2022-jp" =>
            Some(all::ISO_2022_JP as &'static types::Encoding),
        */
        "csshiftjis" |
        "ms_kanji" |
        "shift-jis" |
        "shift_jis" |
        "sjis" |
        "windows-31j" |
        "x-sjis" =>
            Some(all::WINDOWS_31J as &'static types::Encoding),
        "cseuckr" |
        "csksc56011987" |
        "euc-kr" |
        "iso-ir-149" |
        "korean" |
        "ks_c_5601-1987" |
        "ks_c_5601-1989" |
        "ksc5601" |
        "ksc_5601" |
        "windows-949" =>
            Some(all::WINDOWS_949 as &'static types::Encoding),
        "csiso2022kr" |
        "iso-2022-kr" |
        "iso-2022-cn" |
        "iso-2022-cn-ext" =>
            Some(all::whatwg::REPLACEMENT as &'static types::Encoding),
        /*
        "utf-16be" =>
            Some(all::UTF_16BE as &'static types::Encoding),
        "utf-16" |
        "utf-16le" =>
            Some(all::UTF_16LE as &'static types::Encoding),
        */
        "x-user-defined" =>
            Some(all::whatwg::X_USER_DEFINED as &'static types::Encoding),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    extern mod extra;
    use super::encoding_from_whatwg_label;

    #[test]
    fn test_encoding_from_whatwg_label() {
        assert!(encoding_from_whatwg_label("utf-8").is_some())
        assert!(encoding_from_whatwg_label("UTF-8").is_some())
        assert!(encoding_from_whatwg_label("\t\n\x0C\r utf-8\t\n\x0C\r ").is_some())
        assert!(encoding_from_whatwg_label("\u00A0utf-8").is_none(),
                "Non-ASCII whitespace should not be trimmed")
        assert!(encoding_from_whatwg_label("greek").is_some())
        assert!(encoding_from_whatwg_label("gree\u212A").is_none(),
                "Case-insensitive matching should be ASCII only. Kelvin sign does not match k.")
    }

    #[bench]
    fn bench_encoding_from_whatwg_label(harness: &mut extra::test::BenchHarness) {
        do harness.iter() {
            encoding_from_whatwg_label("iso-8859-bazinga");
        }
    }
}

