from pathlib import Path

import pytest

from ry import AsyncFile


#
# @pytest.fixture
# def temp_file_path():
#     with tempfile.NamedTemporaryFile(delete=False) as f:
#         yield f.name
#     os.remove(f.name)
@pytest.mark.anyio
async def test_write_and_read(tmp_path: Path):
    f = AsyncFile(tmp_path / "file.txt", "w+")
    await f.open()
    await f.write(b"hello\nworld\n")
    await f.close()

    f = AsyncFile(tmp_path / "file.txt", "r")
    await f.open()
    data = await f.read()
    assert data == b"hello\nworld\n"
    await f.close()


@pytest.mark.anyio
async def test_read_size(tmp_path: Path):
    f = AsyncFile(tmp_path / "file.txt", "w+")
    await f.open()
    await f.write(b"1234567890")
    await f.close()

    f = AsyncFile(tmp_path / "file.txt", "r")
    await f.open()
    part = await f.read(4)
    assert part == b"1234"
    part2 = await f.read(2)
    assert part2 == b"56"


@pytest.mark.anyio
async def test_async_iteration(tmp_path: Path):
    temp_file_path = tmp_path / "test_file.txt"
    lines = b"line1\nline2\nline3\n"
    f = AsyncFile(temp_file_path, "w+")
    await f.open()
    await f.write(lines)
    await f.close()

    f = AsyncFile(temp_file_path, "r")
    await f.open()

    collected = []
    async for line in f:
        collected.append(line)
    assert collected == [b"line1\n", b"line2\n", b"line3\n"]


@pytest.mark.anyio
async def test_context_manager(tmp_path: Path) -> None:
    temp_file_path = tmp_path / "test_file.txt"
    async with AsyncFile(temp_file_path, "w") as f:
        await f.write(b"ctx test")

    async with AsyncFile(temp_file_path, "r") as f:
        contents = await f.read()
        assert contents == b"ctx test"


@pytest.mark.anyio
async def test_fail_read_before_open(tmp_path: Path) -> None:
    temp_file_path = tmp_path / "test_file.txt"
    f = AsyncFile(temp_file_path, "r")
    with pytest.raises(RuntimeError):
        await f.read()
