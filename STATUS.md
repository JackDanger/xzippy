# lazippier STATUS

**Current focus:** Phase 1 wrapper complete (lzma-rust2 backend). Phase 2: native LZMA2 chunk-orchestration impl.

| Piece | Status |
|---|---|
| props byte decode | ✅ |
| encoder (wrapper) | ✅ (lzma-rust2 backend) |
| decoder (wrapper) | ✅ (lzma-rust2 backend) |
| round-trip tests | ✅ |
| oracle (round-trip vs 7zz) | ✅ (via 7zippy layer5_cross) |
| streaming | ⬜ |
| multi-chunk support | ⬜ (Phase 2) |
| decode bench | ✅ |
| encode bench | ✅ |
| fuzz | ⬜ |

**Phase 1 backend:** `lzma-rust2 v0.16` (pure-Rust LZMA2 `Lzma2Writer` + `Lzma2Reader`).
**Phase 2:** Replace with lazippier's own chunk-orchestration implementation.

Symbols: ⬜ not started, 🟡 in progress, ✅ done, ❌ blocked.
