from __future__ import annotations

import ry.dev as ry


def test_anystr_string() -> None:
    s = "abc"
    assert ry.anystr_noop(s) == s
    assert isinstance(ry.anystr_noop(s), str)


def test_anystr_bytes() -> None:
    b = b"abc"
    assert ry.anystr_noop(b) == b
    assert isinstance(ry.anystr_noop(b), bytes)
