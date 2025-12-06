import pathlib

import ry

_UNCANNONICAL_PATH = "tests/std/../../tests/std/test_cannonicalize.py"

_EXPECTED_PATH_TRAILER = pathlib.Path("tests/std/test_cannonicalize.py")


def test_canonicalize_str() -> None:
    cannon = ry.canonicalize(_UNCANNONICAL_PATH)
    assert cannon.endswith(str(_EXPECTED_PATH_TRAILER))


def test_canonicalize_path() -> None:
    cannon = ry.canonicalize(pathlib.Path(_UNCANNONICAL_PATH))
    assert str(cannon).endswith(str(_EXPECTED_PATH_TRAILER))
