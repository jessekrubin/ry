import pytest

import ry


def _ry_classes():
    return [k for k, v in vars(ry).items() if isinstance(v, type)]


def _ry_functions():
    return [k for k, v in vars(ry).items() if callable(v)]


# parametrize over all classes
@pytest.mark.parametrize("name", _ry_classes())
def test_class_module_dunder(name: str) -> None:
    assert getattr(ry, name).__module__ != "builtins"


@pytest.mark.parametrize("name", _ry_functions())
def test_function_module_dunder(name: str) -> None:
    assert getattr(ry, name).__module__ != "builtins"
