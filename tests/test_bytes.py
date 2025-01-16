import random

import ry.dev as ry

python_b = b"chongo was here!\n\x00"

print(f"python str: {python_b}\n")

ry_b = ry.Bytes(python_b)

print(f"ry bytes: {ry_b}\n")


def random_byte_int() -> int:
    return random.randint(0, 255)


def make_random_bytes(length: int = 32) -> bytes:
    return bytes(random_byte_int() for _ in range(length))


random_bytes = make_random_bytes()


def expected_repr(b: bytes) -> str:
    s = repr(b)
    # strip the leading `b'` and trailing `'` and replace with `Bytes(b"` and `")`
    # return f'Bytes(b"{s[2:-1]}")'
    return f"Bytes(b'{s[2:-1]}')"


import pytest


@pytest.mark.parametrize(
    "b",
    [
        #     all bytes
        bytes([i])
        for i in range(256)
    ],
)
def test_random_bytes_repr(b: bytes) -> None:
    ry_bytes = ry.Bytes(b)
    ry_bytes_str = str(ry_bytes)
    ry_bytes_str_eval = eval("ry." + ry_bytes_str)
    assert ry_bytes_str_eval == ry_bytes


def strip_ry_bytes_string(s: str) -> str:
    return s[6:-1]


@pytest.mark.parametrize("b", [make_random_bytes(32) for i in range(1000)])
def test_random_bytes_repr2(b: bytes) -> None:
    ry_bytes = ry.Bytes(b)
    ry_bytes_str = str(ry_bytes)
    # check that eval of bytes works...
    ry_bytes_str_eval = eval("ry." + ry_bytes_str)
    # check that it is equal to the original bytes
    assert ry_bytes == ry_bytes_str_eval, "ry_bytes != ry_bytes_str_eval"


@pytest.mark.parametrize("b", [bytes([i]) for i in range(256)])
def test_hex_str(b: bytes) -> None:
    print(b)
    ry_bytes = ry.Bytes(b)
    ry_hex_str = ry_bytes.hex()
    py_hex_str = b.hex()
    assert ry_hex_str == py_hex_str


# for i in range(100):
#     print('-------------')
#     random_bytes = make_random_bytes()
#     _test_random_bytes_repr(random_bytes)
#


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
        "decode",
        "endswith",
        "expandtabs",
        "find",
        "fromhex",
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
        "startswith",
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
    method = getattr(rust_bytes, fn_name)
    with pytest.raises(NotImplementedError):
        if fn_name in ["__mod__", "__rmod__"]:
            method(1)  # provide an argument
        else:
            method()
