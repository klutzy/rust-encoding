// This is a part of rust-encoding.
// Copyright (c) 2013, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Asymmetric or special encoding constructions required by the WHATWG Encoding standard.

use codec;
use types::*;

/// Replacement encoding used to solve a particular attack vector due to mismatching server and
/// client supports for encodings. It is rarely useful outside.
#[deriving(Clone)]
pub struct EncoderOnlyUTF8Encoding;

impl Encoding for EncoderOnlyUTF8Encoding {
    fn name(&self) -> &'static str { "encoder-only-utf-8" }
    fn whatwg_name(&self) -> Option<&'static str> { Some("replacement") } // WHATWG compatibility
    fn encoder(&self) -> ~Encoder { codec::utf_8::UTF8Encoding.encoder() }
    fn decoder(&self) -> ~Decoder { codec::error::ErrorEncoding.decoder() }
}

// indices for x-user-defined encoding
#[inline]
pub fn x_user_defined_forward(code: u8) -> u16 {
    0xf780 + (code as u16)
}

#[inline]
pub fn x_user_defined_backward(code: u16) -> u8 {
    if 0xf780 <= code && code <= 0xf7ff {(code - 0xf780) as u8} else {0xff}
}

