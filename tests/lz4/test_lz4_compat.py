from __future__ import annotations

import pytest

from ry import dev as ry

_JSONISH = b'{"name":"ry","kind":"lz4","value":123456789}\n' * 32

# =============================================================================
# INTEROP w/ python-lz4
# =============================================================================
lz4 = pytest.importorskip("lz4")


class TestLz4PythonInterop:
    def test_frame_stores_content_size_by_default(self) -> None:
        import lz4.frame

        compressed = ry.lz4_compress(_JSONISH)
        frame_info = lz4.frame.get_frame_info(bytes(compressed))
        assert frame_info["content_size"] == len(_JSONISH)

    def test_frame_interop_python_lz4_decompresses_ours(self) -> None:
        import lz4.frame

        compressed = ry.lz4_compress(_JSONISH)
        assert lz4.frame.decompress(bytes(compressed)) == _JSONISH

    def test_frame_interop_we_decompress_python_lz4(self) -> None:
        import lz4.frame

        compressed = lz4.frame.compress(_JSONISH)
        assert ry.lz4_decompress(compressed) == _JSONISH

    def test_block_interop_python_lz4(self) -> None:
        import lz4.block

        # size=True matches python-lz4's default (store_size=True) u32-le prefix
        prefixed = ry.lz4_compress_block(_JSONISH, size=True)
        assert lz4.block.decompress(bytes(prefixed)) == _JSONISH
        assert ry.lz4_decompress_block(lz4.block.compress(_JSONISH)) == _JSONISH

    def test_block_interop_python_lz4_raw(self) -> None:
        import lz4.block

        compressed = ry.lz4_compress_block(_JSONISH, size=False)
        assert (
            lz4.block.decompress(bytes(compressed), uncompressed_size=len(_JSONISH))
            == _JSONISH
        )
        py_compressed = lz4.block.compress(_JSONISH, store_size=False)
        assert ry.lz4_decompress_block(py_compressed, len(_JSONISH)) == _JSONISH
