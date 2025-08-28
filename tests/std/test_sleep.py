from __future__ import annotations

import time

import pytest

import ry


def test_sleep() -> None:
    start = time.time()
    res = ry.sleep(0)
    end = time.time()
    assert res >= 0
    assert isinstance(res, float)
    assert end - start >= 0


def test_sleep_value_error_negative_number() -> None:
    with pytest.raises(ValueError):
        ry.sleep(-1)


def test_sleep_overflow_error_super_big_number() -> None:
    max_u64 = 2**64
    with pytest.raises(OverflowError):
        ry.sleep(max_u64)
