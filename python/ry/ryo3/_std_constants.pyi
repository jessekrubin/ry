"""ryo3-std ~ numeric constants"""

import typing as t

# ruff: noqa: PYI054
# u8
U8_BITS: t.Literal[8]
U8_MAX: t.Literal[255]
U8_MIN: t.Literal[0]
# i8
I8_BITS: t.Literal[8]
I8_MAX: t.Literal[127]
I8_MIN: t.Literal[-128]
# i16
I16_BITS: t.Literal[16]
I16_MAX: t.Literal[32_767]
I16_MIN: t.Literal[-32_768]
# u16
U16_BITS: t.Literal[16]
U16_MAX: t.Literal[65_535]
U16_MIN: t.Literal[0]

# u32
U32_BITS: t.Literal[32]
U32_MAX: t.Literal[4_294_967_295]
U32_MIN: t.Literal[0]

# i32
I32_BITS: t.Literal[32]
I32_MAX: t.Literal[2_147_483_647]
I32_MIN: t.Literal[-2_147_483_648]

# u64
U64_BITS: t.Literal[64]
U64_MAX: t.Literal[18_446_744_073_709_551_615]
U64_MIN: t.Literal[0]

# i64
I64_BITS: t.Literal[64]
I64_MAX: t.Literal[9_223_372_036_854_775_807]
I64_MIN: t.Literal[-9_223_372_036_854_775_808]

# u128
U128_BITS: t.Literal[128]
U128_MAX: t.Literal[340_282_366_920_938_463_463_374_607_431_768_211_455]
U128_MIN: t.Literal[0]

# i128
I128_BITS: t.Literal[128]
I128_MAX: t.Literal[170_141_183_460_469_231_731_687_303_715_884_105_727]
I128_MIN: t.Literal[-170_141_183_460_469_231_731_687_303_715_884_105_728]

# usize
USIZE_BITS: t.Literal[32, 64]
USIZE_MAX: t.Literal[4_294_967_295, 18_446_744_073_709_551_615]
USIZE_MIN: t.Literal[0]
# isize
ISIZE_BITS: t.Literal[32, 64]
ISIZE_MAX: t.Literal[2_147_483_647, 9_223_372_036_854_775_807]
ISIZE_MIN: t.Literal[-2_147_483_648, -9_223_372_036_854_775_808]
