"""dev entry point"""

from __future__ import annotations

import ry
from ry import ryo3  # noqa: F401
from ry.__main__ import _lib_info

# noinspection PyUnresolvedReferences
from ry.dev import *  # noqa: F403


def _banner() -> str:
    json_info = ry.stringify(_lib_info(), fmt=True).decode("utf-8")
    return "\n".join((
        "~~~~~~~~~~~~~",
        "ry.dev ~ repl",
        "~~~~~~~~~~~~~",
        json_info,
    ))


def _main() -> None:
    import datetime as pydt

    try:
        import rich
        from rich import inspect
        from rich import print as pprint
    except ImportError:
        from pprint import pprint  # type: ignore[assignment]

        rich = inspect = None  # type: ignore[assignment]

    # locals
    local = globals()
    local.update({
        "pydt": pydt,
        "inspect": inspect,
        "pprint": pprint,
        "rich": rich,
    })
    # try to do das IPython first and 4-most...!
    try:
        import sys

        import IPython

        IPython.InteractiveShell.banner1 = _banner()  # type: ignore[attr-defined,assignment]
        rich = None  # type: ignore[assignment]
        ipython_argv = [
            "--no-tip",
            "--TerminalInteractiveShell.editing_mode=vi",
            "--TerminalInteractiveShell.emacs_bindings_in_vi_insert_mode=False",
            *sys.argv[1:],
        ]
        if rich is not None:
            ipython_argv.extend(["--ext", "rich"])
        IPython.start_ipython(argv=ipython_argv, user_ns=local)  # type: ignore[no-untyped-call]
        return
    except ImportError:
        ...

    import code

    code.interact(_banner(), local=local)


if __name__ == "__main__":
    _main()
