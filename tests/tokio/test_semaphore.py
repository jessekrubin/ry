from __future__ import annotations

import asyncio

import pytest

import ry


def test_semaphore_argument_validation() -> None:
    with pytest.raises(ValueError, match="value must be >= 1"):
        ry.Semaphore(0)

    sem = ry.Semaphore(1)
    with pytest.raises(ValueError, match="n must be >= 1"):
        sem.release(0)


@pytest.mark.anyio
async def test_semaphore_acquire_release_and_context_manager() -> None:
    sem = ry.Semaphore(2)

    assert sem.value == 2
    assert sem.available_permits() == 2
    assert not sem.locked()

    await sem.acquire()
    assert sem.value == 1
    assert sem.available_permits() == 1
    assert not sem.locked()

    async with sem:
        assert sem.value == 0
        assert sem.available_permits() == 0
        assert sem.locked()

    assert sem.value == 1
    assert sem.available_permits() == 1
    assert not sem.locked()

    sem.release()
    assert sem.value == 2
    assert sem.available_permits() == 2


@pytest.mark.anyio
async def test_semaphore_waits_for_release() -> None:
    sem = ry.Semaphore(1)
    waiter_started = asyncio.Event()
    waiter_acquired = asyncio.Event()

    await sem.acquire()

    async def waiter() -> None:
        waiter_started.set()
        await sem.acquire()
        waiter_acquired.set()

    task = asyncio.create_task(waiter())
    await waiter_started.wait()
    await asyncio.sleep(0.01)

    assert not waiter_acquired.is_set()
    assert sem.locked()

    sem.release()
    await asyncio.wait_for(waiter_acquired.wait(), timeout=1)

    assert sem.locked()
    task.cancel()


@pytest.mark.anyio
async def test_semaphore_close_errors_pending_and_future_acquires() -> None:
    sem = ry.Semaphore(1)

    await sem.acquire()
    pending_acquire = asyncio.create_task(sem.acquire())
    await asyncio.sleep(0.01)

    sem.close()

    assert sem.is_closed()
    with pytest.raises(RuntimeError, match="Semaphore is closed"):
        await pending_acquire

    with pytest.raises(RuntimeError, match="Semaphore is closed"):
        await sem.acquire()
