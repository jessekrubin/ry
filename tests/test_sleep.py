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


@pytest.mark.asyncio
async def test_sleep_async() -> None:
    start = time.time()
    res = await ry.sleep_async(0)
    end = time.time()
    # is float
    assert res >= 0
    assert end - start >= 0

    # assert isinstance(res, float)


@pytest.mark.asyncio
async def test_asleep() -> None:
    start = time.time()
    res = await ry.asleep(0)
    end = time.time()
    # is float
    assert res >= 0
    assert end - start >= 0

    # assert isinstance(res, float)
