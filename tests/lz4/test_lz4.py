from __future__ import annotations

import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from ry.ryo3._lz4rip import _Lz4BlockMode, _Lz4BlockSize
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
        assert ry.lz4_decompress_block(ry.lz4_compress_block(b"", size=True)) == b""
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
        assert ry.lz4_decompress(ry.lz4_compress(b"")) == b""

    def test_frame_round_trip_large(self) -> None:
        data = _JSONISH * 1024  # ~1.4mb, spans multiple blocks
        assert ry.lz4_decompress(ry.lz4_compress(data)) == data

    def test_frame_round_trip_dictionary(self) -> None:
        compressed = ry.lz4_compress(_JSONISH, dictionary=_DICTIONARY, dict_id=42)
        decompressed = ry.lz4_decompress(compressed, dictionary=_DICTIONARY, dict_id=42)
        assert decompressed == _JSONISH

    def test_frame_info_block_options(
        self, lz4_block_size: _Lz4BlockSize, lz4_block_mode: _Lz4BlockMode
    ) -> None:
        compressed = ry.lz4_compress(
            _JSONISH,
            frame_info={"block_size": lz4_block_size, "block_mode": lz4_block_mode},
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
            ry.lz4_compress(_10X_10Y, frame_info={"block_szie": "auto"})  # type: ignore[arg-type]

    def test_frame_info_invalid_block_size(self) -> None:
        with pytest.raises(ValueError, match="Invalid block-size"):
            ry.lz4_compress(_10X_10Y, frame_info={"block_size": "max-9000kb"})  # type: ignore[arg-type]

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


# =============================================================================
# TRAIN-DICT
# =============================================================================


class TestLz4TrainDict:
    @staticmethod
    def _generate_samples() -> list[bytes]:
        return [b'{"name":"ry","kind":"lz4","value":%d}' % i for i in range(64)]

    def test_train_dict_block_round_trip(self) -> None:
        dictionary = ry.lz4_train_dict(self._generate_samples(), 2048)
        assert isinstance(dictionary, ry.Bytes)
        assert 0 < len(dictionary) <= 2048
        msg = self._generate_samples()[0]
        compressed = ry.lz4_compress_block(msg, dictionary=dictionary)
        assert len(compressed) < len(bytes(ry.lz4_compress_block(msg)))
        decompressed = ry.lz4_decompress_block(
            compressed, len(msg), dictionary=dictionary
        )
        assert decompressed == msg

    def test_train_dict_frame_round_trip(self) -> None:
        dictionary = ry.lz4_train_dict(self._generate_samples(), 2048)
        msg = self._generate_samples()[0]
        compressed = ry.lz4_compress(msg, dictionary=dictionary, dict_id=42)
        decompressed = ry.lz4_decompress(compressed, dictionary=dictionary, dict_id=42)
        assert decompressed == msg

    def test_train_dict_accepts_any_iterable(self) -> None:
        dictionary = ry.lz4_train_dict(iter(self._generate_samples()), 2048)
        assert len(dictionary) > 0

    def test_train_dict_size_capped_at_max_distance(self) -> None:
        dictionary = ry.lz4_train_dict(self._generate_samples(), 1_000_000)
        assert len(dictionary) <= 0xFFFF

    def test_train_dict_too_few_samples(self) -> None:
        with pytest.raises(ValueError, match="dict training failed"):
            ry.lz4_train_dict([b"one-lonely-sample"], 2048)

    def test_train_dict_zero_dict_size(self) -> None:
        with pytest.raises(ValueError, match="dict_size must be positive"):
            ry.lz4_train_dict(self._generate_samples(), 0)

    def test_train_dict_not_iterable(self) -> None:
        with pytest.raises(TypeError):
            ry.lz4_train_dict(123, 2048)  # type: ignore[arg-type]
