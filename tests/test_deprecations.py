import bz2
import re
import typing as t
import warnings

import pytest

import ry


def _deprecation_message(
    old: str, new: str, removal_version: str, *, escape: bool = True
) -> str:
    msg = f"`{old}` is deprecated; use `{new}` instead [removal: {removal_version}]"
    return re.escape(msg) if escape else msg


def test_deprecation_msg() -> None:
    assert (
        _deprecation_message("old_func", "new_func", "v0.1.0", escape=False)
        == "`old_func` is deprecated; use `new_func` instead [removal: v0.1.0]"
    )


class TestDeprecationWarningEg:
    def _deprecated_fn(self) -> None:
        msg = _deprecation_message(
            "ry.deprecated_fn", "ry.new_fn", "v0.1.0", escape=False
        )
        warnings.warn(msg, DeprecationWarning, stacklevel=1)

    def test_deprecated_fn_warning(self) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message("ry.deprecated_fn", "ry.new_fn", "v0.1.0"),
        ):
            self._deprecated_fn()


class TestGzipDeprecations:
    """`*_encode`/`*_decode` aliases -> `gzip_compress`/`gzip_decompress`"""

    _DATA = b"XXXXXXXXXXYYYYYYYYYY"

    @pytest.mark.parametrize(
        ("old", "new"),
        [
            ("gzip_encode", "gzip_compress"),
            ("gzip", "gzip_compress"),
        ],
    )
    def test_compress_aliases(self, old: str, new: str) -> None:
        expected = getattr(ry, new)(self._DATA, level=9)
        msg = _deprecation_message(old, new, "v0.0.109")
        # positional + `quality=` keep working until removal
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA, 9) == expected
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA, quality=9) == expected

    @pytest.mark.parametrize(
        ("old", "new"),
        [
            ("gzip_decode", "gzip_decompress"),
            ("gunzip", "gzip_decompress"),
            ("is_gzipped", "is_gzip"),
        ],
    )
    def test_aliases(self, old: str, new: str) -> None:
        compressed = ry.gzip_compress(self._DATA)
        with pytest.warns(
            DeprecationWarning, match=_deprecation_message(old, new, "v0.0.109")
        ):
            assert getattr(ry, old)(compressed) == getattr(ry, new)(compressed)

    def test_quality_value_error(self) -> None:
        with (
            pytest.warns(
                DeprecationWarning,
                match=_deprecation_message("gzip_encode", "gzip_compress", "v0.0.109"),
            ),
            pytest.raises(ValueError, match="Invalid compression level"),
        ):
            ry.gzip_encode(b"data", quality=10)  # type: ignore[arg-type,deprecated]  # ty:ignore[invalid-argument-type, deprecated]


class TestBrotliDeprecations:
    """`brotli`/`brotli_encode`/`brotli_decode` -> `brotli_compress`/`brotli_decompress`"""

    _DATA = b"XXXXXXXXXXYYYYYYYYYY"

    @pytest.mark.parametrize("old", ["brotli_encode", "brotli"])
    def test_compress_aliases(self, old: str) -> None:
        msg = _deprecation_message(old, "brotli_compress", "v0.0.109")
        expected = ry.brotli_compress(self._DATA, level=5)
        # positional + `quality=` keep working until removal
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA, 5) == expected
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA, quality=5) == expected
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA) == expected

    @pytest.mark.parametrize("old", ["brotli_encode", "brotli"])
    @pytest.mark.parametrize("magic_number", [False, True])
    def test_magic_number_removed(self, old: str, *, magic_number: bool) -> None:
        with pytest.raises(TypeError, match="magic_number"):
            getattr(ry, old)(self._DATA, magic_number=magic_number)

    def test_decompress_alias(self) -> None:
        compressed = ry.brotli_compress(self._DATA)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "brotli_decode", "brotli_decompress", "v0.0.109"
            ),
        ):
            assert ry.brotli_decode(compressed) == ry.brotli_decompress(compressed)  # type: ignore[deprecated]  # ty:ignore[deprecated]

    @pytest.mark.parametrize("old", ["brotli_encode", "brotli"])
    @pytest.mark.parametrize("quality", [6, 11, 12])
    def test_quality_value_error(self, old: str, quality: int) -> None:
        with (
            pytest.warns(
                DeprecationWarning,
                match=_deprecation_message(old, "brotli_compress", "v0.0.109"),
            ),
            pytest.raises(ValueError, match="Compression level must be an integer 0-5"),
        ):
            getattr(ry, old)(b"data", quality=quality)


