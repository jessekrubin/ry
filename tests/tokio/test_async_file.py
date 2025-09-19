from __future__ import annotations

import asyncio
import dataclasses
import io
from itertools import permutations
from typing import TYPE_CHECKING

import pytest

import ry
from ry import AsyncFile, aiopen

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
async def test_async_iteration(tmp_path: Path) -> None:
    temp_file_path = tmp_path / "test_file.txt"
    lines = b"line1\nline2\nline3\n"
    f = AsyncFile(temp_file_path, "wb+")
    await f.open()
    await f.write(lines)
    await f.close()

    f = AsyncFile(temp_file_path, "rb")
    await f.open()

    collected = []
    async for line in f:
        collected.append(line)
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


@pytest.fixture()
def aiopen_fixtures(tmp_path: Path) -> FileFixtures:
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
    @pytest.mark.anyio()
    @pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
    @pytest.mark.parametrize("buffering", [-1, 0])
    async def test_simple_iteration(
        self, aiopen_fixtures: FileFixtures, mode: str, buffering: int
    ) -> None:
        """Test iterating over lines from a file."""

        async with aiopen(
            aiopen_fixtures.multiline_file_path, mode=mode, buffering=buffering
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

    @pytest.mark.anyio()
    @pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
    @pytest.mark.parametrize("buffering", [-1, 0])
    async def test_simple_readlines(
        self, aiopen_fixtures: FileFixtures, mode: str, buffering: int
    ) -> None:
        """Test the readlines functionality."""
        with open(aiopen_fixtures.multiline_file_path, mode="rb") as f:
            expected = f.readlines()

        async with aiopen(
            aiopen_fixtures.multiline_file_path, mode=mode, buffering=buffering
        ) as file:
            # Append mode needs us to seek.
            await file.seek(0)

            actual = await file.readlines()

        assert actual == expected


@pytest.mark.anyio()
@pytest.mark.parametrize("mode", _mode_permutations(["rb+", "wb", "ab"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_flush(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test flushing to a file."""
    filename = "file.bin"

    full_file = tmp_path.joinpath(filename)

    if "r" in mode:
        full_file.touch()  # Read modes want it to already exist.

    if buffering == 0:
        pytest.skip("not supported by the current implementation of aiopen")
    async with aiopen(str(full_file), mode=mode, buffering=buffering) as file:
        await file.write(b"0")  # Shouldn't flush.

        if buffering == -1:
            assert full_file.read_bytes() == b""
        else:
            assert full_file.read_bytes() == b"0"

        await file.flush()

        assert full_file.read_bytes() == b"0"


@pytest.mark.anyio()
@pytest.mark.parametrize("mode", _mode_permutations(["rb+", "wb+", "ab+"]))
async def test_simple_peek(mode: str, tmp_path: Path) -> None:
    """Test flushing to a file."""
    filename = "file.bin"

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(b"0123456789")

    async with aiopen(str(full_file), mode=mode) as file:
        if "a" in mode:
            await file.seek(0)  # Rewind for append modes.

        peeked = await file.peek(1)

        # Technically it's OK for the peek to return less bytes than requested.
        if peeked:
            assert peeked.startswith(b"0")

            read = await file.read(1)

            assert peeked.startswith(read)


@pytest.mark.anyio()
@pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_read(
    aiopen_fixtures: FileFixtures, mode: str, buffering: int
) -> None:
    """Just read some bytes from a test file."""
    filename = str(aiopen_fixtures.multiline_file_path)
    async with aiopen(filename, mode=mode, buffering=buffering) as file:
        await file.seek(0)  # Needed for the append mode.

        actual = await file.read()

        assert (await file.read()) == b""
    assert actual == open(filename, mode="rb").read()


@pytest.mark.anyio()
@pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_staggered_read(
    aiopen_fixtures: FileFixtures, mode: str, buffering: int
) -> None:
    """Read bytes repeatedly."""
    filename = str(aiopen_fixtures.multiline_file_path)
    async with aiopen(filename, mode=mode, buffering=buffering) as file:
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


@pytest.mark.anyio()
@pytest.mark.parametrize("mode", _mode_permutations(["rb", "rb+", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_seek(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test seeking and then reading."""
    filename = "bigfile.bin"
    content = b"0123456789" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)
    full_file.write_bytes(content)

    async with aiopen(str(full_file), mode=mode, buffering=buffering) as file:
        await file.seek(4)

        assert (await file.read(1)) == b"4"


@pytest.mark.anyio()
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

    async with aiopen(str(full_file), mode=mode, buffering=buffering) as file:
        assert not file.closed

    assert file.closed


@pytest.mark.anyio()
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

    file = await aiopen(str(full_file), mode=mode, buffering=buffering)
    assert not file.closed

    await file.close()

    assert file.closed


# TODO: FIGURE OUT WHY "ab+" DONT WORK ON WINDOWS
@pytest.mark.anyio()
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

    async with aiopen(str(full_file), mode=mode, buffering=buffering) as file:
        # The append modes want us to seek first.
        await file.seek(0)

        if "w" in mode:
            # We've just erased the entire file.
            await file.write(content)
            await file.flush()
            await file.seek(0)

        await file.truncate()

    assert full_file.read_bytes() == b""


@pytest.mark.anyio()
@pytest.mark.parametrize("mode", _mode_permutations(["wb", "rb+", "wb+", "ab", "ab+"]))
@pytest.mark.parametrize("buffering", [-1, 0])
async def test_simple_write(mode: str, buffering: int, tmp_path: Path) -> None:
    """Test writing into a file."""
    filename = "bigfile.bin"
    content = b"0" * 4 * io.DEFAULT_BUFFER_SIZE

    full_file = tmp_path.joinpath(filename)

    if "r" in mode:
        full_file.touch()  # Read modes want it to already exist.

    async with aiopen(str(full_file), mode=mode, buffering=buffering) as file:
        bytes_written = await file.write(content)

    assert bytes_written == len(content)
    assert content == full_file.read_bytes()


@pytest.mark.anyio()
async def test_simple_readall(tmp_path: Path) -> None:
    """Test the readall function by reading a large file in.

    Only RawIOBase supports readall().
    """
    filename = "bigfile.bin"
    content = b"0" * 4 * io.DEFAULT_BUFFER_SIZE  # Hopefully several reads.

    sync_file = tmp_path.joinpath(filename)
    sync_file.write_bytes(content)

    file = await aiopen(str(sync_file), mode="rb", buffering=0)

    actual = await file.readall()

    assert actual == content

    await file.close()
    assert file.closed


@pytest.mark.anyio()
async def test_file_async_context_aexit(aiopen_fixtures: FileFixtures) -> None:
    test_file = aiopen_fixtures.test_file_path
    async with aiopen(test_file) as fp:
        ...

    with pytest.raises(RuntimeError):
        _line = await fp.read()

    async with aiopen(test_file) as fp:
        line = await fp.read()
        assert line.decode() == "0123456789"


@pytest.mark.anyio()
async def test_filetask_async_context_aexit(
    aiopen_fixtures: FileFixtures,
) -> None:
    test_file = aiopen_fixtures.test_file_path
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
    file_ctx = aiopen(test_file)

    task = asyncio.create_task(
        _process_test_file(file_ctx=file_ctx, sleep_time=sleep_time)
    )
    try:
        await asyncio.wait_for(task, timeout=cancel_time)
    except TimeoutError:
        assert task.cancelled  # type: ignore[truthy-function]
    assert file_ref is not None
    assert file_ref.closed
