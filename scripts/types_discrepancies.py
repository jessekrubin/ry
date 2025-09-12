from __future__ import annotations

import dataclasses
import shutil
from functools import lru_cache
from pathlib import Path

import griffe
from rich import print  # noqa

import ry.dev as ry

PWD = Path.cwd()
__dirname = Path(__file__).parent

TYPES_PATH = PWD / "python" / "ry"


@lru_cache(maxsize=1)
def build_faux_types_pkg(*, strip_overload: bool = True) -> None:
    if (__dirname / "ryo3types").exists():
        shutil.rmtree(__dirname / "ryo3types")

    shutil.copytree(
        TYPES_PATH,
        __dirname / "ryo3types",
        dirs_exist_ok=True,
        copy_function=shutil.copy2,
    )

    # delete all non pyi files in ryo3types
    for f in ry.glob(str(__dirname / "ryo3types" / "**/*"), dtype=ry.FsPath):
        if (
            f.is_file()
            and not str(f).endswith(".pyi")
            and not str(f).endswith("_types.py")
        ):
            ry.remove_file(f)

    root_level_pyi_files = ry.glob(str(__dirname / "ryo3types" / "*.pyi")).collect()

    base_init_pyi_lines = [
        "from .ryo3  import *",
        *(f"from .{filepath.stem} import *" for filepath in root_level_pyi_files),
    ]
    # write the __init__.pyi file
    with open(str(__dirname / "ryo3types" / "__init__.pyi"), "w") as init_file:
        init_file.write("\n".join(base_init_pyi_lines))

    # for each file in ryo3types
    if strip_overload:
        for filepath in (__dirname / "ryo3types").rglob("*.pyi"):
            # replace `from ry import` with `from ry3types import`
            file_text = filepath.read_text()

            new_file_string = file_text
            if "@t.overload" in file_text:
                # remove all lines with overload
                new_file_string = "\n".join(
                    filter(
                        lambda line: "@t.overload" not in line, file_text.split("\n")
                    )
                )

            if "from ry." in file_text:
                new_file_string = new_file_string.replace("from ry.", "from ryo3types.")
            if "from ry " in file_text:
                new_file_string = new_file_string.replace("from ry ", "from ryo3types ")
            if new_file_string != file_text:
                with open(filepath, "w") as fobj:
                    fobj.write(new_file_string)


def load_types(*, strip_overload: bool = True) -> griffe.Object | griffe.Alias:
    build_faux_types_pkg(strip_overload=strip_overload)

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
    # TODO
    "__get_pydantic_core_schema__",
    "_pydantic_validate",
    "try_from",  # have not settled on this one yet...
    # MAYBE IGNORE
    "__eq__",
    "__ge__",
    "__gt__",
    "__hash__",
    "__le__",
    "__rtruediv__",
    "__len__",
    "__lt__",
    "__ne__",
    # DEFO IGNORE
    "__add__",
    "__str__",
    "__class__",
    "__setattr__",
    "__getitem__",
    "__release_buffer__",
    "__rmul__",
    "__delattr__",
    "__dir__",
    "__doc__",
    "__format__",
    "__getattribute__",
    "__getstate__",
    "__getnewargs__",
    "__getnewargs_ex__",
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


def get_members(obj: griffe.Object | griffe.Alias) -> set[str]:
    """Get all members of a griffe object."""
    return {
        *obj.members,
        *obj.inherited_members,
    }


def compare_member(toget: str) -> MembersComparison:
    types_package = load_types()
    ry_actual_members = getattr(ry, toget)
    types_info = types_package.get_member(toget)

    actual_members = set(dir(ry_actual_members))

    types_members = get_members(types_info)
    # get missing in types, as well as missing in actual
    missing_from_types = (actual_members - types_members) - IGNORED_MEMBERS
    missing_from_actual = (types_members - actual_members) - IGNORED_MEMBERS
    return MembersComparison(
        member=toget,
        missing_from_types=tuple(sorted(missing_from_types)),
        missing_from_actual=tuple(sorted(missing_from_actual)),
    )


def ry_classes_n_types() -> list[str]:
    return list(filter(lambda el: isinstance(getattr(ry, el), type), dir(ry)))


def main() -> None:
    classes2ignore = {"ReqwestError"}
    class_members = [el for el in ry_classes_n_types() if el not in classes2ignore]

    all_good = []
    problems = []

    for member in class_members:
        res = compare_member(member)
        if not res.missing_from_actual and not res.missing_from_types:
            all_good.append(res)
        else:
            problems.append(res)
            print(res)
    shutil.rmtree(__dirname / "ryo3types")


if __name__ == "__main__":
    main()
