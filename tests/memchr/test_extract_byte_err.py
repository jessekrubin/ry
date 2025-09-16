from typing import Any

import pytest

import ry


@pytest.mark.parametrize(
    "v",
    [-1, 256, b"too long", {"not": "a byte"}],
)
def test_extract_byte_err(v: Any) -> None:
    with pytest.raises(TypeError):
        ry.memchr(v, b"the quick brown fox")
