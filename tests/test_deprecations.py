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
