from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path

TEMPLATE = """# `ryo3-CRATEID`

ryo3-wrapper for `CRATEID` crate

[//]: # (<GENERATED>)

## Ref:

- docs.rs: [https://docs.rs/CRATEID](https://docs.rs/CRATEID)
- crates: [https://crates.io/crates/CRATEID](https://crates.io/crates/CRATEID)

[//]: # (</GENERATED>)
"""


def test_ry_crates_have_readme_file_individual(ry_repo_crate: Path) -> None:
    readme_filepath = ry_repo_crate / "README.md"
    assert readme_filepath.exists()


def test_ry_crate_readme_is_lf_line_endings(ry_repo_crate: Path) -> None:
    readme_filepath = ry_repo_crate / "README.md"
    assert readme_filepath.exists()
    text = readme_filepath.read_text(encoding="utf-8")
    assert "\r\n" not in text
    assert "\r" not in text
    assert "\n" in text
