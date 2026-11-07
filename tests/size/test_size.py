from __future__ import annotations

import dataclasses
import pickle
import typing as t
from typing import TYPE_CHECKING

import pytest

import ry
from ry import Size, SizeFormatter

if TYPE_CHECKING:
    from ry.ryo3._size import _TFormatSizeBase, _TFormatSizeStyle

_FORMAT_SIZE_BASES: list[_TFormatSizeBase] = [2, 10]
_FORMAT_SIZE_STYLES: list[_TFormatSizeStyle] = [
    "default",
    "abbreviated",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
]


class _FmtKwargs(t.TypedDict, total=False):
    base: _TFormatSizeBase
    style: _TFormatSizeStyle


@dataclasses.dataclass(frozen=True)
class _FmtOptions:
    base: _TFormatSizeBase | None = None
    style: _TFormatSizeStyle | None = None

    def as_kwargs(self) -> _FmtKwargs:
        kw: _FmtKwargs = {}
        if self.base:
            kw["base"] = self.base
        if self.style:
            kw["style"] = self.style
        return kw

    @property
    def expected_style(self) -> _TFormatSizeStyle:
        return self.style or "default"

    @property
    def expected_base(self) -> t.Literal[2, 10]:
        return self.base or 2


@pytest.fixture(
    params=[
        _FmtOptions(base=base, style=style)
        for base in [None, *_FORMAT_SIZE_BASES]
        for style in [None, *_FORMAT_SIZE_STYLES]
    ]
)
def fmt_options(request: pytest.FixtureRequest) -> _FmtOptions:
    return t.cast("_FmtOptions", request.param)


_SIZES = [
    0,
    (2**63) - 1,  # max i64
    (2**63) * -1,  # min i64
    *(10**i for i in range(19)),
    *(-n for n in (10**i for i in range(19))),
]
# i64::MAX fmts to "8 EiB" which is 2**63 parsing errs
_ROUNDTRIP_SIZES = [n for n in _SIZES if n != (2**63) - 1]


def _assert_roundtrip_close(size: int, formatted: str, parsed: int) -> None:
    # parsed wont be EXACTLY the same as size, but it should be close
    # enough for the purposes of this test
    lowered = formatted.lower()
    if lowered.endswith((" bytes", " b")):
        assert parsed == size
    elif lowered.endswith((" eb", " eib", "exabytes", "exbibytes")):
        # exa-units are formatted w/ 0 decimals (size>=0.6) so allow more slop
        assert abs(parsed - size) / abs(size) < 0.05
    else:
        # make sure it is at most 1% off
        assert abs(parsed - size) / abs(size) < 0.01


def test_fmt_parse_roundtrip(fmt_options: _FmtOptions) -> None:
    for size in _ROUNDTRIP_SIZES:
        formatted = ry.fmt_size(size, **fmt_options.as_kwargs())
        parsed = ry.parse_size(formatted)
        _assert_roundtrip_close(size, formatted, parsed)


def test_fmt_parse_formatter(fmt_options: _FmtOptions) -> None:
    formatter = ry.SizeFormatter(**fmt_options.as_kwargs())

    for size in _ROUNDTRIP_SIZES:
        formatted = formatter.format(size)
        formatted_via_call = formatter(size)
        assert formatted == formatted_via_call
        parsed = ry.parse_size(formatted)
        size_obj = Size(size)
        formatted_struct = size_obj.format(**fmt_options.as_kwargs())
        assert formatted == formatted_struct
        _assert_roundtrip_close(size, formatted, parsed)


