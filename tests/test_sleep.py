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


@pytest.mark.parametrize("interval", [0, 1001])
def test_sleep_check_interval_err(interval: int) -> None:
    with pytest.raises(ValueError):
        ry.Duration(1, 1).sleep(interval=interval)


@pytest.mark.anyio
async def test_sleep_async() -> None:
    start = time.time()
    res = await ry.sleep_async(0)
    end = time.time()
    assert res >= 0
    assert end - start >= 0

    assert isinstance(res, float)


@pytest.mark.anyio
async def test_asleep() -> None:
    start = time.time()
    res = await ry.asleep(0)
    end = time.time()
    assert res >= 0
    assert end - start >= 0
    assert isinstance(res, float)
