from hypothesis import strategies as st
from hypothesis.strategies import SearchStrategy

MIN_U8 = 0
MAX_U8 = (2**8) - 1  # 255
MIN_U16 = 0
MAX_U16 = (2**16) - 1  # 65_535
MIN_U32 = 0
MAX_U32 = (2**32) - 1  # 4_294_967_295
MIN_U64 = 0
MAX_U64 = (2**64) - 1  # 18_446_744_073_709_551_615

MIN_I8 = -(2**7)  # -128
MAX_I8 = (2**7) - 1
MIN_I16 = -(2**15)
MAX_I16 = (2**15) - 1
MIN_I32 = -(2**31)
MAX_I32 = (2**31) - 1
MIN_I64 = -(2**63)
MAX_I64 = (2**63) - 1


# returns a strategy for "normal" numbers
def st_u8() -> SearchStrategy[int]:
    """Strategy for unsigned 8-bit integers."""
    return st.integers(min_value=MIN_U8, max_value=MAX_U8)


def st_i8() -> SearchStrategy[int]:
    """Strategy for signed 8-bit integers."""
    return st.integers(min_value=MIN_I8, max_value=MAX_I8)


def st_u16() -> SearchStrategy[int]:
    """Strategy for unsigned 16-bit integers."""
    return st.integers(min_value=MIN_U16, max_value=MAX_U16)


def st_i16() -> SearchStrategy[int]:
    """Strategy for signed 16-bit integers."""
    return st.integers(min_value=MIN_I16, max_value=MAX_I16)


def st_u32() -> SearchStrategy[int]:
    """Strategy for unsigned 32-bit integers."""
    return st.integers(min_value=MIN_U32, max_value=MAX_U32)


def st_i32() -> SearchStrategy[int]:
    """Strategy for signed 32-bit integers."""
    return st.integers(min_value=MIN_I32, max_value=MAX_I32)


def st_u64() -> SearchStrategy[int]:
    """Strategy for unsigned 64-bit integers."""
    return st.integers(min_value=MIN_U64, max_value=MAX_U64)


def st_i64() -> SearchStrategy[int]:
    """Strategy for signed 64-bit integers."""
    return st.integers(min_value=MIN_I64, max_value=MAX_I64)
