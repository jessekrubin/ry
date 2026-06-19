from types import ModuleType

import pytest

import ry
import ry.dev as rydev


def test_dev_exports_all_from_root(subtests: pytest.Subtests) -> None:
    non_module_members = {
        el: getattr(ry, el)
        for el, mem in vars(ry).items()
        if not isinstance(mem, ModuleType) and not el.startswith("_")
    }

    for thing in non_module_members:
        with subtests.test(thing):
            assert hasattr(rydev, thing)
