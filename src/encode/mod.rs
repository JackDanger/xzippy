//! LZMA2 encoder — Phase 1 wrapper around `lzma-rust2`.
//!
//! Encodes a raw LZMA2 stream (no XZ/lzip framing) and returns the compressed
//! bytes plus the 1-byte 7z properties blob encoding the dictionary size.

use std::io::Write;

use lzma_rust2::{Lzma2Options, Lzma2Writer, LzmaOptions};

use crate::error::{LazippierError, LazippierResult};

/// Encode the dictionary size as a 7z LZMA2 properties byte.
///
/// Finds the smallest `b` such that `props_byte_to_dict_size(b) >= dict_size`.
/// Returns `40` (meaning `UINT32_MAX`) if no smaller value fits.
pub fn dict_size_to_props_byte(dict_size: u32) -> u8 {
    for b in 0u8..40 {
        let candidate = ((2u32 | (b as u32 & 1)) << ((b as u32 >> 1) + 11)) as u32;
        if candidate >= dict_size {
            return b;
        }
    }
    40
}

/// Compress bytes using raw LZMA2 and return `(props_bytes, compressed_bytes)`.
///
/// `props_bytes` is a 1-element `Vec<u8>` with the LZMA2 properties byte
/// (the dictionary-size encoding that 7z stores as the coder properties).
/// `compressed_bytes` is the raw LZMA2 chunk stream.
///
/// # Errors
/// Returns an error if compression fails.
pub fn encode_7z(input: &[u8], dict_size: u32) -> LazippierResult<(Vec<u8>, Vec<u8>)> {
    let props_byte = dict_size_to_props_byte(dict_size);

    let opts = Lzma2Options {
        lzma_options: {
            let mut o = LzmaOptions::with_preset(6);
            o.dict_size = dict_size;
            o
        },
        ..Lzma2Options::with_preset(6)
    };

    let mut compressed = Vec::new();
    {
        let mut writer = Lzma2Writer::new(&mut compressed, opts);
        writer
            .write_all(input)
            .map_err(|e| LazippierError::Backend(e.to_string()))?;
        writer
            .finish()
            .map_err(|e| LazippierError::Backend(e.to_string()))?;
    }

    Ok((vec![props_byte], compressed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::{decode_7z, props_byte_to_dict_size};

    #[test]
    fn dict_size_to_props_byte_256k() {
        // 262144 = 256 KiB -> props_byte should be 12
        assert_eq!(dict_size_to_props_byte(262144), 12);
    }

    #[test]
    fn dict_size_to_props_byte_round_trips() {
        for b in 0u8..=40 {
            let d = props_byte_to_dict_size(b).unwrap();
            // dict_size_to_props_byte should recover b (or the next-larger b)
            let b2 = dict_size_to_props_byte(d);
            let d2 = props_byte_to_dict_size(b2).unwrap();
            assert_eq!(d, d2, "round-trip failed for b={b}");
        }
    }

    #[test]
    fn encode_decode_round_trip() {
        let input = b"Hello, LZMA2 world! This is a round-trip test.";
        let (props, compressed) = encode_7z(input, 262144).unwrap();
        let decompressed = decode_7z(&compressed, &props, input.len() as u64).unwrap();
        assert_eq!(decompressed, input);
    }
}
