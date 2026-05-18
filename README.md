# xzippy

Pure-Rust `.xz` format (LZMA2) encoder/decoder. Drop-in replacement for `xz`, `unxz`, and `xzcat`.
Part of the [7zippy](https://github.com/JackDanger/7zippy) umbrella of pure-Rust compression tools.

LZMA2 extends LZMA with multi-chunk streaming and optional uncompressed chunk passthrough.
It is the primary compression method used in modern `.xz` files and 7z archives.

See [STATUS.md](./STATUS.md) for the current implementation state.

## Build & Test

```sh
cargo build
cargo test
cargo bench --no-run   # verify bench targets compile
```

## 7z Properties byte

7z stores a single 1-byte properties blob for LZMA2 that encodes the dictionary size:

- `b == 40` → `dict_size = 0xFFFF_FFFF`
- `b < 40` → `dict_size = (2 | (b & 1)) << ((b >> 1) + 11)`
