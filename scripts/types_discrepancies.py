import dataclasses
import shutil
from pathlib import Path
from shutil import copy2, copytree

import griffe
from rich import print  # noqa

import ry.dev as ry

PWD = Path.cwd()
__dirname = Path(__file__).parent

TYPES_PATH = PWD / "python" / "ry" / "ryo3"


def load_types(strip_overload: bool = True) -> griffe.Object | griffe.Alias:
    # copy file to ryo3-types.pyi
    copytree(
        TYPES_PATH, __dirname / "ryo3types", dirs_exist_ok=True, copy_function=copy2
    )

    # for each file in ryo3types
    if strip_overload:
        for filepath in (__dirname / "ryo3types").rglob("*.pyi"):
            # replace `from ry import` with `from ry3types import`
            file_text = filepath.read_text()

            if "@t.overload" in file_text:
                # remove all lines with overload
                new_file_string = "\n".join(
                    filter(
                        lambda line: "@t.overload" not in line, file_text.split("\n")
                    )
                )
                with open(filepath, "w") as f:
                    f.write(new_file_string)

    # get the dummy types thingy
    types_package = griffe.load("ryo3types")
    return types_package


def load_ry() -> griffe.Object | griffe.Alias:
    ry_package = griffe.load("ry")
    return ry_package


OVERLOADS = {
    "Instant": {"__sub__"},
    "Date": {"__sub__"},
    "Time": {"__sub__"},
    "DateTime": {"__sub__"},
}
# "ry.api.Instant.__sub__",
# "ry.api.Date.checked_sub",
# "ry.api.Time.__sub__",
# "ry.api.DateTime.__sub__",
# "ry.api.ZonedDateTime.__isub__"


IGNORED_MEMBERS = {
    # MAYBE IGNORE
    "__eq__",
    "__ge__",
    "__gt__",
    "__hash__",
    "__le__",
    "__len__",
    "__lt__",
    "__ne__",
    # DEFO IGNORE
    "__add__",
    "__class__",
    "__setattr__",
    "__delattr__",
    "__dir__",
    "__doc__",
    "__format__",
    "__getattribute__",
    "__getstate__",
    "__init_subclass__",
    "__module__",
    "__new__",
    "__radd__",
    "__reduce__",
    "__reduce_ex__",
    "__repr__",
    "__richcmp__",
    "__rsub__",
    "__sizeof__",
    "__sub__",
    "__isub__",
    "__subclasshook__",
    "dbg",
}


@dataclasses.dataclass
class MembersComparison:
    member: str
    missing_from_types: tuple[str, ...]
    missing_from_actual: tuple[str, ...]


def compare_member(toget: str) -> MembersComparison:
    types_package = load_types()
    ry_actual_members = getattr(ry, toget)
    types_info = types_package.get_member(toget)

    actual_members = set(dir(ry_actual_members))
    types_members = set(types_info.members)
    # get missing in types, as well as missing in actual
    missing_from_types = (actual_members - types_members) - IGNORED_MEMBERS
    missing_from_actual = (types_members - actual_members) - IGNORED_MEMBERS
    return MembersComparison(
        member=toget,
        missing_from_types=tuple(sorted(missing_from_types)),
        missing_from_actual=tuple(sorted(missing_from_actual)),
    )


def ry_classes_n_types():
    return list(filter(lambda el: isinstance(getattr(ry, el), type), dir(ry)))


def main():
    class_members = [
        el for el in ry_classes_n_types() if el not in {"Headers", "HttpStatus"}
    ]
    for member in class_members:
        res = compare_member(member)
        print(res)
    shutil.rmtree(__dirname / "ryo3types")


if __name__ == "__main__":
    main()
