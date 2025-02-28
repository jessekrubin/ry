from __future__ import annotations

import dataclasses
import subprocess as sp
from functools import lru_cache

from ry import FsPath, which

# this filesdir
PWDPATH = FsPath(__file__).resolve().parent
REPO_ROOT = PWDPATH.parent
PYTHON_ROOT = REPO_ROOT / "python"
RYO_PYI_DIRPATH = REPO_ROOT / "python" / "ry" / "ryo3"
API_PYI_FILEPATH = REPO_ROOT / "python" / "ry" / "ryo3.pyi"
README_FILEPATH = REPO_ROOT / "README.md"


@dataclasses.dataclass
class RyTypeStubInfo:
    def __init__(self, name: str, pyi_filepath: FsPath):
        self.name = name
        self.pyi_filepath = pyi_filepath

    def __repr__(self):
        return f"RyTypeStubInfo(name={self.name!r}, pyi_filepath={self.pyi_filepath!r})"


def filepath2module(filepath: FsPath) -> str:
    return (
        filepath.with_suffix("").strip_prefix(PYTHON_ROOT).as_posix().replace("/", ".")
    )


@lru_cache
def get_types_dictionary():
    types_dict = {}
    for pyi_filepath in RYO_PYI_DIRPATH.iterdir():
        if pyi_filepath.is_file() and pyi_filepath.suffix == ".pyi":
            module_name = filepath2module(pyi_filepath)

            types_dict[
                module_name
                # pyi_filepath.file_name()
            ] = pyi_filepath.read_text()
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
    return run_res.stdout


@lru_cache
def get_api_content_readme(
    line_length: int = 80,
    indent_width: int = 4,
):
    dictionary = get_types_dictionary()
    # format all of them...
    dictionary_formatted = {
        k: ruff_format_pyi(v, line_length, indent_width) for k, v in dictionary.items()
    }
    # first one is '__init__.pyi'
    # then the rest are just sorted...
    parts = []
    root_pyi = dictionary_formatted.pop("ry.ryo3.__init__")
    parts.append(("ry.ryo3", root_pyi))
    parts.extend(sorted(dictionary_formatted.items()))
    api_content_parts = []
    for mod_name, content in parts:
        api_content_parts.append(f"## `{mod_name}`\n")
        api_content_parts.append(f"```python\n{content}\n```")
    api_content_formatted = "\n".join(api_content_parts)
    return api_content_formatted


def update_api_docs():
    """Update the API.md file in ./docs/src/API.md"""
    filepath = REPO_ROOT / "docs" / "src" / "API.md"
    assert filepath.exists(), f"API.md does not exist: {filepath}"
    api_content_formatted = get_api_content_readme()
    with open(filepath, "w", newline="\n") as f:
        f.write(
            "\n".join(
                [
                    "# API",
                    "",
                    api_content_formatted,
                ]
            )
        )


def update_readme():
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


def main():
    # dictionary = get_types_dictionary()
    # print(dictionary)
    # print(list(dictionary.keys()))
    update_readme()
    update_api_docs()


if __name__ == "__main__":
    main()
