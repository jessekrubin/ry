from __future__ import annotations

from hypothesis import given
from hypothesis import strategies as st

import ry

MIN_I64 = -(2**63)
MAX_I64 = (2**63) - 1
MIN_U64 = 0
MAX_U64 = (2**64) - 1


@given(
    s=st.integers(min_value=MIN_I64, max_value=MAX_I64),
    i=st.integers(),
)
def test_mul_ints(s, i: int) -> None:
    s = ry.Size(s)
    r = s * i
    assert r == s * i


@given(
    s=st.integers(min_value=MIN_I64, max_value=MAX_I64),
    i=st.floats(),
)
def test_mul_floats(s, i: int) -> None:
    s = ry.Size(s)
    r = s * i
    assert r == s * i
