from __future__ import annotations

import asyncio
import dataclasses
import io
import sys
from itertools import permutations
from typing import TYPE_CHECKING

import pytest

import ry
from ry import AsyncFile, aopen

if TYPE_CHECKING:
    from pathlib import Path


@pytest.mark.anyio
async def test_write_and_read(tmp_path: Path) -> None:
    f = AsyncFile(tmp_path / "file.txt", "wb+")
    await f.open()
    await f.write(b"hello\nworld\n")
    await f.close()

    f = AsyncFile(tmp_path / "file.txt", "rb")
    await f.open()
    data = await f.read()
    assert data == b"hello\nworld\n"
    await f.close()


@pytest.mark.anyio
async def test_read_size(tmp_path: Path) -> None:
    f = AsyncFile(tmp_path / "file.txt", "wb+")
    await f.open()
    await f.write(b"1234567890")
    await f.close()

    f = AsyncFile(tmp_path / "file.txt", "rb")
    await f.open()
    part = await f.read(4)
    assert part == b"1234"
    part2 = await f.read(2)
    assert part2 == b"56"


@pytest.mark.anyio
async def test_read_peek_huge_size(tmp_path: Path) -> None:
    """Huge `read(n)`/`peek(n)` allocate for the data, not the request."""
    (tmp_path / "file.txt").write_bytes(b"1234567890")
    f = AsyncFile(tmp_path / "file.txt", "rb")
    await f.open()
    peeked = await f.peek(2**62)
    assert b"1234567890".startswith(peeked)
    assert len(peeked) > 0
    assert await f.read(2**62) == b"1234567890"
    assert await f.read(2**62) == b""
    await f.close()


@pytest.mark.anyio
async def test_async_iteration(tmp_path: Path) -> None:
    temp_file_path = tmp_path / "test_file.txt"
    lines = b"line1\nline2\nline3\n"
    f = AsyncFile(temp_file_path, "wb+")
    await f.open()
    await f.write(lines)
    await f.close()

    f = AsyncFile(temp_file_path, "rb")
    await f.open()

    collected = [line async for line in f]
    assert collected == [b"line1\n", b"line2\n", b"line3\n"]


@pytest.mark.anyio
async def test_context_manager(tmp_path: Path) -> None:
    temp_file_path = tmp_path / "test_file.txt"
    async with AsyncFile(temp_file_path, "wb") as f:
        await f.write(b"ctx test")

    async with AsyncFile(temp_file_path, "rb") as f:
        contents = await f.read()
        assert contents == b"ctx test"


@pytest.mark.anyio
async def test_fail_read_before_open(tmp_path: Path) -> None:
    temp_file_path = tmp_path / "test_file.txt"
    f = AsyncFile(temp_file_path, "rb")
    with pytest.raises(RuntimeError):
        await f.read()


@dataclasses.dataclass
class FileFixtures:
    """Dataclass to hold file fixtures."""

    tmp_path: Path
    multiline_file_path: Path
    test_file_path: Path


@pytest.fixture
def aopen_fixtures(tmp_path: Path) -> FileFixtures:
    """Fixture to create a temporary file and return its path and an AsyncFile object."""
    ry.mkdir(tmp_path / "resources")
    multiline_file_path = tmp_path / "resources" / "multiline_file.txt"
    test_file_path = tmp_path / "resources" / "test_file1.txtt"
    with open(multiline_file_path, "w", encoding="utf-8", newline="\n") as f:
        f.write("line 1\nline 2\nline 3\n")
    with open(test_file_path, "w", encoding="utf-8", newline="\n") as f:
        f.write("0123456789")
    return FileFixtures(
        tmp_path=tmp_path,
        multiline_file_path=multiline_file_path,
        test_file_path=test_file_path,
    )


def _mode_permutations(modes: list[str]) -> list[str]:
    """Generate all orderings of a mode"""
    return sorted({"".join(p) for mode in modes for p in permutations(mode)})


