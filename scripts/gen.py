from __future__ import annotations

import sys
from typing import Any

from ry import ryo3


def eprint(*args: Any, **kwargs: Any) -> None:
    print(*args, **kwargs, file=sys.stderr)


def main() -> None:
    ry_all = ryo3.__all__  # type: ignore[attr-defined]

    eprint(ryo3.__description__)
    eprint(ryo3.__pkg_name__)

    def sort_all(strings: list[str]) -> list[str]:
        dunders = {x for x in strings if x.startswith("__") and x.endswith("__")}
        faux_private = {x for x in strings if x.startswith("_") and x not in dunders}
        non_dunders = {x for x in strings if x not in dunders and x not in faux_private}

        return [*sorted(dunders), *sorted(non_dunders), *sorted(faux_private)]

    all_tuple_sorted = tuple(sort_all(ry_all))

    # import lines for __init__.py
    import_lines = [
        "from ry import ryo3",
        "from ry.ryo3 import (",
        *(f"    {x}," for x in all_tuple_sorted),
        ")",
    ]
    # '__all__' lines for __init__.py
    package_all_list_lines = [
        "__all__ = (",
        *(f'    "{x}",' for x in (*all_tuple_sorted, "ryo3")),
        ")",
    ]

    # __init__.py string lines
    lines = [
        f'"""{ryo3.__doc__}\n"""',
        "",
        *import_lines,
        "",
        *package_all_list_lines,
    ]

    # __init__.py string
    init_string = "\n".join(lines)

    # test it
    try:
        exec(init_string)  # noqa: S102
        sys.stdout.buffer.write(init_string.encode("utf-8"))
    except Exception as e:
        print(init_string)
        eprint(e)
        raise e from None


if __name__ == "__main__":
    main()
