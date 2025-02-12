from __future__ import annotations

import pytest

import ry


@pytest.mark.parametrize(
    "fn_name",
    [
        "__bytes__",
        "__getnewargs__",
        "__iter__",
        "__mod__",
        "__rmod__",
        "capitalize",
        "center",
        "count",
        "endswith",
        "expandtabs",
        "find",
        "index",
        "istitle",
        "join",
        "ljust",
        "lstrip",
        "maketrans",
        "partition",
        "replace",
        "rfind",
        "rindex",
        "rjust",
        "rpartition",
        "rsplit",
        "rstrip",
        "split",
        "splitlines",
        "strip",
        "swapcase",
        "title",
        "translate",
        "zfill",
    ],
)
def test_bytes_not_impl(fn_name: str) -> None:
    b = b"asdf"
    rust_bytes = ry.Bytes(b)
    method = getattr(rust_bytes, fn_name, None)
    if method is None:
        return
    with pytest.raises(NotImplementedError):
        if fn_name in ["__mod__", "__rmod__"]:
            method(1)  # provide an argument
        else:
            method()
