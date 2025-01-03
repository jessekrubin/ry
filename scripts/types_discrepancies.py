import dataclasses
import shutil
from pathlib import Path
from shutil import copy2, copytree

import griffe
from rich import print  # noqa

import ry

PWD = Path.cwd()
__dirname = Path(__file__).parent

TYPES_PATH = PWD / "python" / "ry" / "ryo3"


def load_types() -> griffe.Object | griffe.Alias:
    # copy file to ryo3-types.pyi
    copytree(
        TYPES_PATH, __dirname / "ryo3types", dirs_exist_ok=True, copy_function=copy2
    )
    # get the dummy types thingy
    types_package = griffe.load("ryo3types")
    return types_package


def load_ry() -> griffe.Object | griffe.Alias:
    ry_package = griffe.load("ry")
    return ry_package


IGNORED_MEMBERS = {
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
    # get all classes in ry
    members = [
        # url
        "URL",
        # globset
        "Glob",
        "GlobSet",
        "Globster",
        # xxhash
        # "Xxh3",
        # "Xxh32",
        # "Xxh64",
        # path
        "FsPath",
        # std
        "Duration",
        "Instant",
        # jiff
        "SignedDuration",
        "DateTime",
        "TimeSpan",
        "Date",
        "Time",
        "ZonedDateTime",
        "Offset",
        "TimeZone",
    ]
    for member in members:
        res = compare_member(member)
        print(res)

    shutil.rmtree(__dirname / "ryo3types")


if __name__ == "__main__":
    main()
