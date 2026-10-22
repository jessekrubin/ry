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


@pytest.mark.anyio
@pytest.mark.parametrize("sleep_fn", [ry.asleep, ry.sleep_async])
async def test_async_sleep_can_be_cancelled(
    sleep_fn: t.Callable[[float], t.Coroutine[t.Any, t.Any, float]],
) -> None:
    start = time.monotonic()
    task = asyncio.create_task(sleep_fn(60))
    await asyncio.sleep(0)
    task.cancel()

    with pytest.raises(asyncio.CancelledError):
        await task
    assert time.monotonic() - start < 5


@pytest.mark.anyio
async def test_async_sleep_wait_for_timeout() -> None:
    start = time.monotonic()
    with pytest.raises(asyncio.TimeoutError):
        await asyncio.wait_for(ry.asleep(60), 0.01)
    assert time.monotonic() - start < 5


@pytest.mark.parametrize(
    ("sleep_fn", "name"), [(ry.asleep, "asleep"), (ry.sleep_async, "sleep_async")]
)
def test_async_sleep_is_coroutine(
    sleep_fn: t.Callable[[float], t.Coroutine[t.Any, t.Any, float]], name: str
) -> None:
    coro: t.Any = sleep_fn(0)
    assert type(coro).__name__ == "Coroutine"
    assert coro.__qualname__ == name
    assert coro.__name__ == name
    assert asyncio.iscoroutine(coro)
    coro.close()


@pytest.mark.anyio
@pytest.mark.parametrize("sleep_fn", [ry.asleep, ry.sleep_async])
@pytest.mark.parametrize(
    ("secs", "exc"),
    [
        (-1.0, ValueError),
        (float("-inf"), ValueError),
        (float("nan"), ValueError),
        (float("inf"), OverflowError),
        (1e300, OverflowError),
    ],
)
async def test_async_sleep_bad_secs(
    sleep_fn: t.Callable[[float], t.Coroutine[t.Any, t.Any, float]],
    secs: float,
    exc: type[Exception],
) -> None:
    with pytest.raises(exc):
        await sleep_fn(secs)
