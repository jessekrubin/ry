from __future__ import annotations

import asyncio
import os
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from pathlib import Path

PWD = os.path.dirname(os.path.abspath(__file__))


async def test_read_dir() -> None:
    items = os.listdir(PWD)

    async for direntry in await ry.read_dir_async(PWD):
        basename = os.path.basename(direntry)
        assert basename in items
        metadata = await direntry.metadata()
        assert isinstance(metadata, ry.Metadata)
        ftype = await direntry.file_type()
        assert isinstance(ftype, ry.FileType)
        assert isinstance(direntry.filename, str)

    collected_dir_entries = await (await ry.read_dir_async(PWD)).collect()
    collected_paths = {os.path.basename(direntry) for direntry in collected_dir_entries}
    assert collected_paths == set(items)


async def test_read_dir_take() -> None:
    items = os.listdir(PWD)

    readdir_async = await ry.read_dir_async(PWD)
    take_two = []
    # take 2 at a time until out o' items
    while True:
        taken = await readdir_async.take(2)

        if not taken:
            break

        take_two.extend(taken)

    take_two_paths = {os.path.basename(direntry) for direntry in take_two}
    assert take_two_paths == set(items)


async def test_cancelled_read_dir_collect_releases_lock(tmp_path: Path) -> None:
    directory = os.fspath(tmp_path)
    for i in range(2_000):
        with open(os.path.join(directory, f"some-fucking-file-{i}"), "wb"):
            pass
    # gen create
    read_dir = await ry.read_dir_async(directory)
    # collect all the files (which should take a sec)
    task = asyncio.create_task(read_dir.collect())
    # oh shit take a nap
    await asyncio.sleep(0)
    # fuckit no need to nap
    task.cancel()

    # MUST RAISE
    with pytest.raises(asyncio.CancelledError):
        await task

    await asyncio.wait_for(read_dir.take(1), timeout=1)


async def test_read_dir_methods_are_coroutines() -> None:
    read_dir = await ry.read_dir_async(PWD)
    entry = await anext(read_dir)
    coros = {
        "AsyncReadDir.__anext__": anext(read_dir),
        "AsyncReadDir.collect": read_dir.collect(),
        "AsyncReadDir.take": read_dir.take(),
        "AsyncDirEntry.metadata": entry.metadata(),
        "AsyncDirEntry.file_type": entry.file_type(),
    }
    for qualname, coro in coros.items():
        assert asyncio.iscoroutine(coro)
        assert coro.__qualname__ == qualname
        coro.close()


async def test_read_dir_exhausted(tmp_path: Path) -> None:
    (tmp_path / "a").write_bytes(b"")
    read_dir = await ry.read_dir_async(tmp_path)
    assert [e.filename async for e in read_dir] == ["a"]
    with pytest.raises(StopAsyncIteration):
        await anext(read_dir)
    assert await read_dir.take(3) == []
    assert await read_dir.collect() == []
