import re
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


_SECS_NANOS_EXPECTED = [
    ((0, 0), 0),
    ((1, 0), 1_000_000_000),
    ((0, 1), 1),
    ((1, 1), 1_000_000_001),
    ((1, 500_000_000), 1_500_000_000),
]


class TestDurationIntDeprecation:
    """
    for some insane reason i decided to make `__int__` return nanoseconds...

    probably bc I thought it was more pure/precise, but in practice it was
    a huge pain in the ass so it has been deprecated for both `Duration` and
    `SignedDuration`
    """

    @pytest.mark.parametrize(("secs_nanos", "expected"), _SECS_NANOS_EXPECTED)
    def test_duration_int(self, secs_nanos: tuple[int, int], expected: int) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "Duration.__int__", "Duration.as_nanos", "v0.0.108"
            ),
        ):
            assert int(ry.Duration(*secs_nanos)) == expected

    @pytest.mark.parametrize(
        ("secs_nanos", "expected"),
        [*_SECS_NANOS_EXPECTED, ((-1, -500_000_000), -1_500_000_000)],
    )
    def test_signed_duration_int(
        self, secs_nanos: tuple[int, int], expected: int
    ) -> None:
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "SignedDuration.__int__", "SignedDuration.as_nanos", "v0.0.108"
            ),
        ):
            assert int(ry.SignedDuration(*secs_nanos)) == expected


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
            assert getattr(ry, old)(self._DATA, magic_number=True) == (
                ry.brotli_compress(self._DATA, magic_number=True)
            )

    def test_decompress_alias(self) -> None:
        compressed = ry.brotli_compress(self._DATA)
        with pytest.warns(
            DeprecationWarning,
            match=_deprecation_message(
                "brotli_decode", "brotli_decompress", "v0.0.109"
            ),
        ):
            assert ry.brotli_decode(compressed) == ry.brotli_decompress(compressed)  # type: ignore[deprecated]  # ty:ignore[deprecated]

    def test_quality_value_error(self) -> None:
        with (
            pytest.warns(
                DeprecationWarning,
                match=_deprecation_message(
                    "brotli_encode", "brotli_compress", "v0.0.109"
                ),
            ),
            pytest.raises(
                ValueError, match="Compression level must be an integer 0-11"
            ),
        ):
            ry.brotli_encode(b"data", quality=12)  # type: ignore[arg-type,deprecated]  # ty:ignore[invalid-argument-type, deprecated]


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
