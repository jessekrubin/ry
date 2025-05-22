from typing import Final

from hypothesis import strategies as st

# unsigned ──────────────────────────────────────────────────────────
MIN_U8: Final = 0
MAX_U8: Final = (1 << 8) - 1  # 255

MIN_U16: Final = 0
MAX_U16: Final = (1 << 16) - 1  # 65_535

MIN_U32: Final = 0
MAX_U32: Final = (1 << 32) - 1  # 4_294_967_295

MIN_U64: Final = 0
MAX_U64: Final = (1 << 64) - 1  # 18_446_744_073_709_551_615

MIN_U128: Final = 0
MAX_U128: Final = (
    1 << 128
) - 1  # 340_282_366_841_710_656_408_393_487_639_999_999_999_999_999_999_999_999_999_999

# signed ────────────────────────────────────────────────────────────
MIN_I8: Final = -(1 << 7)  # -128
MAX_I8: Final = (1 << 7) - 1  # 127

MIN_I16: Final = -(1 << 15)  # -32_768
MAX_I16: Final = (1 << 15) - 1  # 32_767

MIN_I32: Final = -(1 << 31)  # -2_147_483_648
MAX_I32: Final = (1 << 31) - 1  # 2_147_483_647

MIN_I64: Final = -(1 << 63)  # -9_223_372_036_854_775_808
MAX_I64: Final = (1 << 63) - 1  # 9_223_372_036_854_775_807

MIN_I128: Final = -(1 << 127)  # -170_141_183_460_469_231_731_687_303_715_884_105_728
MAX_I128: Final = (1 << 127) - 1  # 170_141_183_460_469_231_731_687_303_715_884_105_727

# unsigned ────────────────────────────────────────────────────────────
st_u8 = st.integers(min_value=MIN_U8, max_value=MAX_U8)
st_u16 = st.integers(min_value=MIN_U16, max_value=MAX_U16)
st_u32 = st.integers(min_value=MIN_U32, max_value=MAX_U32)
st_u64 = st.integers(min_value=MIN_U64, max_value=MAX_U64)
st_u128 = st.integers(min_value=MIN_U128, max_value=MAX_U128)
# signed ─────────────────────────────────────────────────────────────
st_i8 = st.integers(min_value=MIN_I8, max_value=MAX_I8)
st_i16 = st.integers(min_value=MIN_I16, max_value=MAX_I16)
st_i32 = st.integers(min_value=MIN_I32, max_value=MAX_I32)
st_i64 = st.integers(min_value=MIN_I64, max_value=MAX_I64)
st_i128 = st.integers(min_value=MIN_I128, max_value=MAX_I128)
