from __future__ import annotations

import pytest

from ry import dev as ry

_dev = ry
_10X_10Y = b"XXXXXXXXXXYYYYYYYYYY"
_DICTIONARY = b'{"name":"ry","kind":"lz4","value":' * 8
_JSONISH = b'{"name":"ry","kind":"lz4","value":123456789}\n' * 32


# =============================================================================
# BLOCK
# =============================================================================


class TestLz4Block:
    def test_block_round_trip_size(self) -> None:
        compressed = ry.lz4_compress_block(_10X_10Y, size=True)
        assert isinstance(compressed, ry.Bytes)
        assert bytes(compressed)[:4] == len(_10X_10Y).to_bytes(4, "little")
        decompressed = ry.lz4_decompress_block(compressed)
        assert decompressed == _10X_10Y

    def test_block_round_trip_raw(self) -> None:
        compressed = ry.lz4_compress_block(_10X_10Y, size=False)
        decompressed = ry.lz4_decompress_block(compressed, len(_10X_10Y))
        assert decompressed == _10X_10Y

    def test_block_round_trip_empty(self) -> None:
        assert ry.lz4_decompress_block(_dev.lz4_compress_block(b"", size=True)) == b""
        compressed_raw = ry.lz4_compress_block(b"", size=False)
        assert ry.lz4_decompress_block(compressed_raw, 0) == b""

    def test_block_round_trip_dictionary(self) -> None:
        compressed = ry.lz4_compress_block(_JSONISH, dictionary=_DICTIONARY)
        compressed_nodict = ry.lz4_compress_block(_JSONISH)
        assert len(compressed) <= len(compressed_nodict)
        decompressed = ry.lz4_decompress_block(
            compressed, size=len(_JSONISH), dictionary=_DICTIONARY
        )
        assert decompressed == _JSONISH

    def test_block_compressor_round_trip(self) -> None:
        compressor = ry.Lz4BlockCompressor()
        decompressor = ry.Lz4BlockDecompressor()
        for _ in range(3):  # reusable
            prefixed = compressor.compress(_JSONISH, size=True)
            assert decompressor.decompress(prefixed) == _JSONISH
            raw = compressor.compress(_JSONISH)
            assert decompressor.decompress(raw, len(_JSONISH)) == _JSONISH

    def test_block_dict_compressor_round_trip(self) -> None:
        # NOTE: output is not byte-identical to `lz4_compress_block(dictionary=...)`
        # (the reusable compressor uses a different hash-table size), but both are
        # valid lz4 block data
        compressor = ry.Lz4BlockCompressor(_DICTIONARY)
        decompressor = ry.Lz4BlockDecompressor(_DICTIONARY)
        compressed = compressor.compress(_JSONISH, size=True)
        assert decompressor.decompress(compressed) == _JSONISH
        raw = compressor.compress(_JSONISH)
        decompressed = ry.lz4_decompress_block(
            raw, len(_JSONISH), dictionary=_DICTIONARY
        )
        assert decompressed == _JSONISH

    def test_block_decompress_wtf_is_this(self) -> None:
        with pytest.raises(
            ValueError,
            match="block decompression error: literal is out of bounds of the input",
        ):
            ry.lz4_decompress_block(b"\xff\xff\xff\xff", 128)

    def test_block_decompress_prefix_too_short(self) -> None:
        with pytest.raises(ValueError, match="input too short for u32-le size prefix"):
            ry.lz4_decompress_block(b"\x01\x02")

    def test_block_decompress_prefix_lies(self) -> None:
        with pytest.raises(ValueError, match="impossibly large"):
            ry.lz4_decompress_block(b"\xff\xff\xff\xff\x00")

    def test_block_decompress_prefix_mismatch(self) -> None:
        compressed = bytes(ry.lz4_compress_block(_10X_10Y, size=False))
        wrong_prefix = (len(_10X_10Y) + 5).to_bytes(4, "little")
        with pytest.raises(ValueError, match=r"size prefix .* != decompressed size"):
            ry.lz4_decompress_block(wrong_prefix + compressed)


# =============================================================================
# FRAME
# =============================================================================


