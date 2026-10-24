"""stubtest monkey-patched to handle stub-only re-exports correctly

PROBLEM-PR: https://github.com/python/mypy/pull/20417
"""

from __future__ import annotations

import functools
import importlib.util
import sys
import typing as t

from mypy import stubtest


@functools.cache
def _runtime_module_exists(name: str) -> bool:
    try:
        return importlib.util.find_spec(name) is not None
    except Exception:
        return False


class _RuntimeBackedStubs(dict[str, t.Any]):
    def __contains__(self, key: object) -> bool:
        return (
            super().__contains__(key)
            and isinstance(key, str)
            and _runtime_module_exists(key)
        )


_orig_build_stubs = stubtest.build_stubs

_NO_NATIVE_PARSER = "--no-native-parser" in sys.argv
if _NO_NATIVE_PARSER:
    sys.argv.remove("--no-native-parser")


@functools.wraps(_orig_build_stubs)
def _build_stubs(
    modules: list[str], options: t.Any, *args: t.Any, **kwargs: t.Any
) -> list[str]:
    if _NO_NATIVE_PARSER:
        options.native_parser = False
    res = _orig_build_stubs(modules, options, *args, **kwargs)
    stubtest._all_stubs = _RuntimeBackedStubs(stubtest._all_stubs)
    return res


stubtest.build_stubs = _build_stubs  # ty: ignore[invalid-assignment]

if __name__ == "__main__":
    sys.exit(stubtest.main())
