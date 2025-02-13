from __future__ import annotations

from typing import Literal

import pytest

import ry

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
    base: None | Literal[2, 10],
    style: None
    | Literal[
        "default",
        "abbreviated",
        "abbreviated_lowercase",
        "abbreviated-lowercase",
        "full",
        "full-lowercase",
    ],
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
    base: None | Literal[2, 10],
    style: None
    | Literal[
        "default",
        "abbreviated",
        "abbreviated_lowercase",
        "abbreviated-lowercase",
        "full",
        "full-lowercase",
    ],
) -> None:
    formatter = ry.SizeFormatter(base=base, style=style)

    for size in SIZES:
        formatted = formatter.format(size)
        formatted_via_call = formatter(size)
        assert formatted == formatted_via_call
        parsed = ry.parse_size(formatted)
        # parsed won't be EXACTLY the same as size, but it should be close
        # enough for the purposes of this test
        if formatted.lower().endswith(" bytes") or formatted.lower().endswith(" b"):
            assert parsed == size
        else:
            # make sure it is at most 1% off
            assert abs(parsed - size) / size < 0.01
