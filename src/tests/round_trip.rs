//! Round-trip tests for lazippier encode + decode.

use super::fixtures;

/// Verify a round-trip through lazippier's encode_7z / decode_7z.
fn assert_round_trip(input: &[u8], dict_size: u32) {
    let (props, compressed) = crate::encode::encode_7z(input, dict_size)
        .unwrap_or_else(|e| panic!("encode failed: {e}"));
    let decompressed = crate::decode::decode_7z(&compressed, &props, input.len() as u64)
        .unwrap_or_else(|e| panic!("decode failed: {e}"));
    assert_eq!(
        decompressed, input,
        "round-trip mismatch for {} bytes",
        input.len()
    );
}

#[test]
fn round_trip_empty() {
    assert_round_trip(&[], 262144);
}

#[test]
fn round_trip_small() {
    assert_round_trip(b"Hello, LZMA2!", 262144);
}

#[test]
fn round_trip_zeros_1kib() {
    assert_round_trip(&fixtures::zeros(1024), 262144);
}

#[test]
fn round_trip_sequential_4kib() {
    assert_round_trip(&fixtures::sequential(4096), 262144);
}

#[test]
fn round_trip_random_64kib() {
    assert_round_trip(&fixtures::random_seeded(65536), 262144);
}

#[test]
fn round_trip_mixed_128kib() {
    assert_round_trip(&fixtures::mixed(131072), 262144);
}