class TestSizeObj:
    def test_size_creation(self) -> None:
        size = Size(1024)
        assert int(size) == 1024
        assert str(size) == "1.00 KiB"
        assert repr(size) == "Size(1024)"

    def test_size_comparisons(self) -> None:
        size1 = Size(1024)
        size2 = Size(2048)

        assert size1 < size2
        assert size1 <= size2
        assert size2 > size1
        assert size2 >= size1
        assert size1 != size2
        assert size1 == Size(1024)

    def test_size_arithmetic(self) -> None:
        size1 = Size(1024)
        size2 = Size(2048)
        assert (size1 + size2) == Size(3072)
        assert (size2 - size1) == Size(1024)
        assert (size1 * 2) == Size(2048)
        assert (-size1) == Size(-1024)
        assert (+size1) == Size(1024)
        assert (~size1) == Size(~1024)

    def test_size_parsing(self) -> None:
        size = Size.parse("1KB")
        assert int(size) == 1000

        with pytest.raises(ValueError, match="Error parsing Size"):
            Size.parse("invalid")

    def test_size_from_methods(self) -> None:
        # kib ~ 1024
        assert int(Size.from_kib(1)) == 1024
        assert int(Size.from_kibibytes(1)) == 1024
        # kb ~ 1000
        assert int(Size.from_kb(1)) == 1000
        assert int(Size.from_kilobytes(1)) == 1000
        # mib ~ 1024 * 1024
        assert int(Size.from_mib(1)) == 1024 * 1024
        assert int(Size.from_mebibytes(1)) == 1024 * 1024
        # mb ~ 1000 * 1000
        assert int(Size.from_mb(1)) == 1000 * 1000
        assert int(Size.from_megabytes(1)) == 1000 * 1000
        # gib ~ 1024 * 1024 * 1024
        assert int(Size.from_gib(1)) == 1024 * 1024 * 1024
        assert int(Size.from_gibibytes(1)) == 1024 * 1024 * 1024
        # gb ~ 1000 * 1000 * 1000
        assert int(Size.from_gb(1)) == 1000 * 1000 * 1000
        assert int(Size.from_gigabytes(1)) == 1000 * 1000 * 1000
        # tib ~ 1024 * 1024 * 1024 * 1024
        assert int(Size.from_tib(1)) == 1024 * 1024 * 1024 * 1024
        assert int(Size.from_tebibytes(1)) == 1024 * 1024 * 1024 * 1024
        # tb ~ 1000 * 1000 * 1000 * 1000
        assert int(Size.from_tb(1)) == 1000 * 1000 * 1000 * 1000
        assert int(Size.from_terabytes(1)) == 1000 * 1000 * 1000 * 1000
        # pib  ~ 1024 * 1024 * 1024 * 1024 * 1024
        assert int(Size.from_pib(1)) == 1024 * 1024 * 1024 * 1024 * 1024
        assert int(Size.from_pebibytes(1)) == 1024 * 1024 * 1024 * 1024 * 1024
        # pb ~ 1000 * 1000 * 1000 * 1000 * 1000
        assert int(Size.from_pb(1)) == 1000 * 1000 * 1000 * 1000 * 1000
        assert int(Size.from_petabytes(1)) == 1000 * 1000 * 1000 * 1000 * 1000
        # eib ~ 1024 * 1024 * 1024 * 1024 * 1024 * 1024
        assert int(Size.from_eib(1)) == 1024 * 1024 * 1024 * 1024 * 1024 * 1024
        assert int(Size.from_exbibytes(1)) == 1024 * 1024 * 1024 * 1024 * 1024 * 1024
        # eb ~ 1000 * 1000 * 1000 * 1000 * 1000 * 1000
        assert int(Size.from_eb(1)) == 1000 * 1000 * 1000 * 1000 * 1000 * 1000
        assert int(Size.from_exabytes(1)) == 1000 * 1000 * 1000 * 1000 * 1000 * 1000


def test_weird_off_by_one_multiplication() -> None:
    si = 94906265
    i = 94906267
    expected = si * i
    size_obj = Size(si)
    result = size_obj * i
    assert result == expected, f"Expected {expected}, got {result} for si={si}, i={i}"


def test_size_formatter_pickling(fmt_options: _FmtOptions) -> None:
    formatter = ry.SizeFormatter(**fmt_options.as_kwargs())
    unpickled = pickle.loads(pickle.dumps(formatter))
    assert formatter == unpickled