class TestAsyncFileAiopen:
    @pytest.mark.anyio
    @pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
    @pytest.mark.parametrize("buffering", [-1, 0])
    async def test_simple_iteration(
        self, aopen_fixtures: FileFixtures, mode: str, buffering: int
    ) -> None:
        """Test iterating over lines from a file."""
        if buffering == 0:
            with pytest.raises(
                NotImplementedError, match="aopen buffering not implemented: 0"
            ):
                _f = await aopen(
                    aopen_fixtures.multiline_file_path, mode=mode, buffering=buffering
                )
            return
        async with aopen(
            aopen_fixtures.multiline_file_path, mode=mode, buffering=buffering
        ) as file:
            # Append mode needs us to seek.
            await file.seek(0)

            counter = 1
            # The old iteration pattern:
            while True:
                line = await file.readline()
                if not line:
                    break
                assert bytes(line).strip() == b"line " + str(counter).encode()
                counter += 1

            counter = 1
            await file.seek(0)
            # The new iteration pattern:
            async for line in file:
                assert bytes(line).strip() == b"line " + str(counter).encode()
                counter += 1

        assert file.closed

    @pytest.mark.anyio
    async def test_readline_size_keeps_rest_of_line(
        self, aopen_fixtures: FileFixtures
    ) -> None:
        async with aopen(aopen_fixtures.multiline_file_path, mode="rb") as file:
            assert bytes(await file.readline(0)) == b""
            assert bytes(await file.readline(3)) == b"lin"
            assert bytes(await file.readline(3)) == b"e 1"
            assert bytes(await file.readline(3)) == b"\n"
            assert bytes(await file.readline()) == b"line 2\n"
            assert bytes(await file.readline(100)) == b"line 3\n"

    @pytest.mark.anyio
    @pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
    @pytest.mark.parametrize("buffering", [-1, 0])
    async def test_simple_readlines(
        self, aopen_fixtures: FileFixtures, mode: str, buffering: int
    ) -> None:
        """Test the readlines functionality."""
        if buffering == 0:
            with pytest.raises(
                NotImplementedError, match="aopen buffering not implemented: 0"
            ):
                _f = await aopen(
                    aopen_fixtures.multiline_file_path, mode=mode, buffering=buffering
                )
            return
        with open(aopen_fixtures.multiline_file_path, mode="rb") as f:
            expected = f.readlines()

        async with aopen(
            aopen_fixtures.multiline_file_path, mode=mode, buffering=buffering
        ) as file:
            # Append mode needs us to seek.
            await file.seek(0)

            actual = await file.readlines()

        assert actual == expected