class TestBzip2Deprecations:
    """bzip2 et al fns are deprecated in favor of python std lib"""

    _DATA = b"XXXXXXXXXXYYYYYYYYYY"

    @pytest.mark.parametrize("old", ["bzip2_encode", "bzip2"])
    def test_compress_aliases(self, old: str) -> None:
        msg = _deprecation_message(old, "bz2.compress", "v0.0.109")
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA) == bz2.compress(self._DATA, 6)
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA, 9) == bz2.compress(self._DATA, 9)
        with pytest.warns(DeprecationWarning, match=msg):
            assert getattr(ry, old)(self._DATA, quality="fast") == bz2.compress(
                self._DATA, 1
            )

    def test_decompress_alias(self) -> None:
        compressed = bz2.compress(self._DATA)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message("bzip2_decode", "bz2.decompress", "v0.0.109"),
        ):
            assert ry.bzip2_decode(compressed) == bz2.decompress(compressed)  # type: ignore[deprecated]  # ty:ignore[deprecated]

    def test_quality_value_error(self) -> None:
        with (
            pytest.warns(
                DeprecationWarning,
                match=_deprecation_message("bzip2_encode", "bz2.compress", "v0.0.109"),
            ),
            pytest.raises(ValueError, match="Invalid compression level"),
        ):
            ry.bzip2_encode(b"data", quality=10)  # type: ignore[arg-type,deprecated]  # ty:ignore[invalid-argument-type, deprecated]


class TestZstdDeprecations:
    """flat aliases + the whole `ry.zstd` submodule -> flat `ry.zstd_*`"""

    _DATA = b"XXXXXXXXXXYYYYYYYYYY"

    def test_zstd_encode(self) -> None:
        msg = _deprecation_message("zstd_encode", "zstd_compress", "v0.0.109")
        expected = ry.zstd_compress(self._DATA, 9)
        with pytest.warns(DeprecationWarning, match=msg):
            assert ry.zstd_encode(self._DATA, 9) == expected  # type: ignore[deprecated]  # ty:ignore[deprecated]
        with pytest.warns(DeprecationWarning, match=msg):
            assert ry.zstd_encode(self._DATA, level=9) == expected  # type: ignore[deprecated]  # ty:ignore[deprecated]

    def test_zstd_decode(self) -> None:
        compressed = ry.zstd_compress(self._DATA)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message("zstd_decode", "zstd_decompress", "v0.0.109"),
        ):
            assert ry.zstd_decode(compressed) == self._DATA  # type: ignore[deprecated]  # ty:ignore[deprecated]

    @pytest.mark.parametrize(
        ("old", "new"),
        [
            ("decompress", "zstd_decompress"),
            ("decode", "zstd_decompress"),
            ("unzstd", "zstd_decompress"),
            ("is_zstd", "is_zstd"),
        ],
    )
    def test_submodule_functions(self, old: str, new: str) -> None:
        compressed = ry.zstd_compress(self._DATA)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(f"ry.zstd.{old}", f"ry.{new}", "v0.0.109"),
        ):
            assert getattr(ry.zstd, old)(compressed) == getattr(ry, new)(compressed)

    def test_submodule_compress(self) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "ry.zstd.compress", "ry.zstd_compress", "v0.0.109"
            ),
        ):
            assert ry.zstd.compress(self._DATA, 9) == ry.zstd_compress(self._DATA, 9)  # type: ignore[deprecated]  # ty:ignore[deprecated]

    @pytest.mark.parametrize(
        "py_expr",
        ["from ry import zstd", "import ry.zstd", "from ry.ryo3 import zstd"],
    )
    def test_submodule_still_importable(self, py_expr: str) -> None:
        import subprocess as sp
        import sys

        result = sp.run(
            [sys.executable, "-W", "error::DeprecationWarning", "-c", py_expr],
            capture_output=True,
            text=True,
        )
        # importing alone doesn't warn; only using the old names does
        assert result.returncode == 0, result.stderr


class TestStringifyPybytesDeprecation:
    """`pybytes=` -> `dtype=bytes` on `stringify`/`JSON.stringify`/`JSON.dumps`"""

    _MSG = _deprecation_message("pybytes", "dtype=bytes", "v0.0.109")

    @pytest.mark.parametrize(
        "fn",
        [ry.stringify, ry.JSON.stringify, ry.JSON.dumps],
        ids=lambda f: f.__name__,
    )
    @pytest.mark.parametrize(
        ("pybytes", "expected"), [(True, bytes), (False, ry.Bytes)]
    )
    def test_pybytes_warns_and_still_works(
        self, fn: t.Any, *, pybytes: bool, expected: type
    ) -> None:
        with pytest.warns(DeprecationWarning, match=self._MSG):
            res = fn({"a": 1}, pybytes=pybytes)
        assert bytes(res) == b'{"a":1}'
        assert type(res) is expected

    def test_pybytes_and_dtype_mutually_exclusive(self) -> None:
        with pytest.raises(TypeError, match="mutually exclusive"):
            ry.stringify({"a": 1}, pybytes=True, dtype=bytes)  # type: ignore[call-overload]  # ty: ignore[no-matching-overload]

    def test_dtype_does_not_warn(self) -> None:
        with warnings.catch_warnings():
            warnings.simplefilter("error", DeprecationWarning)
            ry.stringify({"a": 1})
            ry.stringify({"a": 1}, dtype=bytes)
            ry.JSON.dumps({"a": 1}, dtype=ry.Bytes)