def test_size_formatter_equality(fmt_options: _FmtOptions) -> None:
    formatter = ry.SizeFormatter(**fmt_options.as_kwargs())

    _base_expected = fmt_options.expected_base
    _style_expected = fmt_options.expected_style
    # _style_expected: t.Literal[
    #     "default",
    #     "abbreviated",
    #     "abbreviated-lowercase",
    #     "full",
    #     "full-lowercase",
    # ] = "default" if style is None or style is Ellipsis else style.replace("_", "-")
    assert (
        repr(formatter)
        == f"SizeFormatter(base={_base_expected!r}, style={_style_expected!r}, scale=None)"
    )
    assert formatter == ry.SizeFormatter(base=_base_expected, style=_style_expected)

    assert formatter.base == _base_expected
    assert formatter.style == _style_expected
    assert formatter.scale is None
    assert formatter == eval(repr(formatter), {"SizeFormatter": ry.SizeFormatter})


class TestScale:
    def test_fmt_size_scale(self) -> None:
        assert ry.fmt_size(1536) == "1.50 KiB"
        assert ry.fmt_size(1536, scale=0) == "2 KiB"
        assert ry.fmt_size(1536, scale=3) == "1.500 KiB"
        assert ry.fmt_size(1500, base=10, scale=1) == "1.5 KB"
        # bytes are always whole numbers
        assert ry.fmt_size(10, scale=3) == "10 bytes"

    def test_size_format_scale(self) -> None:
        assert Size(1536).format(scale=3) == ry.fmt_size(1536, scale=3)

    def test_formatter_scale(self) -> None:
        formatter = ry.SizeFormatter(scale=3)
        assert formatter.scale == 3
        assert formatter(1536) == "1.500 KiB"
        assert repr(formatter) == "SizeFormatter(base=2, style='default', scale=3)"
        assert formatter == eval(repr(formatter), {"SizeFormatter": ry.SizeFormatter})
        assert formatter != ry.SizeFormatter()
        assert formatter.with_scale(None) == ry.SizeFormatter()
        assert ry.SizeFormatter().with_scale(3) == formatter
        # scale survives with_base/with_style
        assert formatter.with_base(10).scale == 3
        assert formatter.with_style("full").scale == 3

    def test_formatter_scale_pickle(self) -> None:
        formatter = ry.SizeFormatter(base=10, style="full", scale=1)
        unpickled = pickle.loads(pickle.dumps(formatter))
        assert unpickled == formatter
        assert unpickled.scale == 1

    def test_scale_max(self) -> None:
        assert len(ry.fmt_size(1536, scale=255)) == len("1.") + 255 + len(" KiB")

    @pytest.mark.parametrize("scale", [-1, 256, 65_536, 2**64])
    def test_scale_out_of_range(self, scale: int) -> None:
        with pytest.raises(OverflowError):
            ry.fmt_size(1536, scale=scale)
        with pytest.raises(OverflowError):
            Size(1536).format(scale=scale)
        with pytest.raises(OverflowError):
            ry.SizeFormatter(scale=scale)
        with pytest.raises(OverflowError):
            ry.SizeFormatter().with_scale(scale)


@pytest.mark.parametrize("s", ["1.5 GB 😱", "1234‍MB", "1é"])
def test_parse_size_non_ascii_errors(s: str) -> None:
    # used to panic in size<0.6 (split_at on a non-char boundary)
    with pytest.raises(ValueError, match="parse size error"):
        ry.parse_size(s)


def test_size_reflected() -> None:
    a = ry.Size(3)
    b = ry.Size(10)
    res_add = a.__radd__(b)  # noqa: PLC2801
    assert isinstance(res_add, ry.Size)
    res_sub = a.__rsub__(b)  # noqa: PLC2801
    assert isinstance(res_sub, ry.Size)
    assert res_add == b + a
    assert res_sub == b - a


@pytest.mark.parametrize("other", [5, 2.5, True])
def test_size_rhs_num(other: float) -> None:
    size = Size(3)
    assert other + size == size + other
    assert isinstance(other + size, Size)
    assert other - size == -(size - other)
    assert isinstance(other - size, Size)


