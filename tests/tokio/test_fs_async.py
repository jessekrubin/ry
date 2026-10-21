"""`ry.*_async` fs functions (all `ry.Coroutine`s on the tokio rt)"""

from __future__ import annotations

import asyncio
import os
import sys
import typing as t
from pathlib import Path

import pytest

import ry

_TOKIO_FS_AIO_FNS: tuple[t.Callable[..., t.Any], ...] = (
    ry.canonicalize_async,
    ry.copy_async,
    ry.create_dir_all_async,
    ry.create_dir_async,
    ry.exists_async,
    ry.hard_link_async,
    ry.metadata_async,
    ry.read_async,
    ry.read_dir_async,
    ry.read_link_async,
    ry.read_to_string_async,
    ry.remove_dir_all_async,
    ry.remove_dir_async,
    ry.remove_file_async,
    ry.rename_async,
    ry.try_exists_async,
    ry.write_async,
)


def _build_fn_args(fn: t.Callable[..., t.Any], tmp_path: Path) -> tuple[t.Any, ...]:
    if fn is ry.write_async:
        return (tmp_path / "w", b"x")
    if fn in (ry.copy_async, ry.hard_link_async, ry.rename_async):
        return (tmp_path / "a", tmp_path / "b")
    return (tmp_path / "a",)


@pytest.mark.parametrize("fn", _TOKIO_FS_AIO_FNS, ids=lambda fn: fn.__name__)
def test_is_coroutine(fn: t.Callable[..., t.Any], tmp_path: Path) -> None:
    coro = fn(*_build_fn_args(fn, tmp_path))
    assert type(coro).__name__ == "Coroutine"
    assert asyncio.iscoroutine(coro)
    assert coro.__qualname__ == fn.__name__
    coro.close()


@pytest.mark.parametrize(
    "fn",
    [ry.write_async, ry.create_dir_async, ry.create_dir_all_async],
    ids=lambda fn: fn.__name__,
)
def test_lazy(fn: t.Callable[..., t.Any], tmp_path: Path) -> None:
    args = _build_fn_args(fn, tmp_path)
    coro = fn(*args)
    del coro
    assert not Path(args[0]).exists()


@pytest.mark.anyio
async def test_roundtrip(tmp_path: Path) -> None:
    a = tmp_path / "a.txt"
    assert await ry.write_async(a, b"hello") == 5
    assert await ry.read_async(a) == b"hello"
    assert await ry.read_to_string_async(a) == "hello"
    assert await ry.exists_async(a) is True
    assert await ry.try_exists_async(tmp_path / "nope") is False
    md = await ry.metadata_async(a)
    assert md.len == 5
    assert os.fspath(await ry.canonicalize_async(a)) == str(a.resolve())

    b = tmp_path / "b.txt"
    assert await ry.copy_async(a, b) == 5
    c = tmp_path / "c.txt"
    await ry.rename_async(b, c)
    assert not b.exists()
    assert c.read_bytes() == b"hello"
    d = tmp_path / "d.txt"
    await ry.hard_link_async(a, d)
    assert d.read_bytes() == b"hello"
    await ry.remove_file_async(c)
    assert not c.exists()

    nested = tmp_path / "x" / "y"
    await ry.create_dir_all_async(nested)
    assert nested.is_dir()
    await ry.create_dir_async(nested / "z")
    await ry.remove_dir_async(nested / "z")
    await ry.remove_dir_all_async(tmp_path / "x")
    assert not (tmp_path / "x").exists()

    names = sorted(
        e.filename for e in await (await ry.read_dir_async(tmp_path)).collect()
    )
    assert names == ["a.txt", "d.txt"]


@pytest.mark.skipif(sys.platform == "win32", reason="symlinks need privileges")
@pytest.mark.anyio
async def test_read_link(tmp_path: Path) -> None:
    (tmp_path / "a").write_bytes(b"")
    (tmp_path / "l").symlink_to(tmp_path / "a")
    assert os.fspath(await ry.read_link_async(tmp_path / "l")) == str(tmp_path / "a")


@pytest.mark.anyio
async def test_errors_raise(tmp_path: Path) -> None:
    with pytest.raises(FileNotFoundError):
        await ry.read_async(tmp_path / "nope")
    with pytest.raises(FileNotFoundError):
        await ry.metadata_async(tmp_path / "nope")
    with pytest.raises(FileExistsError):
        await ry.create_dir_async(tmp_path)


@pytest.mark.anyio
async def test_gather(tmp_path: Path) -> None:
    await asyncio.gather(
        *(ry.write_async(tmp_path / f"{i}.txt", str(i).encode()) for i in range(50))
    )
    got = await asyncio.gather(
        *(ry.read_to_string_async(tmp_path / f"{i}.txt") for i in range(50))
    )
    assert got == [str(i) for i in range(50)]
