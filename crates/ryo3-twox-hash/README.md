# `ryo3-twox-hash`

ryo3-wrapper for `twox-hash` crate

[//]: # "<GENERATED>"

## Ref

- docs.rs: [docs.rs/twox-hash](https://docs.rs/twox-hash)
- crates: [crates.io/crates/twox-hash](https://crates.io/crates/twox-hash)

[//]: # "</GENERATED>"

---

## LEGACY ONE SHOT FUNCTION SIGNATURES

```python
xxh128 = xxh3_128

# -----------------------------------------------------------------------------
# ONE-SHOT FUNCTIONS
# -----------------------------------------------------------------------------

# xxh32
def xxh32_digest(data: Buffer, *, seed: int = 0) -> bytes: ...
def xxh32_hexdigest(data: Buffer, *, seed: int = 0) -> str: ...
def xxh32_intdigest(data: Buffer, *, seed: int = 0) -> int: ...

# xxh64
def xxh64_digest(data: Buffer, *, seed: int = 0) -> bytes: ...
def xxh64_hexdigest(data: Buffer, *, seed: int = 0) -> str: ...
def xxh64_intdigest(data: Buffer, *, seed: int = 0) -> int: ...

# xxh3
def xxh3_64_digest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> bytes: ...
def xxh3_64_intdigest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> int: ...
def xxh3_64_hexdigest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> str: ...

# xxh128
def xxh3_128_digest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> bytes: ...
def xxh3_128_intdigest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> int: ...
def xxh3_128_hexdigest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> str: ...

# xxh128
def xxh128_digest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> bytes: ...
def xxh128_hexdigest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> str: ...
def xxh128_intdigest(
    data: Buffer, *, seed: int = 0, secret: Buffer | None = None
) -> int: ...
```
