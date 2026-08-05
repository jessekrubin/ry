import json
import math
import platform
import subprocess
import sys
import typing as t

import pytest

import ry

np = pytest.importorskip("numpy")


def _loads(value: t.Any, **kwargs: t.Any) -> object:
    return json.loads(bytes(ry.stringify(value, numpy=True, **kwargs)))


@pytest.mark.parametrize(
    ("dtype", "values"),
    [
        (np.bool_, [True, False, True]),
        (np.int8, [-128, 0, 127]),
        (np.int16, [-32768, 0, 32767]),
        (np.int32, [-(2**31), 0, 2**31 - 1]),
        (np.int64, [-(2**63), 0, 2**63 - 1]),
        (np.uint8, [0, 127, 255]),
        (np.uint16, [0, 32767, 65535]),
        (np.uint32, [0, 2**31, 2**32 - 1]),
        (np.uint64, [0, 2**63, 2**64 - 1]),
        (np.float32, [-1.25, 0.0, 1.25]),
        (np.float64, [-1.25, 0.0, 1.25]),
    ],
)
def test_numpy_supported_arrays(dtype: t.Any, values: list[t.Any]) -> None:
    array = np.array(values, dtype=dtype)
    assert _loads(array) == values


@pytest.mark.parametrize(
    ("scalar", "expected"),
    [
        (np.bool_(1), True),
        (np.int8(-8), -8),
        (np.int16(-16), -16),
        (np.int32(-32), -32),
        (np.int64(-64), -64),
        (np.uint8(8), 8),
        (np.uint16(16), 16),
        (np.uint32(32), 32),
        (np.uint64(64), 64),
        (np.float32(1.25), 1.25),
        (np.float64(2.5), 2.5),
    ],
)
def test_numpy_supported_scalars(scalar: t.Any, expected: t.Any) -> None:
    assert _loads(scalar) == expected


def test_numpy_scalar_subclass_uses_default() -> None:
    class Int64Subclass(np.int64):
        pass

    scalar = Int64Subclass(42)
    assert _loads(scalar, default=int) == 42


def test_numpy_nested_array() -> None:
    array = np.arange(24, dtype=np.int64).reshape(2, 3, 4)
    assert _loads({"array": array}) == {"array": array.tolist()}


@pytest.mark.parametrize("shape", [(0,), (2, 0), (2, 0, 3), (1, 2, 0, 4)])
def test_numpy_empty_array(shape: tuple[int, ...]) -> None:
    array = np.empty(shape, dtype=np.int32)
    assert _loads(array) == array.tolist()


def test_numpy_unaligned_c_contiguous_array() -> None:
    backing = bytearray(1 + 4 * np.dtype(np.int64).itemsize)
    array = np.ndarray(shape=(4,), dtype=np.int64, buffer=backing, offset=1)
    array[:] = [1, 2, 3, 4]
    assert array.flags.c_contiguous
    assert not array.flags.aligned
    assert _loads(array) == [1, 2, 3, 4]


def test_numpy_accepts_array_that_is_c_and_f_contiguous() -> None:
    array = np.asfortranarray(np.arange(4, dtype=np.int64))
    assert array.flags.c_contiguous
    assert array.flags.f_contiguous
    assert _loads(array) == [0, 1, 2, 3]


def test_numpy_non_finite_floats_are_null() -> None:
    array = np.array([math.nan, math.inf, -math.inf], dtype=np.float64)
    assert bytes(ry.stringify(array, numpy=True)) == b"[null,null,null]"


def test_numpy_options_and_output_types() -> None:
    value = {"b": np.arange(3, dtype=np.int16), "a": np.bool_(1)}
    result = ry.JSON.dumps(
        value,
        numpy=True,
        fmt=True,
        sort_keys=True,
        append_newline=True,
        pybytes=True,
    )
    assert isinstance(result, bytes)
    assert result.endswith(b"\n")
    assert json.loads(result) == {"a": True, "b": [0, 1, 2]}