@pytest.mark.anyio
@pytest.mark.parametrize("mode", _mode_permutations(["rb+", "wb", "ab"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_flush(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test flushing to a file."""
    filename = "file.bin"

    full_file = tmp_path.joinpath(filename)

    if "r" in mode:
        full_file.touch()  # Read modes want it to already exist.

    if buffering == 0:
        pytest.skip("not supported by the current implementation of aopen")
    async with aopen(str(full_file), mode=mode, buffering=buffering) as file:
        await file.write(b"0")  # Shouldn't flush.

        if buffering == -1:
            assert full_file.read_bytes() == b""
        else:
            assert full_file.read_bytes() == b"0"

        await file.flush()

        assert full_file.read_bytes() == b"0"


@pytest.mark.anyio
@pytest.mark.parametrize("mode", _mode_permutations(["rb+", "wb+", "ab+"]))
async def test_simple_peek(mode: str, tmp_path: Path) -> None:
    """Test flushing to a file."""
    filename = "file.bin"

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(b"0123456789")

    async with aopen(str(full_file), mode=mode) as file:
        if "a" in mode:
            await file.seek(0)  # Rewind for append modes.

        peeked = await file.peek(1)

        # Technically it's OK for the peek to return less bytes than requested.
        if peeked:
            assert peeked.startswith(b"0")

            read = await file.read(1)

            assert peeked.startswith(read)


@pytest.mark.anyio
@pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_read(
    aopen_fixtures: FileFixtures, mode: str, buffering: int
) -> None:
    """Just read some bytes from a test file."""
    filename = str(aopen_fixtures.multiline_file_path)
    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(filename, mode=mode, buffering=buffering)
        return
    async with aopen(filename, mode=mode, buffering=buffering) as file:
        await file.seek(0)  # Needed for the append mode.

        actual = await file.read()

        assert (await file.read()) == b""
    with open(filename, mode="rb") as f:
        data = f.read()
    assert actual == data


@pytest.mark.anyio
@pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_staggered_read(
    aopen_fixtures: FileFixtures, mode: str, buffering: int
) -> None:
    """Read bytes repeatedly."""
    filename = str(aopen_fixtures.multiline_file_path)
    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(filename, mode=mode, buffering=buffering)
        return
    async with aopen(filename, mode=mode, buffering=buffering) as file:
        await file.seek(0)  # Needed for the append mode.

        actual = []
        while True:
            byte = await file.read(1)
            if byte:
                actual.append(byte)
            else:
                break

        assert (await file.read()) == b""

        expected = []
        with open(filename, mode="rb") as f:
            while True:
                byte = ry.Bytes(f.read(1))
                if byte:
                    expected.append(byte)
                else:
                    break

    assert actual == expected


@pytest.mark.anyio
@pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_seek(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test seeking and then reading."""
    filename = "bigfile.bin"
    content = b"0123456789" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(content)

    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(str(full_file), mode=mode, buffering=buffering)
        return

    async with aopen(str(full_file), mode=mode, buffering=buffering) as file:
        await file.seek(4)

        assert (await file.read(1)) == b"4"


@pytest.mark.anyio
@pytest.mark.parametrize(
    "mode", _mode_permutations(["wb", "rb", "rb+", "wb+", "ab", "ab+"])
)
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_close_ctx_mgr(mode: str, buffering: int, tmp_path: Path) -> None:
    """Open a file, read a byte, and close it."""
    filename = "bigfile.bin"
    content = b"0" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(content)
    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(str(full_file), mode=mode, buffering=buffering)
        return

    async with aopen(str(full_file), mode=mode, buffering=buffering) as file:
        assert not file.closed

    assert file.closed


@pytest.mark.anyio
@pytest.mark.parametrize(
    "mode", _mode_permutations(["wb", "rb", "rb+", "wb+", "ab", "ab+"])
)
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_close_no_ctx_mgr(
    mode: str, buffering: int, tmp_path: Path
) -> None:
    """Open a file, read a byte, and close it."""
    filename = "bigfile.bin"
    content = b"0" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(content)

    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(str(full_file), mode=mode, buffering=buffering)
        return

    file = await aopen(str(full_file), mode=mode, buffering=buffering)
    assert not file.closed

    await file.close()

    assert file.closed


# TODO: FIGURE OUT WHY "ab+" DONT WORK ON WINDOWS
@pytest.mark.anyio
@pytest.mark.parametrize(
    "mode",
    [
        "rb+",
        "wb",
    ],
)
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_truncate(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test truncating files."""
    filename = "bigfile.bin"
    content = b"0123456789" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(content)
    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(str(full_file), mode=mode, buffering=buffering)
        return

    async with aopen(str(full_file), mode=mode, buffering=buffering) as file:
        # The append modes want us to seek first.
        await file.seek(0)

        if "w" in mode:
            # We've just erased the entire file.
            await file.write(content)
            await file.flush()
            await file.seek(0)

        await file.truncate()

    assert full_file.read_bytes() == b""


@pytest.mark.anyio
@pytest.mark.parametrize("mode", _mode_permutations(["wb", "rb+", "wb+", "ab", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_write(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test writing into a file."""
    filename = "bigfile.bin"
    content = b"0" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)

    if "r" in mode:
        full_file.touch()  # Read modes want it to already exist.

    if buffering == 0:
        with pytest.raises(
            NotImplementedError, match="aopen buffering not implemented: 0"
        ):
            _f = await aopen(str(full_file), mode=mode, buffering=buffering)
        return

    async with aopen(str(full_file), mode=mode, buffering=buffering) as file:
        bytes_written = await file.write(content)

    assert bytes_written == len(content)
    assert content == full_file.read_bytes()


@pytest.mark.anyio
async def test_simple_readall(tmp_path: Path) -> None:
    """Test the readall function by reading a large file in.

    Only RawIOBase supports readall().
    """
    filename = "bigfile.bin"
    content = b"0" * 4 * io.DEFAULT_BUFFER_SIZE  # Hopefully several reads.

    sync_file = tmp_path.joinpath(filename)
    sync_file.write_bytes(content)

    file = await aopen(str(sync_file), mode="rb")

    actual = await file.readall()

    assert actual == content

    await file.close()
    assert file.closed


@pytest.mark.anyio
async def test_file_async_context_aexit(aopen_fixtures: FileFixtures) -> None:
    test_file = aopen_fixtures.test_file_path
    async with aopen(test_file) as fp:
        ...

    with pytest.raises(RuntimeError):
        _line = await fp.read()

    async with aopen(test_file) as fp:
        line = await fp.read()
        assert line.decode() == "0123456789"


@pytest.mark.anyio
async def test_filetask_async_context_aexit(
    aopen_fixtures: FileFixtures,
) -> None:
    test_file = aopen_fixtures.test_file_path
    file_ref = None

    async def _process_test_file(file_ctx: AsyncFile, sleep_time: float = 1.0) -> None:
        nonlocal file_ref
        async with file_ctx as fp:
            file_ref = file_ctx
            await asyncio.sleep(sleep_time)
            await fp.read()

    cancel_time, sleep_time = 0.1, 10
    assert cancel_time <= (sleep_time / 10)

    file_ref = None
    file_ctx = aopen(test_file)

    task = asyncio.create_task(
        _process_test_file(file_ctx=file_ctx, sleep_time=sleep_time)
    )
    try:
        await asyncio.wait_for(task, timeout=cancel_time)
    except TimeoutError:
        assert task.cancelled()
    assert file_ref is not None
    assert file_ref.closed


@pytest.mark.anyio
async def test_async_file_flushes_on_drop(tmp_path: Path) -> None:
    """Test that the file is flushed when the object is dropped."""
    if sys.implementation.name == "pypy":
        pytest.skip("something is messed up w/ pypy")
    temp_file_path = tmp_path / "test_file_drop.txt"
    f = AsyncFile(temp_file_path, "wb")
    await f
    await f.write(b"some data")
    del f
    await asyncio.sleep(0.1)
    # check that got flusshed
    contents = ry.read_bytes(temp_file_path)
    assert contents == b"some data"


class TestAsyncFileCoroutines:
    """`ry.Coroutine` shape + cancel-safety (cancel == the rust future is dropped)"""

    async def test_qualnames(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"0123456789")
        f = await aopen(p, "rb+")
        coros = {
            "AsyncFile.__await__": f.__await__(),  # noqa: PLC2801
            "AsyncFile.__aenter__": f.__aenter__(),  # noqa: PLC2801
            "AsyncFile.__aexit__": f.__aexit__(None, None, None),  # noqa: PLC2801
            "AsyncFile.__anext__": anext(f),
            "AsyncFile.close": f.close(),
            "AsyncFile.flush": f.flush(),
            "AsyncFile.isatty": f.isatty(),
            "AsyncFile.open": f.open(),
            "AsyncFile.peek": f.peek(),
            "AsyncFile.read": f.read(),
            "AsyncFile.readall": f.readall(),
            "AsyncFile.readline": f.readline(),
            "AsyncFile.readlines": f.readlines(),
            "AsyncFile.seek": f.seek(0),
            "AsyncFile.tell": f.tell(),
            "AsyncFile.truncate": f.truncate(),
            "AsyncFile.write": f.write(b""),
        }
        for qualname, coro in coros.items():
            assert asyncio.iscoroutine(coro)
            assert coro.__qualname__ == qualname
            coro.close()
        await f.close()

    async def test_isatty_raises_on_await(self, tmp_path: Path) -> None:
        f = AsyncFile(tmp_path / "f.bin", "wb")
        coro = f.isatty()  # no raise on call
        with pytest.raises(NotImplementedError):
            await coro

    async def test_await_again_does_not_reopen(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        f = await aopen(p, "wb")
        await f.write(b"keep me")
        assert await f is f  # already open ~ must not truncate/drop the buffer
        await f.close()
        assert p.read_bytes() == b"keep me"

    async def test_readlines_raises_io_errors(self, tmp_path: Path) -> None:
        async with aopen(tmp_path / "f.bin", "wb") as f:
            with pytest.raises(OSError):  # noqa: PT011
                await f.readlines()

    async def test_cancelled_seek_does_not_wedge(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"x" * 4096)
        async with aopen(p, "rb") as f:
            for i in range(300):
                task = asyncio.create_task(f.seek(i))
                await asyncio.sleep(0)
                task.cancel()
                await asyncio.gather(task, return_exceptions=True)
                assert await f.seek(7) == 7
                assert await f.tell() == 7

    async def test_cancelled_peek_keeps_position(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"0123456789" * 1000)
        async with aopen(p, "rb") as f:
            for _ in range(300):
                await f.seek(0)
                task = asyncio.create_task(f.peek(4))
                await asyncio.sleep(0)
                task.cancel()
                await asyncio.gather(task, return_exceptions=True)
                assert await f.tell() == 0
                assert await f.read(1) == b"0"

    async def test_cancelled_close_keeps_data(self, tmp_path: Path) -> None:
        data = b"z" * (1024 * 1024)
        for i in range(50):
            p = tmp_path / f"f{i}.bin"
            f = await aopen(p, "wb")
            await f.write(data)
            task = asyncio.create_task(f.close())
            await asyncio.sleep(0)
            task.cancel()
            await asyncio.gather(task, return_exceptions=True)
            await f.close()  # no-op if the first one got there
            assert f.closed
            assert p.read_bytes() == data

    async def test_dropped_mid_write_still_flushes(self, tmp_path: Path) -> None:
        """last ref can go away on a tokio thread (aborted op) ~ no block_on panic"""
        import gc  # noqa: PLC0415

        for i in range(50):
            p = tmp_path / f"f{i}.bin"
            f = await aopen(p, "wb")
            await f.write(b"head")
            task = asyncio.create_task(f.write(b"t" * (1024 * 1024)))
            await asyncio.sleep(0)
            task.cancel()
            del f
            await asyncio.gather(task, return_exceptions=True)
            del task
            gc.collect()
            for _ in range(100):
                if p.read_bytes().startswith(b"head"):
                    break
                await asyncio.sleep(0.01)
            assert p.read_bytes().startswith(b"head")


class TestAsyncFileErrors:
    """errors match the builtin `open()`/`BufferedReader` ones"""

    @pytest.mark.parametrize("mode", ["rb", "wb"])
    async def test_open_missing(self, tmp_path: Path, mode: str) -> None:
        p = tmp_path / "nope" / "f.bin"
        with pytest.raises(FileNotFoundError) as ry_exc:
            await aopen(p, mode)
        with pytest.raises(FileNotFoundError) as py_exc:
            open(p, mode)  # noqa: SIM115, PTH123
        assert ry_exc.value.errno == py_exc.value.errno
        assert ry_exc.value.filename == str(p)
        assert str(ry_exc.value) == str(py_exc.value).replace(repr(p), repr(str(p)))

    async def test_open_exists_exclusive(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"")
        with pytest.raises(FileExistsError) as exc:
            await aopen(p, "xb")
        assert exc.value.filename == str(p)

    async def test_seek_invalid_whence(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"0123")
        async with aopen(p, "rb") as f:
            with pytest.raises(ValueError, match=r"invalid whence \(5"):
                await f.seek(0, 5)

    async def test_seek_negative_position(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"0123")
        async with aopen(p, "rb") as f:
            with pytest.raises(ValueError, match="negative seek position -1"):
                await f.seek(-1)
