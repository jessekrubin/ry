"""dev entry point"""

from __future__ import annotations

import ry  # noqa: F401
from ry import ryo3

# noinspection PyUnresolvedReferences
from ry.ryo3 import *  # noqa: F403

# noinspection PyUnresolvedReferences
from ry.ryo3._dev import *  # noqa: F403

__version__ = ryo3.__version__
__build_profile__ = ryo3.__build_profile__
__build_timestamp__ = ryo3.__build_timestamp__
if hasattr(ryo3, "__all__"):
    __all__ = ryo3.__all__ + ryo3._dev.__all__  # type: ignore[attr-defined]

# assign all things in ry03 to this module
for _k in dir(ryo3):
    if not _k.startswith("_"):
        globals()[_k] = getattr(ryo3, _k)

if __name__ == "__main__":
    import json

    from ry.__main__ import _lib_info

    def _banner() -> str:
        json_info = json.dumps(_lib_info(), indent=2)
        return "\n".join(
            (
                "~~~~~~~~~~~~~",
                "ry.dev ~ repl",
                "~~~~~~~~~~~~~",
                json_info,
            )
        )

    # locals
    local = globals()
    # try to do das IPython first and 4-most...!
    try:
        import IPython

        IPython.InteractiveShell.banner1 = _banner()  # type: ignore[attr-defined,assignment]
        IPython.start_ipython(argv=[], user_ns=local)  # type: ignore[no-untyped-call]

    except ImportError:
        import code

        code.interact(_banner(), local=local)
