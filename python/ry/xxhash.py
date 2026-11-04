"""ry.xxhash ~ `xxhash` (pypi) compat re-exports"""

from ry.ryo3 import xxh3_64, xxh3_128, xxh32, xxh64

xxh32_digest = xxh32.oneshot
xxh32_hexdigest = xxh32.oneshot_hex
xxh32_intdigest = xxh32.oneshot_int

xxh64_digest = xxh64.oneshot
xxh64_hexdigest = xxh64.oneshot_hex
xxh64_intdigest = xxh64.oneshot_int

xxh3_64_digest = xxh3_64.oneshot
xxh3_64_hexdigest = xxh3_64.oneshot_hex
xxh3_64_intdigest = xxh3_64.oneshot_int

xxh3_128_digest = xxh3_128.oneshot
xxh3_128_hexdigest = xxh3_128.oneshot_hex
xxh3_128_intdigest = xxh3_128.oneshot_int

# `xxhash` aliases xxh3_128 as xxh128
xxh128 = xxh3_128
xxh128_digest = xxh3_128.oneshot
xxh128_hexdigest = xxh3_128.oneshot_hex
xxh128_intdigest = xxh3_128.oneshot_int

__all__ = (
    "xxh3_64",
    "xxh3_64_digest",
    "xxh3_64_hexdigest",
    "xxh3_64_intdigest",
    "xxh3_128",
    "xxh3_128_digest",
    "xxh3_128_hexdigest",
    "xxh3_128_intdigest",
    "xxh32",
    "xxh32_digest",
    "xxh32_hexdigest",
    "xxh32_intdigest",
    "xxh64",
    "xxh64_digest",
    "xxh64_hexdigest",
    "xxh64_intdigest",
    "xxh128",
    "xxh128_digest",
    "xxh128_hexdigest",
    "xxh128_intdigest",
)
