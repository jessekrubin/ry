from __future__ import annotations

import pytest
from hypothesis import given
from hypothesis import strategies as st

import ry

MIN_I64 = -(2**63)
MAX_I64 = (2**63) - 1
MIN_U64 = 0
MAX_U64 = (2**64) - 1


@given(
    si=st.integers(min_value=MIN_I64, max_value=MAX_I64),
    i=st.integers(),
)
def test_mul_ints(si: int, i: int) -> None:
    should_overflow = False
    try:
        _actual = si * i
        if _actual < MIN_I64 or _actual > MAX_I64:
            should_overflow = True
    except OverflowError:
        should_overflow = True

    if should_overflow:
        with pytest.raises(OverflowError):
            _ = ry.Size(si) * i
    else:
        s = ry.Size(si)
        r = s * i
        assert r == s * i


def is_nan_or_inf(value: float) -> bool:
    return value != value or value == float("inf") or value == float("-inf")


@given(
    si=st.integers(min_value=MIN_I64, max_value=MAX_I64),
    f=st.floats(),
)
def test_mul_floats(si: int, f: float) -> None:
    should_overflow = False
    try:
        _ = si * f
        if _ < MIN_I64 or _ > MAX_I64:
            should_overflow = True
    except OverflowError:
        should_overflow = True

    if is_nan_or_inf(f) and si != 0:
        should_overflow = True

    if should_overflow:
        with pytest.raises(OverflowError):
            _ = ry.Size(si) * f
    else:
        s = ry.Size(si)
        r = s * f
        assert r == s * f
