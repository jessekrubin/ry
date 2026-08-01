from __future__ import annotations

import asyncio
import re
import time
import typing as t

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
    with pytest.raises(
        ValueError,
        match=re.escape("interval must be in the range 1..=1000 milliseconds"),
    ):
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


@pytest.mark.skipif(
    not ry.__pyo3_experimental_async__,
    reason="coroutine cancel requires `experimental-async` feat",
)
@pytest.mark.anyio
@pytest.mark.parametrize("sleep_fn", [ry.asleep, ry.sleep_async])
async def test_async_sleep_can_be_cancelled(
    sleep_fn: t.Callable[[float], t.Coroutine[t.Any, t.Any, float]],
) -> None:
    task = asyncio.create_task(sleep_fn(60))
    await asyncio.sleep(0)
    task.cancel()

    with pytest.raises(asyncio.CancelledError):
        await task
