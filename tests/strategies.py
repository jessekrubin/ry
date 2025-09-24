from __future__ import annotations

import datetime as pydt
import zoneinfo
from functools import lru_cache
from typing import Any, Final

from hypothesis import strategies as st
from hypothesis.strategies import SearchStrategy

import ry

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
MAX_U128: Final = (1 << 128) - 1  # 340_282_366_920_938_463_463_374_607_431_768_211_455

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
st_u8 = st.integers(min_value=ry.U8_MIN, max_value=ry.U8_MAX)
st_u16 = st.integers(min_value=ry.U16_MIN, max_value=ry.U16_MAX)
st_u32 = st.integers(min_value=ry.U32_MIN, max_value=ry.U32_MAX)
st_u64 = st.integers(min_value=ry.U64_MIN, max_value=ry.U64_MAX)
st_u128 = st.integers(min_value=ry.U128_MIN, max_value=ry.U128_MAX)
# signed ─────────────────────────────────────────────────────────────
st_i8 = st.integers(min_value=ry.I8_MIN, max_value=ry.I8_MAX)
st_i16 = st.integers(min_value=ry.I16_MIN, max_value=ry.I16_MAX)
st_i32 = st.integers(min_value=ry.I32_MIN, max_value=ry.I32_MAX)
st_i64 = st.integers(min_value=ry.I64_MIN, max_value=ry.I64_MAX)
st_i128 = st.integers(min_value=ry.I128_MIN, max_value=ry.I128_MAX)


def st_durations(
    *,
    min_value: ry.Duration = ry.Duration.MIN,
    max_value: ry.Duration = ry.Duration.MAX,
) -> SearchStrategy[ry.Duration]:
    """Strategy for `ry.Duration` instances"""
    if not isinstance(min_value, ry.Duration):
        msg = f"min_value must be a ry.Duration, got {type(min_value)}"
        raise TypeError(msg)
    if not isinstance(max_value, ry.Duration):
        msg = f"max_value must be a ry.Duration, got {type(max_value)}"
        raise TypeError(msg)
    if min_value > max_value:
        emsg = f"min_value {min_value} must be <= max_value {max_value}"
        raise ValueError(emsg)
    if min_value == max_value:
        return st.just(min_value)
    if min_value == ry.Duration.MIN and max_value == ry.Duration.MAX:
        return st.builds(
            ry.Duration,
            st.integers(min_value=0, max_value=ry.U64_MAX),
            st.integers(min_value=0, max_value=999_999_999),
        )
    return st.builds(
        ry.Duration,
        st.integers(min_value=0, max_value=ry.U64_MAX),
        st.integers(min_value=0, max_value=999_999_999),
    ).filter(lambda d: min_value <= d <= max_value)


JsonSearchStrategy = SearchStrategy[
    list[Any]
    | dict[str, Any]
    | bool
    | int
    | float
    | str
    | None
    | pydt.time
    | pydt.date
    | pydt.datetime
]


def st_json(
    *,
    finite_only: bool = True,
    min_int: int | None = None,
    max_int: int | None = None,
    datetimes: bool = False,
) -> JsonSearchStrategy:
    """Helper function to describe JSON objects, with optional inf and nan.

    Taken from hypothesis docs

    REF: https://hypothesis.readthedocs.io/en/latest/tutorial/custom-strategies.html#writing-helper-functions
    """
    numbers = st.floats(allow_infinity=not finite_only, allow_nan=not finite_only)
    if datetimes:
        return st.recursive(
            st.none()
            | st.booleans()
            | st.integers(min_value=min_int, max_value=max_int)
            | numbers
            | st.text()
            | st.datetimes()
            | st.dates()
            | st.times(),
            extend=lambda xs: st.lists(xs) | st.dictionaries(st.text(), xs),
        )

    return st.recursive(
        st.none()
        | st.booleans()
        | st.integers(min_value=min_int, max_value=max_int)
        | numbers
        | st.text(),
        extend=lambda xs: st.lists(xs) | st.dictionaries(st.text(), xs),
    )


def st_json_js(
    *, finite_only: bool = True, datetimes: bool = False
) -> JsonSearchStrategy:
    """Helper function to describe JSON strings, with optional inf and nan."""
    return st_json(
        datetimes=datetimes,
        finite_only=finite_only,
        max_int=9_007_199_254_740_991,
        min_int=-9_007_199_254_740_991,
    )


@lru_cache(maxsize=1)
def _ok_timezone_names() -> set[str]:
    """Get a set of valid timezone names."""
    # zoneinfo.available_timezones() returns a set of valid timezone names
    # that can be used with zoneinfo.ZoneInfo
    return {el for el in zoneinfo.available_timezones() if el != "build/etc/localtime"}


def st_timezones(*, no_cache: bool = False) -> SearchStrategy[zoneinfo.ZoneInfo]:
    # weird aliases are super (fucking) annoying and totally not useful
    # unless your hair is too long need a trim

    def _filterfn(tz: zoneinfo.ZoneInfo) -> bool:
        """Filter function to ensure only valid timezones are returned."""
        return str(tz) in _ok_timezone_names()

    return st.timezones(no_cache=no_cache).filter(_filterfn)
