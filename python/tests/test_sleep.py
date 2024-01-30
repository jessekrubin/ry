import time

import pytest

import ry


def test_sleep():
    start = time.time()
    res = ry.sleep(0)
    end = time.time()
    assert res >= 0
    assert isinstance(res, float)
    assert end - start >= 0


@pytest.mark.asyncio
async def test_sleep_async():
    start = time.time()
    res = await ry.sleep_async(0)
    end = time.time()
    # is float
    assert isinstance(res, float)
    assert res >= 0
    assert end - start >= 0
