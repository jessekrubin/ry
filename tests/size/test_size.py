from __future__ import annotations

import pickle
from typing import TYPE_CHECKING

import pytest

import ry
from ry import Size

if TYPE_CHECKING:
    from ry.ryo3._size import FormatSizeBase, FormatSizeStyle

FORMAT_SIZE_BASES = [None, 2, 10]
FORMAT_SIZE_STYLES = [
    None,
    "default",
    "abbreviated",
    "abbreviated_lowercase",
    "abbreviated-lowercase",
    "full",
    "full-lowercase",
]

POSITIVE_SIZES = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
]
SIZES = [
    0,
    (2**63) - 1,  # max i64
    (2**63) * -1,  # min i64
    *POSITIVE_SIZES,
    *(-s for s in POSITIVE_SIZES),
]


@pytest.mark.parametrize("base", FORMAT_SIZE_BASES)
@pytest.mark.parametrize("style", FORMAT_SIZE_STYLES)
def test_fmt_parse_roundtrip(
    base: FormatSizeBase,
    style: FormatSizeStyle,
) -> None:
    for size in SIZES:
        formatted = ry.fmt_size(size, base=base, style=style)
        parsed = ry.parse_size(formatted)
        # parsed won't be EXACTLY the same as size, but it should be close
        # enough for the purposes of this test
        if formatted.lower().endswith(" bytes") or formatted.lower().endswith(" b"):
            assert parsed == size
        else:
            # make sure it is at most 1% off
            assert abs(parsed - size) / size < 0.01


@pytest.mark.parametrize("base", FORMAT_SIZE_BASES)
@pytest.mark.parametrize("style", FORMAT_SIZE_STYLES)
def test_fmt_parse_formatter(
    base: FormatSizeBase,
    style: FormatSizeStyle,
) -> None:
    formatter = ry.SizeFormatter(base=base, style=style)

    for size in SIZES:
        formatted = formatter.format(size)
        formatted_via_call = formatter(size)
        assert formatted == formatted_via_call
        parsed = ry.parse_size(formatted)
        size_obj = Size(size)
        formatted_struct = size_obj.format(base=base, style=style)
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

        with pytest.raises(ValueError):
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


@pytest.mark.parametrize("base", FORMAT_SIZE_BASES)
@pytest.mark.parametrize("style", FORMAT_SIZE_STYLES)
def test_size_formatter_pickling(
    base: FormatSizeBase,
    style: FormatSizeStyle,
) -> None:
    formatter = ry.SizeFormatter(base=base, style=style)
    unpickled = pickle.loads(pickle.dumps(formatter))
    assert formatter == unpickled
