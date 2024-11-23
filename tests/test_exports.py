from __future__ import annotations

import pytest

import ry


def test_has_build_profile() -> None:
    assert hasattr(ry, "__build_profile__")


def test_has_version_lib() -> None:
    assert hasattr(ry, "__version__")


def test_doc_is_not_none() -> None:
    assert hasattr(ry, "__doc__")


_IGNORED_NAMES = (
    # ry ignores
    "_ry",
    "ryo3",
    "dev",
    # misc ignores
    "annotations",
    "__builtins__",
    "__loader__",
    "__spec__",
)


@pytest.mark.parametrize("name", dir(ry))
def test_exports_module_attr_param(name: str) -> None:
    if name in _IGNORED_NAMES or name.startswith("_frozen") or name == "":
        return
    member = getattr(ry, name)
    if isinstance(member, (str, int, float, list, tuple, dict)):
        return

    module_name = member.__module__
    assert module_name is not None, f"{name} has no __module__"
    assert module_name != "builtins", f"{name} is builtin"
    assert module_name in ("ryo3", "ry.ryo3"), f"{name} {member} is not in ryo3"
