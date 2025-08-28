from __future__ import annotations

import argparse
import dataclasses
import itertools as it
import subprocess as sp
from functools import lru_cache
from typing import TYPE_CHECKING

import ry
from ry import FsPath, which

if TYPE_CHECKING:
    from collections.abc import Iterable
    from os import PathLike

# this filesdir
PWDPATH = FsPath(__file__).resolve().parent
REPO_ROOT = PWDPATH.parent
PYTHON_ROOT = REPO_ROOT / "python"
RYO_PYI_DIRPATH = REPO_ROOT / "python" / "ry"
API_PYI_FILEPATH = REPO_ROOT / "python" / "ry" / "ryo3.pyi"
README_FILEPATH = REPO_ROOT / "README.md"


@dataclasses.dataclass
class RyTypeStubInfo:
    def __init__(self, name: str, pyi_filepath: FsPath) -> None:
        self.name = name
        self.pyi_filepath = pyi_filepath

    def __repr__(self) -> str:
        return f"RyTypeStubInfo(name={self.name!r}, pyi_filepath={self.pyi_filepath!r})"


def filepath2module(filepath: FsPath) -> str:
    return (
        filepath.with_suffix("").strip_prefix(PYTHON_ROOT).as_posix().replace("/", ".")
    )


@lru_cache
def get_types_dictionary() -> dict[str, str]:
    types_dict = {}
    files = sorted(
        (
            ry.FsPath(p)
            for p in ry.walkdir(
                RYO_PYI_DIRPATH, glob="**/*.pyi", files=True, dirs=False
            )
        ),
        key=lambda p: filepath2module(p).lower(),  # case-stable across OSes
    )
    for pyi_filepath in files:
        module_name = filepath2module(pyi_filepath)
        types_dict[module_name] = pyi_filepath.read_text()
    return types_dict


@lru_cache
def ruff_format_pyi(string: str, line_length: int = 80, indent_width: int = 4) -> str:
    ruff_path = which("ruff")
    assert ruff_path is not None, "ruff not found in PATH"
    # format the file... w/ 2 spaces so it fits in the README better
    # ruff format --config "indent-width = 2" -
    # line length 80 for the API docs
    run_res = sp.run(
        [
            ruff_path,
            "format",
            "--line-length",
            str(line_length),
            "--config",
            f"indent-width = {indent_width}",
            "-",
        ],
        input=string,
        text=True,
        capture_output=True,
        check=True,
    )
    return run_res.stdout.rstrip("\n")


def _gen_api_content_readme(
    line_length: int = 80,
    indent_width: int = 4,
) -> Iterable[str]:
    dictionary = get_types_dictionary()
    # format all of them...
    dictionary_formatted = {
        k: ruff_format_pyi(v, line_length, indent_width) for k, v in dictionary.items()
    }
    # first one is '__init__.pyi'
    # then the rest are just sorted...
    parts = []
    root_pyi = dictionary_formatted.pop("ry.ryo3.__init__")

    root_level = sorted(
        e for e in dictionary_formatted.keys() if e.startswith("ry.ryo3.")
    )
    submodules = sorted(
        e for e in dictionary_formatted.keys() if not e.startswith("ry.ryo3.")
    )
    sorted_dictionary = {
        **{k: v for k, v in dictionary_formatted.items() if k in root_level},
        **{k: v for k, v in dictionary_formatted.items() if k in submodules},
    }
    parts.append(("ry.ryo3.__init__", root_pyi))
    parts.extend((mod_name, content) for mod_name, content in sorted_dictionary.items())
    # make the toc
    yield "# API"
    yield ""
    yield "## Table of Contents"

    yield from (f"- [`{mod_name}`](#{mod_name})" for mod_name, _ in parts)

    headers = (
        f'<h2 id="{mod_name}"><code>{mod_name}</code></h2>\n' for mod_name, _ in parts
    )
    type_stub_bodies = (f"```python\n{content}\n```\n" for _, content in parts)
    # zipper the headers and bodies into a single chain

    yield from it.chain.from_iterable(zip(headers, type_stub_bodies, strict=False))


