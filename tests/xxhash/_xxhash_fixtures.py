from __future__ import annotations

import ast
from pathlib import Path
from typing import TypedDict

import ry

_PWD = Path(__file__).parent.absolute()
_TEST_DATA = _PWD / "xxhash.ndjson"

XX32_SEEDS = [0, 1, 2**32 - 1]
XX64_SEEDS = [0, 1, 2**64 - 1]
XX128_SEEDS = [0, 1, 2**64 - 1]  # same as 64-bit seeds


class XXHashDataRecord(TypedDict):
    buf: str
    xxh32_0x00000000: str
    xxh32_0x00000001: str
    xxh32_0xFFFFFFFF: str
    xxh64_0x00000000: str
    xxh64_0x00000001: str
    xxh64_0xFFFFFFFFFFFFFFFF: str
    xxh3_64_0x00000000: str
    xxh3_64_0x00000001: str
    xxh3_64_0xFFFFFFFFFFFFFFFF: str
    xxh3_128_0x00000000: str
    xxh3_128_0x00000001: str
    xxh3_128_0xFFFFFFFFFFFFFFFF: str


def _load_data() -> list[XXHashDataRecord]:
    with open(_TEST_DATA) as f:
        xx32_test_data = f.read()
    lines = xx32_test_data.split("\n")
    return [
        XXHashDataRecord(**row)  # type: ignore[typeddict-item]
        for row in (
            ry.parse_json(line) for line in lines if line.strip() if line.strip()
        )
        if row
    ]


XXHASH_TEST_DATA = _load_data()


def _bytes_from_record(rec: XXHashDataRecord) -> bytes:
    """Eval the bytes from the rec"""
    b = ast.literal_eval(rec["buf"])
    assert isinstance(b, bytes)
    return b
