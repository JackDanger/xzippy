# xzippy

> **Part of the [7-zippy](https://github.com/JackDanger/7zippy) family** — pure-Rust compression tooling.
> Full suite: `cargo add sevenzippy`  |  This crate: `cargo add xzippy`

Pure-Rust `.xz` format (LZMA2) encoder/decoder and drop-in replacement for
`xz`, `unxz`, and `xzcat`.

LZMA2 extends LZMA with multi-chunk streaming and optional uncompressed chunk
passthrough. It is the primary compression method in both `.xz` files and 7z
archives. In Phase 2, xzippy's core LZMA engine will be provided by
[lazippy](https://github.com/JackDanger/lazippy); xzippy adds only the LZMA2
chunk framing and `.xz` container format on top.

## Install

```bash
cargo install xzippy
```

## Use as a library

```toml
[dependencies]
xzippy = "0.1.0"
```

## Build & Test

```sh
cargo build
cargo test
cargo bench --no-run   # verify bench targets compile
```

## 7z properties byte

7z stores a single-byte properties blob for LZMA2 that encodes the dictionary size:

- `b == 40` → `dict_size = 0xFFFF_FFFF`
- `b < 40`  → `dict_size = (2 | (b & 1)) << ((b >> 1) + 11)`

See [STATUS.md](./STATUS.md) for the current implementation state.
