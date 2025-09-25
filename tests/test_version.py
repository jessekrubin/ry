from __future__ import annotations

import tomllib
from pathlib import Path

import ry

PWD = Path(__file__).parent
PYPROJECT_TOML = PWD.parent.parent / "pyproject.toml"


def _repo_root() -> Path:
    _pwd = Path(__file__).parent
    for _i in range(5):
        if (_pwd / ".git").exists():
            return _pwd
        _pwd = _pwd.parent
    msg = "Could not find repo root"
    raise RuntimeError(msg)


_REPO_ROOT = _repo_root()


def _version_from_workspace_package() -> str:
    root_cargo_toml_filepath = _REPO_ROOT / "Cargo.toml"
    s = root_cargo_toml_filepath.read_text()
    version = tomllib.loads(s)["workspace"]["package"]["version"]
    if not isinstance(version, str):
        msg = f"Cargo version is not a string: {version}"
        raise RuntimeError(msg)
    return version


def test_version() -> None:
    assert ry.__version__ is not None

    cargo_version = _version_from_workspace_package()
    assert ry.__version__ == cargo_version
