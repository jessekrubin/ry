import sys

import pytest

import ry

_CONSTANTS = [
    ("U8_BITS", 8),
    ("U8_MAX", 255),
    ("U8_MIN", 0),
    ("I8_BITS", 8),
    ("I8_MAX", 127),
    ("I8_MIN", -128),
    ("I16_BITS", 16),
    ("I16_MAX", 32_767),
    ("I16_MIN", -32_768),
    ("U16_BITS", 16),
    ("U16_MAX", 65_535),
    ("U16_MIN", 0),
    ("U32_BITS", 32),
    ("U32_MAX", 4_294_967_295),
    ("U32_MIN", 0),
    ("I32_BITS", 32),
    ("I32_MAX", 2_147_483_647),
    ("I32_MIN", -2_147_483_648),
    ("U64_BITS", 64),
    ("U64_MAX", 18_446_744_073_709_551_615),
    ("U64_MIN", 0),
    ("I64_BITS", 64),
    ("I64_MAX", 9_223_372_036_854_775_807),
    ("I64_MIN", -9_223_372_036_854_775_808),
    ("U128_BITS", 128),
    ("U128_MAX", 340_282_366_920_938_463_463_374_607_431_768_211_455),
    ("U128_MIN", 0),
    ("I128_BITS", 128),
    ("I128_MAX", 170_141_183_460_469_231_731_687_303_715_884_105_727),
    ("I128_MIN", -170_141_183_460_469_231_731_687_303_715_884_105_728),
]


@pytest.mark.parametrize("name,value", _CONSTANTS)
def test_constants(name: str, value: int) -> None:
    assert getattr(ry, name) == value
    assert isinstance(getattr(ry, name), int)


@pytest.mark.parametrize(
    "name,value",
    [
        ("USIZE_BITS", (32, 64)[sys.maxsize > 2**32]),
        ("USIZE_MAX", (4_294_967_295, 18_446_744_073_709_551_615)[sys.maxsize > 2**32]),
        ("USIZE_MIN", 0),
        ("ISIZE_BITS", (32, 64)[sys.maxsize > 2**32]),
        ("ISIZE_MAX", (2_147_483_647, 9_223_372_036_854_775_807)[sys.maxsize > 2**32]),
        (
            "ISIZE_MIN",
            (-2_147_483_648, -9_223_372_036_854_775_808)[sys.maxsize > 2**32],
        ),
    ],
)
def test_constants_isize_usize(name: str, value: int) -> None:
    assert getattr(ry, name) == value
    assert isinstance(getattr(ry, name), int)
