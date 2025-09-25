from __future__ import annotations

import dataclasses
import os
import subprocess
import sys
from pathlib import Path

import pytest

import ry

PWD = Path(__file__).parent


def _repo_root() -> Path:
    _pwd = Path(__file__).parent
    for _i in range(5):
        if (_pwd / ".git").exists():
            return _pwd
        _pwd = _pwd.parent
    msg = "Could not find repo root"
    raise RuntimeError(msg)


REPO_ROOT = _repo_root()
EXAMPLES_ROOT = REPO_ROOT / "examples"


@dataclasses.dataclass
class ExampleScript:
    filepath: ry.FsPath


def examples_scripts() -> list[ExampleScript]:
    e = []
    for f in ry.walkdir(
        EXAMPLES_ROOT,
    ):
        if f.endswith(".py"):
            e.append(ExampleScript(filepath=ry.FsPath(f)))
    return e


@pytest.mark.parametrize("example", examples_scripts())
def test_example_script(example: ExampleScript, tmp_path: Path) -> None:
    if "CI" in os.environ and os.environ["CI"] == "true":
        pytest.skip("SKIP DURING CI (for now)")
    os.chdir(tmp_path)
    assert example.filepath.exists()
    assert example.filepath.is_file()
    assert os.path.exists(example.filepath)
    assert os.path.isfile(example.filepath)
    res = subprocess.run([sys.executable, str(example.filepath)], capture_output=True)

    assert res.returncode == 0, (
        f"Example script {example.filepath} failed with return code {res.returncode}\n"
        f"__STDOUT__:\n{res.stdout.decode()}^^^\n"
        f"__STDERR__:\n{res.stderr.decode()}^^^\n"
    )
