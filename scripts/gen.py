from __future__ import annotations

import sys

from ry import dev as ryo3


def eprint(*args, **kwargs):
    print(*args, **kwargs, file=sys.stderr)


def main():
    ry_all = ryo3.__all__

    eprint(ryo3.__description__)
    eprint(ryo3.__pkg_name__)

    def sort_all(strings: list[str]):
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
    lines = [f'"""{ryo3.__doc__}\n"""', "", *import_lines, "", *package_all_list_lines]

    # __init__.py string
    init_string = "\n".join(lines)

    # test it
    try:
        exec(init_string)  # noqa: S102
    except Exception as e:
        eprint(e)
        raise e from None
    sys.stdout.buffer.write(init_string.encode("utf-8"))


if __name__ == "__main__":
    main()
