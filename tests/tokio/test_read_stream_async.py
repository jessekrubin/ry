from __future__ import annotations

import asyncio
import os
import re
import sys
import types
import typing as t
from pathlib import Path

import pytest

import ry

_THIS_FILEPATH_ABSOLUTE = Path(__file__).resolve()
_DEFAULT_READ_SIZE = 65_536


@pytest.mark.parametrize("buffered", [True, False])
async def test_async_read_stream(tmp_path: Path, *, buffered: bool) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    p = tmp_path / "test.txt"
    string = "\n".join([str(i) for i in range(1000)])
    string_bytes = string.encode()
    with open(p, "wb") as f:
        f.write(string_bytes)
    ry.cd(tmp_path)
    chunks = await ry.read_stream_async(
        "test.txt", read_size=10, buffered=buffered
    ).collect()
    assert b"".join(chunks) == string_bytes
    assert len(chunks) == len(string_bytes) // 10 + 1

    chunks_built = [
        chunk
        async for chunk in ry.read_stream_async(
            "test.txt", read_size=10, buffered=buffered
        )
    ]
    assert b"".join(chunks_built) == string_bytes
    assert len(chunks_built) == len(string_bytes) // 10 + 1

    # with offset

    chunks = await ry.read_stream_async(
        "test.txt", read_size=10, offset=100, buffered=buffered
    ).collect()
    assert b"".join(chunks) == string_bytes[100:]
    assert len(chunks) == len(string_bytes[100:]) // 10 + 1


@pytest.mark.parametrize("buffered", [True, False])
async def test_async_read_stream_str(tmp_path: Path, *, buffered: bool) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    p = tmp_path / "test.txt"
    string = "\n".join([str(i) for i in range(1000)])
    string_bytes = string.encode()
    with open(p, "wb") as f:
        f.write(string_bytes)
    ry.cd(tmp_path)
    stream = await ry.read_stream_async("test.txt", read_size=10, buffered=buffered)
    uno = await stream.take()
    dos = await stream.take(2)
    restante = await stream.collect()
    assert len(uno) == 1
    assert uno == [b"0\n1\n2\n3\n4\n"]
    assert dos == [b"5\n6\n7\n8\n9\n", b"10\n11\n12\n1"]
    assert len(restante) == 386


@pytest.mark.parametrize("buffered", [True, False])
async def test_async_read_stream_huge_read_size_and_take(
    tmp_path: Path, *, buffered: bool
) -> None:
    p = tmp_path / "test.txt"
    string_bytes = "\n".join([str(i) for i in range(1000)]).encode()
    p.write_bytes(string_bytes)

    chunks = await ry.read_stream_async(p, read_size=2**62, buffered=buffered).collect()
    assert chunks == [string_bytes]

    taken = await ry.read_stream_async(p, read_size=10, buffered=buffered).take(2**62)
    assert b"".join(taken) == string_bytes
    assert len(taken) == len(string_bytes) // 10 + 1


async def test_read_stream_file_not_found(tmp_path: Path) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        _rs = await ry.read_stream_async("test.txt", read_size=10)


async def test_read_stream_is_directory(tmp_path: Path) -> None:
    """Test that reading a directory raises an error."""
    ry.cd(tmp_path)
    (tmp_path / "test").mkdir(parents=True)
    with pytest.raises(OSError):  # noqa: PT011
        async for _ in ry.read_stream_async(tmp_path, read_size=10):
            pass


@pytest.mark.parametrize("strict", [True, False])
async def test_read_offset_greater_than_file_size(
    tmp_path: Path, *, strict: bool
) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    p = tmp_path / "test.txt"
    string = "123"
    string_bytes = string.encode()
    with open(p, "wb") as f:
        size = f.write(string_bytes)
    ry.cd(tmp_path)
    read_offset = size + 1
    if strict:
        with pytest.raises(
            ValueError,
            match=re.escape(
                f"offset ({read_offset}) is beyond end of file (len = {size})"
            ),
        ):
            _ = await ry.read_stream_async(
                "test.txt", offset=read_offset, strict=strict
            )
    else:
        rs = await ry.read_stream_async("test.txt", offset=read_offset, strict=strict)
        chunks = await rs.collect()
        assert b"".join(chunks) == b""
        assert len(chunks) == 0


