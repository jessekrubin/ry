from ry import _ry


def main():
    ry_all = _ry.__all__

    print(_ry.__description__)
    print(_ry.__pkg_name__)

    def sort_all(strings: list[str]):
        dunders = {x for x in strings if x.startswith("__") and x.endswith("__")}
        faux_private = {x for x in strings if x.startswith("_") and x not in dunders}
        non_dunders = {x for x in strings if x not in dunders and x not in faux_private}

        return [*sorted(dunders), *sorted(non_dunders), *sorted(faux_private)]

    all_tuple_sorted = tuple(sort_all(ry_all))

    # import lines for __init__.py
    import_lines = [
        "from ry import _ry",
        "from ry._ry import (",
        *(f"    {x}," for x in all_tuple_sorted),
        ")",
    ]
    # '__all__' lines for __init__.py
    package_all_list_lines = [
        "__all__ = (",
        *(f'    "{x}",' for x in (*all_tuple_sorted, "_ry")),
        ")",
    ]

    # __init__.py string lines
    lines = [f'"""{_ry.__doc__}\n"""', "", *import_lines, "", *package_all_list_lines]

    # __init__.py string
    init_string = "\n".join(lines)

    # test it
    try:
        exec(init_string)
    except Exception as e:
        print(e)
        raise e from None
    print(init_string)


if __name__ == "__main__":
    main()
