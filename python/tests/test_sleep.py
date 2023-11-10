import pytest
import ry
import time

def test_sleep():
    start = time.time()
    res = ry.sleep(1)
    end = time.time()
    assert res >= 1
    assert isinstance(res, float)
    assert end - start >= 1

@pytest.mark.asyncio
async def test_sleep_async():
    start = time.time()
    res = await ry.sleep_async(1)
    end = time.time()
    # is float
    assert isinstance(res, float)
    assert res >= 1
    assert end - start >= 1
