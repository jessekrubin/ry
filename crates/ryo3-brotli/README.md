# `ryo3-brotli`

Brotli compression with `burli-encode` and decompression with `brotli`.

`ry.brotli_compress(data, level=5)` accepts compression levels `0` through `5`.
The deprecated `ry.brotli_encode` and `ry.brotli` aliases use the same encoder
and default quality. The `magic_number` keyword is no longer supported.

`ry.brotli_decompress(data)` continues to decode existing Brotli streams,
including streams encoded at levels `6` through `11` or with `magic_number=True`.

[//]: # "<GENERATED>"

## Ref

- docs.rs: [https://docs.rs/burli-encode](https://docs.rs/burli-encode)
- crates: [https://crates.io/crates/burli-encode](https://crates.io/crates/burli-encode)
- docs.rs: [https://docs.rs/brotli](https://docs.rs/brotli)
- crates: [https://crates.io/crates/brotli](https://crates.io/crates/brotli)

[//]: # "</GENERATED>"
