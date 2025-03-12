"""ryo3-which types"""

from __future__ import annotations

from pathlib import Path

from ry.ryo3._regex import Regex

def which(cmd: str, path: None | str = None) -> Path | None: ...
def which_all(cmd: str, path: None | str = None) -> list[Path]: ...
def which_re(regex: str | Regex, path: None | str = None) -> list[Path]: ...
