//! Deterministic test data generators.
//!
//! These generators produce reproducible byte sequences suitable for
//! compression round-trip tests. All use a simple LCG so there are no
//! external dependencies.

/// Generate `n` zero bytes.
pub fn zeros(n: usize) -> Vec<u8> {
    vec![0u8; n]
}

/// Generate `n` bytes with values 0, 1, 2, … (mod 256).
pub fn sequential(n: usize) -> Vec<u8> {
    (0..n).map(|i| (i & 0xFF) as u8).collect()
}

/// Generate `n` pseudo-random bytes from a fixed seed (LCG, seed=42).
pub fn random_seeded(n: usize) -> Vec<u8> {
    let mut state: u64 = 42;
    (0..n)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 33) as u8
        })
        .collect()
}

/// Generate `n` bytes that alternate between compressible runs and random data.
pub fn mixed(n: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(n);
    let mut rand_state: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let mut i = 0usize;
    while out.len() < n {
        // 64-byte compressible run
        let run_len = (64).min(n - out.len());
        out.extend(std::iter::repeat_n(
            b'A'.wrapping_add((i & 0x1F) as u8),
            run_len,
        ));
        i += 1;
        if out.len() >= n {
            break;
        }
        // 32-byte random chunk
        let rand_len = (32).min(n - out.len());
        for _ in 0..rand_len {
            rand_state = rand_state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            out.push((rand_state >> 33) as u8);
        }
    }
    out.truncate(n);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_length() {
        assert_eq!(zeros(64).len(), 64);
        assert!(zeros(64).iter().all(|&b| b == 0));
    }

    #[test]
    fn sequential_length_and_values() {
        let s = sequential(256);
        assert_eq!(s.len(), 256);
        assert_eq!(s[0], 0);
        assert_eq!(s[255], 255);
    }

    #[test]
    fn random_seeded_is_deterministic() {
        assert_eq!(random_seeded(8), random_seeded(8));
    }

    #[test]
    fn mixed_length() {
        assert_eq!(mixed(1024).len(), 1024);
    }
}
