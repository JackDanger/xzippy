//! xzippy — Pure-Rust `.xz` format (LZMA2). Drop-in for xz/unxz/xzcat. Part of the 7zippy umbrella.
//!
//! LZMA2 extends LZMA with multi-chunk streaming and optional uncompressed chunk passthrough.
//! The 7z codec method ID is `[0x21]` with a 1-byte properties blob encoding the dictionary size.
//!
//! ## Properties byte
//!
//! 7z stores a single properties byte `b` for LZMA2:
//!
//! - `b == 40` → `dict_size = 0xFFFF_FFFF`
//! - `b < 40` → `dict_size = (2 | (b & 1)) << ((b >> 1) + 11)`
//!
//! ## Phase 1 vs Phase 2
//!
//! Phase 1 wraps `lzma-rust2`'s `Lzma2Writer`/`Lzma2Reader`. Phase 2 will replace
//! these with xzippy's own native chunk-orchestration implementation.

#![deny(unsafe_op_in_unsafe_fn)]

pub mod decode;
pub mod encode;
pub mod error;

#[cfg(test)]
mod tests;
