# `ryo3-reqwest`

ryo3-wrapper for `reqwest`

Brotli response bodies are decoded incrementally with `burli-decode` for both
async and blocking clients, including response streams and SSE. `brotli=False`
disables negotiation and decoding of Brotli responses. Explicit `Accept-Encoding`
headers are preserved. Gzip, deflate, and Zstandard decoding remain in `reqwest`.

[//]: # "<GENERATED>"

## Ref

- docs.rs: [https://docs.rs/reqwest](https://docs.rs/reqwest)
- crates: [https://crates.io/crates/reqwest](https://crates.io/crates/reqwest)

[//]: # "</GENERATED>"