@lru_cache
def get_api_content_readme(
    line_length: int = 80,
    indent_width: int = 4,
) -> str:
    return "\n".join(_gen_api_content_readme(line_length, indent_width))


def write_text(
    p: str | PathLike[str],
    text: str,
    *,
    check: bool = False,
) -> bool:
    """Write text to a file, creating the file if it doesn't exist."""
    filepath = FsPath(p)
    # check if the file is up to date
    current_text = filepath.read_text()
    if current_text != text:
        print(f"File is up to date: {filepath}")
        if check:
            msg = f"File is not up to date: {filepath}\n"
            raise ValueError(msg)
        else:
            print(f"Writing to file: {filepath}")
            filepath.write_text(text)
            return True
    else:
        print(f"File is up to date: {filepath}")
        return False


def update_api_docs(
    *,
    check: bool = False,
) -> None:
    """Update the API.md file in ./docs/src/api.md"""
    filepath = REPO_ROOT / "docs" / "src" / "api.md"
    assert filepath.exists(), f"api.md does not exist: {filepath}"
    api_content_formatted = get_api_content_readme()
    parts = [api_content_formatted]
    write_text(filepath, "\n".join(parts), check=check)


def update_docs_examples(*, check: bool = False) -> None:
    examples_root = REPO_ROOT / "examples"
    assert examples_root.exists(), f"examples_root does not exist: {examples_root}"
    files = sorted(
        ry.walkdir(examples_root, glob="**/*.py", files=True, dirs=False).collect()
    )
    assert files, f"No files found in {examples_root}"

    def _build_part(filepath: FsPath) -> str:
        # read the file
        content = filepath.read_text()
        # format it
        formatted_content = ruff_format_pyi(content, line_length=80, indent_width=4)
        return formatted_content

    # format the files
    toc = []

    parts = []
    for file in files:
        p = ry.FsPath(file)
        # read the file
        content = p.read_text()
        # format it

        # add the toc entry
        toc.append(f"- [{p.stem}](#{p.stem})")

        formatted_content = ruff_format_pyi(content, line_length=80, indent_width=4)
        parts.append(f"# {p.stem}\n\n```python\n{formatted_content}\n```\n")

    # write the file
    filepath = REPO_ROOT / "docs" / "src" / "examples.md"
    assert filepath.exists(), f"examples.md does not exist: {filepath}"
    write_text(filepath, "\n".join(parts), check=check)


def update_docs(*, check: bool = False) -> None:
    update_api_docs(check=check)
    update_docs_examples(check=check)


def update_readme() -> None:
    assert RYO_PYI_DIRPATH.exists(), (
        f"RYO_PYI_DIRPATH does not exist: {RYO_PYI_DIRPATH}"
    )
    assert README_FILEPATH.exists(), (
        f"README_FILEPATH does not exist: {README_FILEPATH}"
    )
    readme_content = README_FILEPATH.read_text()
    # API goes between `<!-- API_PYI_START -->` and `<!-- API_PYI_END -->`
    api_start = "<!-- API-START -->"
    api_end = "<!-- API-END -->"

    api_start_ix = readme_content.index(api_start)
    assert api_start_ix != -1, f"Could not find {api_start} in README.md"
    api_end_ix = readme_content.index(api_end)
    assert api_end_ix != -1, f"Could not find {api_end} in README.md"

    api_content_formatted = get_api_content_readme()
    readme_chunks = (
        readme_content[: api_start_ix + len(api_start)],
        api_content_formatted,
        readme_content[api_end_ix:],
    )
    updated_readme_content = "\n".join(readme_chunks)

    with open(README_FILEPATH, "w", newline="\n") as f:
        f.write(
            updated_readme_content,
        )


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Update the API docs and examples in the README.md file."
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Check if the API docs are up to date.",
    )

    parsed = parser.parse_args()
    update_docs(
        check=parsed.check,
    )


if __name__ == "__main__":
    main()
