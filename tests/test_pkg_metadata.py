from __future__ import annotations

import tomllib
from pathlib import Path

import ry


def _repo_root() -> Path:
    _pwd = Path(__file__).parent
    for _i in range(5):
        if (_pwd / ".git").exists():
            return _pwd
        _pwd = _pwd.parent
    msg = "Could not find repo root"
    raise RuntimeError(msg)


_REPO_ROOT = _repo_root()


def _version_from_workspace_package(repo_root: Path) -> str:
    root_cargo_toml_filepath = repo_root / "Cargo.toml"
    s = root_cargo_toml_filepath.read_text()
    version = tomllib.loads(s)["workspace"]["package"]["version"]
    if not isinstance(version, str):
        msg = f"Cargo version is not a string: {version}"
        raise RuntimeError(msg)
    return version


def test_version(repo_root: Path) -> None:
    assert ry.__version__ is not None

    cargo_version = _version_from_workspace_package(repo_root)
    assert ry.__version__ == cargo_version


def test_check_build_profile() -> None:
    assert ry.__build_profile__ is not None
    assert ry.__build_profile__ in ("debug", "release"), (
        f"ry.__build_profile__ is not 'debug'/'release': {ry.__build_profile__}"
    )


def test_package_description_and_pyproject_match() -> None:
    from ry.__about__ import __description__

    pyproject_toml_path = _REPO_ROOT / "pyproject.toml"
    s = pyproject_toml_path.read_text()
    pyproject_data = tomllib.loads(s)

    pyproject_description = pyproject_data["project"]["description"]
    assert __description__ == pyproject_description, (
        f"ry.__about__.__description__ ({__description__!r}) != "
        f"pyproject.toml description ({pyproject_description!r})"
    )