class _FileReadStreamOptionsDict(t.TypedDict, total=False):
    read_size: int
    offset: int
    buffered: bool
    strict: bool


@pytest.mark.parametrize("read_size", [None, 1, 100, 512, 1024])
@pytest.mark.parametrize("offset", [None, 0, 1])
@pytest.mark.parametrize("buffered", [None, True, False])
@pytest.mark.parametrize("strict", [None, True, False])
class TestAsyncFileReadStream:
    def _setup_method(self, tmp_path: Path) -> bytes:
        p = tmp_path / "test.txt"
        string_bytes = "\n".join([str(i) for i in range(1000)]).encode()
        p.write_bytes(string_bytes)
        ry.cd(tmp_path)
        return string_bytes

    def _build_kwargs(
        self,
        read_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> _FileReadStreamOptionsDict:
        kwargs: _FileReadStreamOptionsDict = {}
        if read_size is not None:
            kwargs["read_size"] = read_size
        if offset is not None:
            kwargs["offset"] = offset
        if buffered is not None:
            kwargs["buffered"] = buffered
        if strict is not None:
            kwargs["strict"] = strict
        return kwargs

    async def test_async_file_readstream_collect(
        self,
        tmp_path: Path,
        read_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> None:
        string_bytes = self._setup_method(tmp_path)
        stream = await ry.read_stream_async(
            "test.txt",
            **self._build_kwargs(
                read_size,
                offset,
                buffered=buffered,
                strict=strict,
            ),
        )
        # test collect
        collected = await stream.collect()
        assert b"".join(collected) == string_bytes[offset or 0 :]

    async def test_async_file_readstream_take(
        self,
        tmp_path: Path,
        read_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> None:
        string_bytes = self._setup_method(tmp_path)
        stream = await ry.read_stream_async(
            "test.txt",
            **self._build_kwargs(
                read_size,
                offset,
                buffered=buffered,
                strict=strict,
            ),
        )
        first_five = await stream.take(5)
        expected_bytes_range = slice(
            offset or 0, (offset or 0) + sum(len(c) for c in first_five)
        )
        expected_bytes = string_bytes[expected_bytes_range]
        assert b"".join(first_five) == expected_bytes
        assert len(first_five) <= 5

    async def test_async_file_readstream_repr(
        self,
        tmp_path: Path,
        read_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> None:
        p = tmp_path / "test.txt"
        string = "hello world"
        string_bytes = string.encode()
        with open(p, "wb") as f:
            f.write(string_bytes)
        ry.cd(tmp_path)
        kwargs = self._build_kwargs(
            read_size,
            offset,
            buffered=buffered,
            strict=strict,
        )

        stream = await ry.read_stream_async("test.txt", **kwargs)
        repr_str = repr(stream)
        expected_repr_parts = list(
            filter(
                None,
                [
                    f"path='{p.name}'",
                    f"read_size={read_size}"
                    if read_size is not None
                    else f"read_size={_DEFAULT_READ_SIZE}",
                    f"offset={offset}" if offset is not None and offset != 0 else None,
                    "buffered=True" if buffered is None else f"buffered={buffered}",
                    "strict=True" if strict is None else f"strict={strict}",
                ],
            )
        )
        expected_repr = f"AsyncFileReadStream({', '.join(expected_repr_parts)})"
        assert repr_str == expected_repr

        # eval
        evaluated_stream = eval(
            repr_str, {"AsyncFileReadStream": ry.AsyncFileReadStream}
        )
        assert stream == evaluated_stream


class TestAsyncFileReadStreamErrors:
    def test_read_size_zero_raises(self) -> None:
        """Test that read_size of zero raises ValueError."""
        with pytest.raises(ValueError, match="read_size must be greater than 0"):
            _rs = ry.read_stream_async(_THIS_FILEPATH_ABSOLUTE, read_size=0)


class TestAsyncFileReadStreamCoroutines:
    @pytest.fixture
    def path(self, tmp_path: Path) -> Path:
        p = tmp_path / "test.txt"
        p.write_bytes(b"0123456789" * 10)
        return p

    async def test_qualnames(self, path: Path) -> None:
        stream = ry.read_stream_async(path, read_size=10)
        coros: dict[str, t.Any] = {
            "AsyncFileReadStream.__await__": stream.__await__(),  # noqa: PLC2801
            "AsyncFileReadStream.__aenter__": stream.__aenter__(),  # noqa: PLC2801
            "AsyncFileReadStream.__aexit__": stream.__aexit__(None, None, None),  # noqa: PLC2801
            "AsyncFileReadStream.__anext__": anext(stream),
            "AsyncFileReadStream.take": stream.take(),
            "AsyncFileReadStream.collect": stream.collect(),
            "AsyncFileReadStream.close": stream.close(),
        }
        for qualname, coro in coros.items():
            assert coro.__qualname__ == qualname
            assert asyncio.iscoroutine(coro)
            coro.close()

    async def test_await_returns_self(self, path: Path) -> None:
        stream = ry.read_stream_async(path, read_size=10)
        assert await stream is stream

    async def test_yield_from_await_adapter(self, path: Path) -> None:
        @types.coroutine
        def _yield_from(aw: t.Any) -> t.Generator[t.Any, t.Any, t.Any]:
            return (yield from aw.__await__())

        stream = ry.read_stream_async(path, read_size=10)
        assert await _yield_from(stream) is stream
        assert await _yield_from(anext(stream)) == b"0123456789"

    async def test_async_with(self, path: Path) -> None:
        async with ry.read_stream_async(path, read_size=10) as stream:
            assert await stream.take(2) == [b"0123456789"] * 2
        # closed ~ next read re-opens from the start
        assert await stream.take(1) == [b"0123456789"]

    async def test_close_is_lazy(self, path: Path) -> None:
        stream = await ry.read_stream_async(path, read_size=10)
        assert await stream.take(1) == [b"0123456789"]
        stream.close().close()  # never awaited ~ no-op
        assert len(await stream.collect()) == 9

    async def test_close_resets(self, path: Path) -> None:
        stream = await ry.read_stream_async(path, read_size=10)
        assert len(await stream.collect()) == 10
        with pytest.raises(StopAsyncIteration):
            await anext(stream)
        await stream.close()
        assert len(await stream.collect()) == 10

    async def test_cancelled_collect_releases_lock(self, tmp_path: Path) -> None:
        p = tmp_path / "big.bin"
        p.write_bytes(b"x" * (16 * 1024 * 1024))
        stream = await ry.read_stream_async(p, read_size=1)
        task = asyncio.create_task(stream.collect())
        await asyncio.sleep(0)
        task.cancel()
        with pytest.raises(asyncio.CancelledError):
            await task
        assert len(await asyncio.wait_for(stream.take(1), timeout=5)) == 1


async def test_read_stream_collect_raises_io_errors(tmp_path: Path) -> None:
    with pytest.raises(OSError):  # noqa: PT011
        await ry.read_stream_async(tmp_path, read_size=10).collect()


async def test_read_stream_cancelled_next_keeps_order(tmp_path: Path) -> None:
    p = tmp_path / "f.bin"
    data = bytes(range(256)) * 4096
    p.write_bytes(data)
    stream = ry.read_stream_async(p, read_size=4096)
    got = bytearray()
    for _ in range(20):
        task = asyncio.ensure_future(anext(stream))
        await asyncio.sleep(0)
        task.cancel()
        res = await asyncio.gather(task, return_exceptions=True)
        if isinstance(res[0], ry.Bytes):
            got += res[0]
        got += await anext(stream)
    got += b"".join(await stream.collect())
    assert bytes(got) == data


@pytest.mark.parametrize("buffered", [True, False])
@pytest.mark.parametrize("read_size", [1, 7, 4096, 65_536, 200_000])
async def test_read_stream_sizes(
    tmp_path: Path, read_size: int, *, buffered: bool
) -> None:
    p = tmp_path / "f.bin"
    # > 2 buffers for the big sizes; tiny sizes just need a few buffer hits
    data = bytes(range(256)) * (1200 if read_size >= 4096 else 40)
    p.write_bytes(data)
    chunks = await ry.read_stream_async(
        p, read_size=read_size, buffered=buffered
    ).collect()
    assert b"".join(chunks) == data
    assert all(len(c) == read_size for c in chunks[:-1])
    taken = await ry.read_stream_async(
        p, read_size=read_size, buffered=buffered, offset=3
    ).take(3)
    assert b"".join(taken) == data[3 : 3 + 3 * read_size]


async def test_read_stream_take_zero_still_opens(tmp_path: Path) -> None:
    with pytest.raises(FileNotFoundError):
        await ry.read_stream_async(tmp_path / "nope", read_size=10).take(0)


@pytest.mark.skipif(not sys.platform.startswith("linux"), reason="/proc/self/fd")
async def test_read_stream_close_waits_for_read_ahead(tmp_path: Path) -> None:
    p = tmp_path / "f.bin"
    p.write_bytes(b"x" * (32 << 20))

    def open_fds() -> int:
        fds = os.listdir("/proc/self/fd")
        return sum(1 for fd in fds if _readlink(f"/proc/self/fd/{fd}") == str(p))

    # racy (the read-ahead may not have started yet) ~ loop to catch it
    for _ in range(20):
        async with ry.read_stream_async(p, read_size=16 << 20) as stream:
            await anext(stream)  # kicks off the read-ahead of the next chunk
        assert open_fds() == 0
        await asyncio.sleep(0.01)


def _readlink(path: str) -> str | None:
    try:
        return os.readlink(path)
    except OSError:
        return None


@pytest.mark.skipif(not sys.platform.startswith("linux"), reason="mkfifo + /proc")
async def test_read_stream_fifo_close(tmp_path: Path) -> None:
    # the writer stays open, so a read with no data waiting blocks
    fifo = tmp_path / "fifo"
    os.mkfifo(fifo)
    writer = os.open(fifo, os.O_RDWR)  # O_RDWR so neither end blocks on open

    def open_fds() -> int:
        fds = os.listdir("/proc/self/fd")
        return sum(1 for fd in fds if _readlink(f"/proc/self/fd/{fd}") == str(fifo))

    try:
        # no read-ahead on a fifo, so closing doesn't wait for more data
        os.write(writer, b"abcd")
        async with ry.read_stream_async(fifo, read_size=4) as stream:
            assert await anext(stream) == b"abcd"
        assert open_fds() == 1  # just the writer

        # close doesn't wait for a cancelled read that's still blocked; that
        # read keeps the fd open until data arrives
        stream = await ry.read_stream_async(fifo, read_size=4)
        reading = asyncio.ensure_future(anext(stream))
        await asyncio.sleep(0.05)
        reading.cancel()
        await asyncio.wait_for(stream.close(), 1)
        assert open_fds() == 2
        os.write(writer, b"efgh")
        for _ in range(100):
            if open_fds() == 1:
                break
            await asyncio.sleep(0.01)
        assert open_fds() == 1
    finally:
        os.close(writer)


@pytest.mark.parametrize("read_size", [1, 3, 4096])
async def test_read_stream_mixed_next_take_collect(
    tmp_path: Path, read_size: int
) -> None:
    # anext, take and collect read differently; mixed together they must
    # still return every byte in order
    p = tmp_path / "f.bin"
    data = bytes(range(256)) * 300
    p.write_bytes(data)
    async with ry.read_stream_async(p, read_size=read_size) as stream:
        first = await anext(stream)
        taken = await stream.take(5)
        assert len(taken) == 5
        rest = await stream.collect()
        assert await stream.take(3) == []
    assert first + b"".join(taken) + b"".join(rest) == data


@pytest.mark.skipif(not sys.platform.startswith("linux"), reason="mkfifo")
async def test_read_stream_fifo_cancelled_collect_then_close(tmp_path: Path) -> None:
    # collect() grabs the first chunk and then waits for more; cancel it and
    # make sure close() doesn't hang
    fifo = tmp_path / "fifo"
    os.mkfifo(fifo)
    writer = os.open(fifo, os.O_RDWR)
    try:
        os.write(writer, b"abcd")
        stream = await ry.read_stream_async(fifo, read_size=4)
        collecting = asyncio.ensure_future(stream.collect())
        await asyncio.sleep(0.05)
        collecting.cancel()
        await asyncio.wait_for(stream.close(), 1)
    finally:
        os.write(writer, b"efgh")  # let the stuck read finish
        os.close(writer)
