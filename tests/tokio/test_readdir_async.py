from __future__ import annotations

import os

import ry

PWD = os.path.dirname(os.path.abspath(__file__))


async def test_read_dir() -> None:
    items = os.listdir(PWD)

    async for direntry in await ry.read_dir_async(PWD):
        basename = os.path.basename(direntry)
        assert basename in items
        metadata = await direntry.metadata
        assert isinstance(metadata, ry.Metadata)
        ftype = await direntry.file_type
        assert isinstance(ftype, ry.FileType)
        assert isinstance(direntry.basename, str)

    collected_dir_entries = await (await ry.read_dir_async(PWD)).collect()
    collected_paths = {os.path.basename(direntry) for direntry in collected_dir_entries}
    assert collected_paths == set(items)


async def test_read_dir_take() -> None:
    items = os.listdir(PWD)

    readdir_async = await ry.read_dir_async(PWD)
    take_two = []
    # take 2 at a time until we run out of items
    while True:
        taken = await readdir_async.take(2)

        if not taken:
            break

        else:
            take_two.extend(taken)

    take_two_paths = {os.path.basename(direntry) for direntry in take_two}
    assert take_two_paths == set(items)
