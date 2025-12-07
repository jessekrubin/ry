"""Example o' xxhash-ing files in this directory both async/sync

Demonstrates:
  - files are read via `ry.read_stream`/`ry.read_stream_async` in chunks
  - xxhashing is done via `ry.xxhash.xxh64` hasher
  - files are found via `ry.glob`
  - timing is done via `ry.instant()` to compute `ry.Duration`
  - timing is friendly printed w/ `{duration:#}` (see `ry.Duration.__format__`)

"""

import asyncio

import ry
from ry.xxhash import xxh64

_PWD = ry.FsPath(__file__).resolve().parent


def hash_file_sync(path: ry.FsPath) -> tuple[str, str]:
    hasher = xxh64()

    for chunk in ry.read_stream(path):
        hasher.update(chunk)
    return str(path), hasher.hexdigest()


def hash_examples_sync() -> list[tuple[str, str]]:
    files = ry.glob(str(_PWD / "**" / "*.py"), dtype=ry.FsPath).collect()

    return [hash_file_sync(f) for f in files]


async def hash_file_async(path: ry.FsPath) -> tuple[str, str]:
    hasher = xxh64()

    async for chunk in ry.read_stream_async(path):
        hasher.update(chunk)
    return str(path), hasher.hexdigest()


async def hash_examples_async() -> list[tuple[str, str]]:
    files = ry.glob(str(_PWD / "**" / "*.py"), dtype=ry.FsPath).collect()

    return await asyncio.gather(*(hash_file_async(f) for f in files))


def main_sync() -> tuple[list[tuple[str, str]], ry.Duration]:
    start = ry.instant()
    hahses_sync = hash_examples_sync()
    dt = start.elapsed()
    return hahses_sync, dt


async def main_async() -> tuple[list[tuple[str, str]], ry.Duration]:
    start = ry.instant()
    hahses_async = await hash_examples_async()
    dt = start.elapsed()
    return hahses_async, dt


def main() -> None:
    res_sync, dt_sync = main_sync()
    res_async, dt_async = asyncio.run(main_async())
    print(f"HASHING  (SYNC) TOOK: {dt_sync:#}")
    print(f"HASHING (ASYNC) TOOK: {dt_async:#}")
    assert sorted(res_sync) == sorted(res_async)
    print("SYNC/ASYNC RESULTS MATCH!")


if __name__ == "__main__":
    main_sync()

    asyncio.run(main_async())