class TestLz4Frame:
    def test_frame_round_trip(self) -> None:
        compressed = ry.lz4_compress(_10X_10Y)
        assert isinstance(compressed, ry.Bytes)
        decompressed = ry.lz4_decompress(compressed)
        assert decompressed == _10X_10Y

    def test_frame_round_trip_empty(self) -> None:
        assert ry.lz4_decompress(_dev.lz4_compress(b"")) == b""

    def test_frame_round_trip_large(self) -> None:
        data = _JSONISH * 1024  # ~1.4mb, spans multiple blocks
        assert ry.lz4_decompress(_dev.lz4_compress(data)) == data

    def test_frame_round_trip_dictionary(self) -> None:
        compressed = ry.lz4_compress(_JSONISH, dictionary=_DICTIONARY, dict_id=42)
        decompressed = ry.lz4_decompress(compressed, dictionary=_DICTIONARY, dict_id=42)
        assert decompressed == _JSONISH

    @pytest.mark.parametrize("block_size", ["auto", "max-64kb", "max-256kb", 6, 7])
    @pytest.mark.parametrize("block_mode", ["independent", "linked"])
    def test_frame_info_block_options(
        self, block_size: str | int, block_mode: str
    ) -> None:
        compressed = ry.lz4_compress(
            _JSONISH,
            frame_info={"block_size": block_size, "block_mode": block_mode},  # type: ignore[typeddict-item]
        )
        assert ry.lz4_decompress(compressed) == _JSONISH

    def test_frame_info_checksums(self) -> None:
        compressed = ry.lz4_compress(
            _JSONISH,
            frame_info={"content_checksum": True, "block_checksums": True},
        )
        assert ry.lz4_decompress(compressed) == _JSONISH

    def test_frame_info_content_size(self) -> None:
        compressed = ry.lz4_compress(
            _JSONISH, frame_info={"content_size": len(_JSONISH)}
        )
        assert ry.lz4_decompress(compressed) == _JSONISH

    def test_frame_info_invalid_key(self) -> None:
        with pytest.raises(ValueError, match="Invalid FrameInfo key: block_szie"):
            ry.lz4_compress(_10X_10Y, frame_info={"block_szie": "auto"})  # type: ignore[typeddict-unknown-key]

    def test_frame_info_invalid_block_size(self) -> None:
        with pytest.raises(ValueError, match="Invalid block-size"):
            ry.lz4_compress(_10X_10Y, frame_info={"block_size": "max-9000kb"})  # type: ignore[typeddict-item]

    def test_frame_decompress_wtf_is_this(self) -> None:
        with pytest.raises(OSError, match="wrong magic number"):
            ry.lz4_decompress(b"this is not lz4 frame data")

    def test_frame_compressor_round_trip(self) -> None:
        compressor = ry.Lz4FrameCompressor()
        chunks = [compressor.compress(_JSONISH) for _ in range(4)]
        chunks.append(compressor.finish())
        compressed = b"".join(bytes(c) for c in chunks)
        assert ry.lz4_decompress(compressed) == _JSONISH * 4

    def test_frame_compressor_flush(self) -> None:
        compressor = ry.Lz4FrameCompressor()
        data = b"a" * 10
        chunk = compressor.compress(data)
        flushed = compressor.flush()
        assert len(bytes(chunk) + bytes(flushed)) > 0
        tail = compressor.finish()
        compressed = bytes(chunk) + bytes(flushed) + bytes(tail)
        assert ry.lz4_decompress(compressed) == data

    def test_frame_compressor_dictionary(self) -> None:
        compressor = ry.Lz4FrameCompressor(dictionary=_DICTIONARY, dict_id=42)
        compressed = bytes(compressor.compress(_JSONISH)) + bytes(compressor.finish())
        decompressed = ry.lz4_decompress(compressed, dictionary=_DICTIONARY, dict_id=42)
        assert decompressed == _JSONISH

    def test_frame_compressor_finished_is_finished(self) -> None:
        compressor = ry.Lz4FrameCompressor()
        compressor.compress(_10X_10Y)
        compressor.finish()
        with pytest.raises(ValueError, match="finished"):
            compressor.compress(_10X_10Y)
        with pytest.raises(ValueError, match="finished"):
            compressor.flush()
        with pytest.raises(ValueError, match="finished"):
            compressor.finish()
