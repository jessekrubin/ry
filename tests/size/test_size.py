from __future__ import annotations

import dataclasses
import pickle
import typing as t
from typing import TYPE_CHECKING

import pytest

import ry
from ry import Size

if TYPE_CHECKING:
    from ry.ryo3._size import FormatSizeBase, FormatSizeStyle

_FORMAT_SIZE_BASES: list[FormatSizeBase] = [2, 10]
_FORMAT_SIZE_STYLES: list[FormatSizeStyle] = [
    "default",
    "abbreviated",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
]


class _FmtKwargs(t.TypedDict, total=False):
    base: FormatSizeBase
    style: FormatSizeStyle


@dataclasses.dataclass(frozen=True)
class _FmtOptions:
    base: FormatSizeBase | None = None
    style: FormatSizeStyle | None = None

    def as_kwargs(self) -> _FmtKwargs:
        kw: _FmtKwargs = {}
        if self.base:
            kw["base"] = self.base
        if self.style:
            kw["style"] = self.style
        return kw

    @property
    def expected_style(self) -> FormatSizeStyle:
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


def test_fmt_parse_roundtrip(fmt_options: _FmtOptions) -> None:
    for size in _SIZES:
        formatted = ry.fmt_size(size, **fmt_options.as_kwargs())
        parsed = ry.parse_size(formatted)
        # parsed won't be EXACTLY the same as size, but it should be close
        # enough for the purposes of this test
        if formatted.lower().endswith(" bytes") or formatted.lower().endswith(" b"):
            assert parsed == size
        else:
            # make sure it is at most 1% off
            assert abs(parsed - size) / size < 0.01


def test_fmt_parse_formatter(fmt_options: _FmtOptions) -> None:
    formatter = ry.SizeFormatter(**fmt_options.as_kwargs())

    for size in _SIZES:
        formatted = formatter.format(size)
        formatted_via_call = formatter(size)
        assert formatted == formatted_via_call
        parsed = ry.parse_size(formatted)
        size_obj = Size(size)
        formatted_struct = size_obj.format(**fmt_options.as_kwargs())
        assert formatted == formatted_struct
        # parsed won't be EXACTLY the same as size, but it should be close
        # enough for the purposes of this test
        if formatted.lower().endswith(" bytes") or formatted.lower().endswith(" b"):
            assert parsed == size
        else:
            # make sure it is at most 1% off
            assert abs(parsed - size) / size < 0.01


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
        == f"SizeFormatter(base={_base_expected!r}, style={_style_expected!r})"
    )
    assert formatter == ry.SizeFormatter(base=_base_expected, style=_style_expected)

    assert formatter.base == _base_expected
    assert formatter.style == _style_expected
    assert formatter == eval(repr(formatter), {"SizeFormatter": ry.SizeFormatter})