def test_size_rhs_ints() -> None:
    assert 5 + Size(3) == Size(8)
    assert 5 - Size(3) == Size(2)
    assert 3 - Size(5) == Size(-2)
    assert sum([Size(1), Size(2), Size(3)]) == Size(6)
    with pytest.raises(OverflowError):
        _ = -2 - Size(2**63 - 1)


@pytest.mark.parametrize("other", [None, "1 KiB", b"1", object(), [1]])
def test_size_reflected_unknown_types(other: object) -> None:
    s = Size(10)
    assert s.__radd__(other) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]  # noqa: PLC2801
    assert s.__rsub__(other) is NotImplemented  # type: ignore[operator]  # noqa: PLC2801
    with pytest.raises(TypeError):
        _ = other + s  # type: ignore[operator]
    with pytest.raises(TypeError):
        _ = other - s  # type: ignore[operator]


@pytest.mark.parametrize(
    "other",
    [1, 1.0, 1.9, 0.5, True, 2**62, 2**70, -(2**70), float("inf"), float("nan")],
)
def test_size_richcmp_numeric_matches_int(other: float) -> None:
    """Size compares like the python int it wraps (floats are NOT truncated)"""
    for n in (0, 1, 2**62):
        s = Size(n)
        assert (s == other) == (n == other)
        assert (s != other) == (n != other)
        assert (s < other) == (n < other)
        assert (s <= other) == (n <= other)
        assert (s > other) == (n > other)
        assert (s >= other) == (n >= other)
        # reflected
        assert (other == s) == (other == n)
        assert (other < s) == (other < n)


@pytest.mark.parametrize("other", [None, "1 KiB", b"1", object(), [1]])
def test_size_richcmp_unknown_types(other: object) -> None:
    s = Size(1)
    assert s.__eq__(other) is NotImplemented  # noqa: PLC2801
    assert s != other
    assert not (s == other)
    with pytest.raises(TypeError):
        _ = s < other  # type: ignore[operator]


def test_size_formatter_eq() -> None:
    assert SizeFormatter() == SizeFormatter()
    assert SizeFormatter(base=10) != SizeFormatter(base=2)
    assert SizeFormatter().__eq__(None) is NotImplemented  # noqa: PLC2801
    assert SizeFormatter() != None  # noqa: E711


@pytest.mark.parametrize("other", [None, "1 KiB", b"1", object(), [1]])
def test_size_arithmetic_unknown_types(other: object) -> None:
    s = Size(10)
    assert s.__add__(other) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]  # noqa: PLC2801
    assert s.__sub__(other) is NotImplemented  # type: ignore[operator]  # noqa: PLC2801
    assert s.__mul__(other) is NotImplemented  # type: ignore[operator]  # noqa: PLC2801
    assert s.__rmul__(other) is NotImplemented  # type: ignore[operator]  # noqa: PLC2801
    with pytest.raises(TypeError):
        _ = s + other  # type: ignore[operator]
    with pytest.raises(TypeError):
        _ = s - other  # type: ignore[operator]
    with pytest.raises(TypeError):
        _ = s * other  # type: ignore[operator]
    with pytest.raises(TypeError):
        _ = other * s  # type: ignore[operator]


def test_size_arithmetic_number_protocols() -> None:
    """non-int/float numbers still go through `__index__`/`__float__`"""
    from decimal import Decimal  # noqa: PLC0415
    from fractions import Fraction  # noqa: PLC0415

    class Idx:
        def __index__(self) -> int:
            return 7

    class Flt:
        def __float__(self) -> float:
            return 1.5

    s = Size(10)
    assert s + Idx() == Size(17)  # type: ignore[operator]  # ty: ignore[unsupported-operator]
    assert s * Idx() == Size(70)  # type: ignore[operator]  # ty: ignore[unsupported-operator]
    assert s * Flt() == Size(15)  # type: ignore[operator]  # ty: ignore[unsupported-operator]
    assert s * Fraction(5, 2) == Size(25)  # type: ignore[operator]  # ty: ignore[unsupported-operator]
    assert s * Decimal("2.5") == Size(25)  # type: ignore[operator]  # ty: ignore[unsupported-operator]
    assert 3 * s == Size(30)
    with pytest.raises(OverflowError):
        _ = s * 2**63
