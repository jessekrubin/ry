from __future__ import annotations

import subprocess as sp
from functools import lru_cache
from pathlib import Path

from ry import which

# this filesdir
PWDPATH = Path(__file__).resolve().parent
REPO_ROOT = PWDPATH.parent
API_PYI_FILEPATH = REPO_ROOT / "python" / "ry" / "ryo3.pyi"
README_FILEPATH = REPO_ROOT / "README.md"


@lru_cache
def get_api_content():
    api_content_raw = API_PYI_FILEPATH.read_text()

    ruff_path = which("ruff")
    assert ruff_path is not None, "ruff not found in PATH"
    # format the file... w/ 2 spaces so it fits in the README better
    # ruff format --config "indent-width = 2" -
    run_res = sp.run(
        [ruff_path, "format", "--config", "indent-width = 2", "-"],
        input=api_content_raw,
        text=True,
        capture_output=True,
        check=True,
    )
    api_content_formatted = run_res.stdout
    return api_content_formatted


def update_api_docs():
    """Update the API.md file in ./docs/src/API.md"""
    filepath = REPO_ROOT / "docs" / "src" / "API.md"
    assert filepath.exists(), f"API.md does not exist: {filepath}"
    api_content_formatted = get_api_content()
    with open(filepath, "w", newline="\n") as f:
        f.write(
            "\n".join(
                [
                    "# API",
                    "",
                    "```python",
                    api_content_formatted,
                    "```",
                ]
            )
        )


def update_readme():
    assert (
        API_PYI_FILEPATH.exists()
    ), f"API_PYI_FILEPATH does not exist: {API_PYI_FILEPATH}"
    assert (
        README_FILEPATH.exists()
    ), f"README_FILEPATH does not exist: {README_FILEPATH}"
    readme_content = README_FILEPATH.read_text()
    # API goes between `<!-- API_PYI_START -->` and `<!-- API_PYI_END -->`
    api_start = "<!-- API-START -->"
    api_end = "<!-- API-END -->"

    api_start_ix = readme_content.index(api_start)
    assert api_start_ix != -1, f"Could not find {api_start} in README.md"
    api_end_ix = readme_content.index(api_end)
    assert api_end_ix != -1, f"Could not find {api_end} in README.md"

    api_content_formatted = get_api_content()

    api_content_wrapped = f"\n```python\n{api_content_formatted}\n```\n"
    readme_chunks = (
        readme_content[: api_start_ix + len(api_start)],
        api_content_wrapped,
        readme_content[api_end_ix:],
    )
    updated_readme_content = "\n".join(readme_chunks)

    with open(README_FILEPATH, "w", newline="\n") as f:
        f.write(
            updated_readme_content,
        )


def main():
    update_api_docs()
    update_readme()


if __name__ == "__main__":
    main()
