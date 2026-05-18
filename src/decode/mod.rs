//! LZMA2 decoder — Phase 1 wrapper around `lzma-rust2`.
//!
//! The 7z LZMA2 codec stores a 1-byte properties blob that encodes the
//! dictionary size. This module decodes that props byte and calls
//! `lzma_rust2::Lzma2Reader` to decompress the packed stream.

use std::io::Read;

use lzma_rust2::Lzma2Reader;

use crate::error::{XzippyError, XzippyResult};

/// Decode the 7z LZMA2 properties byte into a dictionary size in bytes.
///
/// # Encoding (from 7z SDK `LZMA2Dec.c`):
/// - `b == 40` → `dict_size = 0xFFFF_FFFF`
/// - `b < 40`  → `dict_size = (2 | (b & 1)) << ((b >> 1) + 11)`
///
/// # Errors
/// Returns [`XzippyError::InvalidProperties`] if `b > 40`.
pub fn props_byte_to_dict_size(b: u8) -> XzippyResult<u32> {
    if b == 40 {
        return Ok(u32::MAX);
    }
    if b > 40 {
        return Err(XzippyError::InvalidProperties(b));
    }
    Ok((2u32 | (b as u32 & 1)) << ((b as u32 >> 1) + 11))
}

/// Decompress raw LZMA2 data using a 7z-style 1-byte props blob.
///
/// `props_bytes` must be exactly 1 byte: the LZMA2 properties byte
/// that encodes the dictionary size (see [`props_byte_to_dict_size`]).
///
/// # Errors
/// Returns an error if `props_bytes` is wrong length, props byte is invalid,
/// or the stream is corrupt.
pub fn decode_7z(
    input: &[u8],
    props_bytes: &[u8],
    _uncompressed_size: u64,
) -> XzippyResult<Vec<u8>> {
    if props_bytes.len() != 1 {
        return Err(XzippyError::Backend(format!(
            "LZMA2 expects exactly 1 props byte, got {}",
            props_bytes.len()
        )));
    }
    let dict_size = props_byte_to_dict_size(props_bytes[0])?;
    let mut reader = Lzma2Reader::new(input, dict_size, None);
    let mut out = Vec::new();
    reader
        .read_to_end(&mut out)
        .map_err(|e| XzippyError::Backend(e.to_string()))?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn props_byte_zero_gives_4kib() {
        assert_eq!(props_byte_to_dict_size(0).unwrap(), 4096);
    }

    #[test]
    fn props_byte_12_gives_256kib() {
        assert_eq!(props_byte_to_dict_size(12).unwrap(), 262144);
    }

    #[test]
    fn props_byte_40_gives_u32_max() {
        assert_eq!(props_byte_to_dict_size(40).unwrap(), u32::MAX);
    }

    #[test]
    fn props_byte_41_is_invalid() {
        assert!(props_byte_to_dict_size(41).is_err());
    }
}
