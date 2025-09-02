import string

import pytest

from ry import shlex

_SAFEUNQUOTED = string.ascii_letters + string.digits + "@%_-+=:,./"
_UNICODE_SAMPLE = "\xe9\xe0\xdf"  # e + acute accent, a + grave, sharp s
_UNSAFE = '"`$\\!' + _UNICODE_SAMPLE
_TEST_CASES = [
    ("", "''"),
    (_SAFEUNQUOTED, "'" + _SAFEUNQUOTED + "'"),
    ("test file name", "'test file name'"),
    *((f"test{u}name", f'"test{u}name"') for u in _UNSAFE),
    *((f"test{u}'name'", f"'test{u}'\"'\"'name'\"'\"''") for u in _UNSAFE),
]


@pytest.mark.parametrize("string, expected", _TEST_CASES)
def test_quote(string: str, expected: str) -> None:
    assert shlex.quote(string, allow_nul=True) == expected
