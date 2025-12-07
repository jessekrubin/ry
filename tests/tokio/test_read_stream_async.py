from __future__ import annotations

import typing as t
from pathlib import Path

import pytest

import ry

_THIS_FILEPATH_ABOSLUTE = Path(__file__).resolve()
_DEFAULT_CHUNK_SIZE = 65_536


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
        "test.txt", chunk_size=10, buffered=buffered
    ).collect()
    assert b"".join(chunks) == string_bytes
    assert len(chunks) == len(string_bytes) // 10 + 1

    chunks_built = []
    async for chunk in ry.read_stream_async(
        "test.txt", chunk_size=10, buffered=buffered
    ):
        chunks_built.append(chunk)
    assert b"".join(chunks_built) == string_bytes
    assert len(chunks_built) == len(string_bytes) // 10 + 1

    # with offset

    chunks = await ry.read_stream_async(
        "test.txt", chunk_size=10, offset=100, buffered=buffered
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
    stream = await ry.read_stream_async("test.txt", chunk_size=10, buffered=buffered)
    uno = await stream.take()
    dos = await stream.take(2)
    restante = await stream.collect()
    assert len(uno) == 1
    assert uno == [b"0\n1\n2\n3\n4\n"]
    assert dos == [b"5\n6\n7\n8\n9\n", b"10\n11\n12\n1"]
    assert len(restante) == 386


async def test_read_stream_file_not_found(tmp_path: Path) -> None:
    """Test that reading a file in chunks works w/ and w/o offset."""
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        _rs = await ry.read_stream_async("test.txt", chunk_size=10)


async def test_read_stream_is_directory(tmp_path: Path) -> None:
    """Test that reading a directory raises an error."""
    ry.cd(tmp_path)
    (tmp_path / "test").mkdir(parents=True)
    with pytest.raises(OSError):
        async for _ in ry.read_stream_async(tmp_path, chunk_size=10):
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
        with pytest.raises(ValueError):
            _ = await ry.read_stream_async(
                "test.txt", offset=read_offset, strict=strict
            )
    else:
        rs = await ry.read_stream_async("test.txt", offset=read_offset, strict=strict)
        chunks = await rs.collect()
        assert b"".join(chunks) == b""
        assert len(chunks) == 0


class _FileReadStreamOptionsDict(t.TypedDict, total=False):
    chunk_size: int
    offset: int
    buffered: bool
    strict: bool


@pytest.mark.parametrize("chunk_size", [None, 1, 100, 512, 1024])
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
        chunk_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> _FileReadStreamOptionsDict:
        kwargs: _FileReadStreamOptionsDict = {}
        if chunk_size is not None:
            kwargs["chunk_size"] = chunk_size
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
        chunk_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> None:
        string_bytes = self._setup_method(tmp_path)
        stream = await ry.read_stream_async(
            "test.txt",
            **self._build_kwargs(
                chunk_size,
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
        chunk_size: int | None,
        offset: int | None,
        *,
        buffered: bool | None,
        strict: bool | None,
    ) -> None:
        string_bytes = self._setup_method(tmp_path)
        stream = await ry.read_stream_async(
            "test.txt",
            **self._build_kwargs(
                chunk_size,
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
        chunk_size: int | None,
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
            chunk_size,
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
                    f"chunk_size={chunk_size}"
                    if chunk_size is not None
                    else f"chunk_size={_DEFAULT_CHUNK_SIZE}",
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
    def test_chunk_size_zero_raises(self) -> None:
        """Test that chunk_size of zero raises ValueError."""
        with pytest.raises(ValueError):
            _rs = ry.read_stream_async(_THIS_FILEPATH_ABOSLUTE, chunk_size=0)
