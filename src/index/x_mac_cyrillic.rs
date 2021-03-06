// AUTOGENERATED FROM index-x-mac-cyrillic.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index-x-mac-cyrillic.txt see the Encoding Standard
// http://encoding.spec.whatwg.org/

static FORWARD_TABLE: &'static [u16] = &[
    1040, 1041, 1042, 1043, 1044, 1045, 1046, 1047, 1048, 1049, 1050, 1051,
    1052, 1053, 1054, 1055, 1056, 1057, 1058, 1059, 1060, 1061, 1062, 1063,
    1064, 1065, 1066, 1067, 1068, 1069, 1070, 1071, 8224, 176, 1168, 163, 167,
    8226, 182, 1030, 174, 169, 8482, 1026, 1106, 8800, 1027, 1107, 8734, 177,
    8804, 8805, 1110, 181, 1169, 1032, 1028, 1108, 1031, 1111, 1033, 1113,
    1034, 1114, 1112, 1029, 172, 8730, 402, 8776, 8710, 171, 187, 8230, 160,
    1035, 1115, 1036, 1116, 1109, 8211, 8212, 8220, 8221, 8216, 8217, 247,
    8222, 1038, 1118, 1039, 1119, 8470, 1025, 1105, 1103, 1072, 1073, 1074,
    1075, 1076, 1077, 1078, 1079, 1080, 1081, 1082, 1083, 1084, 1085, 1086,
    1087, 1088, 1089, 1090, 1091, 1092, 1093, 1094, 1095, 1096, 1097, 1098,
    1099, 1100, 1101, 1102, 8364,
];

#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[code as uint]
}

#[inline]
pub fn backward(code: u16) -> u8 {
    match code {
        1040 => 0, 1041 => 1, 1042 => 2, 1043 => 3, 1044 => 4, 1045 => 5,
        1046 => 6, 1047 => 7, 1048 => 8, 1049 => 9, 1050 => 10, 1051 => 11,
        1052 => 12, 1053 => 13, 1054 => 14, 1055 => 15, 1056 => 16, 1057 => 17,
        1058 => 18, 1059 => 19, 1060 => 20, 1061 => 21, 1062 => 22, 1063 => 23,
        1064 => 24, 1065 => 25, 1066 => 26, 1067 => 27, 1068 => 28, 1069 => 29,
        1070 => 30, 1071 => 31, 8224 => 32, 176 => 33, 1168 => 34, 163 => 35,
        167 => 36, 8226 => 37, 182 => 38, 1030 => 39, 174 => 40, 169 => 41,
        8482 => 42, 1026 => 43, 1106 => 44, 8800 => 45, 1027 => 46, 1107 => 47,
        8734 => 48, 177 => 49, 8804 => 50, 8805 => 51, 1110 => 52, 181 => 53,
        1169 => 54, 1032 => 55, 1028 => 56, 1108 => 57, 1031 => 58, 1111 => 59,
        1033 => 60, 1113 => 61, 1034 => 62, 1114 => 63, 1112 => 64, 1029 => 65,
        172 => 66, 8730 => 67, 402 => 68, 8776 => 69, 8710 => 70, 171 => 71,
        187 => 72, 8230 => 73, 160 => 74, 1035 => 75, 1115 => 76, 1036 => 77,
        1116 => 78, 1109 => 79, 8211 => 80, 8212 => 81, 8220 => 82, 8221 => 83,
        8216 => 84, 8217 => 85, 247 => 86, 8222 => 87, 1038 => 88, 1118 => 89,
        1039 => 90, 1119 => 91, 8470 => 92, 1025 => 93, 1105 => 94, 1103 => 95,
        1072 => 96, 1073 => 97, 1074 => 98, 1075 => 99, 1076 => 100,
        1077 => 101, 1078 => 102, 1079 => 103, 1080 => 104, 1081 => 105,
        1082 => 106, 1083 => 107, 1084 => 108, 1085 => 109, 1086 => 110,
        1087 => 111, 1088 => 112, 1089 => 113, 1090 => 114, 1091 => 115,
        1092 => 116, 1093 => 117, 1094 => 118, 1095 => 119, 1096 => 120,
        1097 => 121, 1098 => 122, 1099 => 123, 1100 => 124, 1101 => 125,
        1102 => 126, 8364 => 127, _ => 255
    }
}

#[cfg(test)]
mod tests {
    use super::{forward, backward};

    #[test]
    fn test_correct_table() {
        for i in range(0u8, 128) {
            let j = forward(i);
            if j != 0xffff { assert_eq!(backward(j), i); }
        }
    }
}