class TestNpImporting:
    def test_numpy_false_does_not_import_numpy(self) -> None:
        code = "import sys, ry; assert 'numpy' not in sys.modules; ry.stringify({'x': [1, 2, 3]}); assert 'numpy' not in sys.modules"
        subprocess.run([sys.executable, "-c", code], check=True)

    def test_numpy_import_error_is_preserved(self) -> None:
        code = ry.unindent("""
        import builtins
        import ry

        real_import = builtins.__import__
        def blocked(name, *args, **kwargs):
            if name == "numpy":
                raise ModuleNotFoundError("No module named 'numpy'", name="numpy")
            return real_import(name, *args, **kwargs)

        builtins.__import__ = blocked
        try:
            ry.stringify(1, numpy=True)
        except ModuleNotFoundError as exc:
            assert exc.name == "numpy"
        else:
            raise AssertionError("expected ModuleNotFoundError")
        """)
        subprocess.run([sys.executable, "-c", code], check=True)


class TestNumpyErrors:
    def test_numpy_rejects_non_native_endian(self) -> None:
        native = np.dtype(np.int32)
        swapped = native.newbyteorder(">" if sys.byteorder == "little" else "<")
        array = np.arange(4, dtype=swapped)
        with pytest.raises(TypeError, match="native byte order"):
            ry.stringify(array, numpy=True)

    def test_numpy_rejects_zero_dimensional_array(self) -> None:
        with pytest.raises(TypeError, match="zero-dimensional"):
            ry.stringify(np.array(42, dtype=np.int64), numpy=True)

    def test_numpy_rejects_more_than_32_dimensions(self) -> None:
        def _has_32_dims_limit() -> bool:
            try:
                np.empty((1,) * 33, dtype=np.int8)
            except ValueError:
                return True
            return False

        if not _has_32_dims_limit():
            pytest.skip("np version does not enforce a 32-dimension limit")
        _array = np.empty((1,) * 33, dtype=np.int8)
        with pytest.raises(TypeError, match="more than 32 dimensions"):
            ry.stringify(_array, numpy=True)

    def test_numpy_scalar_requires_opt_in(self) -> None:
        scalar = np.int64(42)
        with pytest.raises(TypeError, match="not json-serializable"):
            ry.stringify(scalar)
        assert json.loads(bytes(ry.stringify(scalar, default=int))) == 42

    @pytest.mark.parametrize(
        "array",
        [
            np.arange(12, dtype=np.int64).reshape(3, 4)[:, ::2],
            np.asfortranarray(np.arange(12, dtype=np.int64).reshape(3, 4)),
        ],
    )
    def test_numpy_rejects_non_c_contiguous(self, array: t.Any) -> None:
        with pytest.raises(TypeError, match="must be C-contiguous"):
            ry.stringify(array, numpy=True, default=lambda _: "fallback")

    @pytest.mark.parametrize(
        "array",
        [
            np.array([1.0], dtype=np.float16),
            np.array([1 + 2j], dtype=np.complex64),
            np.array([object()], dtype=object),
            np.array(["hello"], dtype=np.str_),
            np.array(["2025-01-01"], dtype="datetime64[D]"),
            np.array([1], dtype="timedelta64[D]"),
        ],
    )
    def test_numpy_rejects_unsupported_array_dtype(self, array: t.Any) -> None:
        with pytest.raises(TypeError, match=r"dtype is not supported|float16"):
            ry.stringify(array, numpy=True, default=lambda _: "fallback")

    def test_numpy_rejects_float16_scalar(self) -> None:
        with pytest.raises(TypeError, match="float16 scalar is not supported"):
            ry.stringify(np.float16(1.25), numpy=True, default=lambda _: "fallback")

    @pytest.mark.skipif(
        platform.python_implementation() == "CPython",
        reason="non-CPython behavior",
    )
    def test_numpy_mode_is_cpython_only(self) -> None:
        with pytest.raises(NotImplementedError, match="CPython only"):
            ry.stringify(1, numpy=True)
