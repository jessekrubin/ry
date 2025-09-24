"""ry pytest conftest"""

from __future__ import annotations

from functools import lru_cache
from pathlib import Path

import pytest


@lru_cache(maxsize=1)
def _repo_root() -> Path:
    root = Path(__file__).parent.parent.resolve()
    assert root.is_dir()
    assert (root / ".git").is_dir()
    return root


@pytest.fixture
def repo_root() -> Path:
    return _repo_root()


@pytest.fixture
def ry_repo_crates(ry_repo_root: Path) -> list[Path]:
    return list(ry_repo_root.glob("crates/*"))


@pytest.fixture
def ry_pyproject_toml_path(ry_repo_root: Path) -> Path:
    return ry_repo_root / "pyproject.toml"


@pytest.fixture(
    params=[
        pytest.param(crate, id=crate.name)
        for crate in (
            cargo_toml.parent for cargo_toml in _repo_root().glob("crates/*/Cargo.toml")
        )
    ]
)
def ry_repo_crate(request: pytest.FixtureRequest) -> Path:
    """Returns the `pathlib.Path` each `ryo3-*` crate directory"""
    assert isinstance(request.param, Path)
    return request.param
