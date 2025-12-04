from __future__ import annotations

import sys
import tomllib
from collections import defaultdict

from pydantic import BaseModel

import ry


class Ryo3Crate(BaseModel):
    name: str
    deps: list[str]


class CrateDepGraph(BaseModel):
    crates: list[Ryo3Crate]

    def mermaid_lines(self) -> list[str]:
        return [
            "flowchart LR\n",
            *(
                f"    {a} --> {b}\n"
                for a, b in sorted({
                    (crate.name, dep) for crate in self.crates for dep in crate.deps
                })
            ),
        ]

    def to_mermaid(self) -> str:
        return "".join(self.mermaid_lines())


def _build_graph() -> CrateDepGraph:
    cargo_files = ry.glob("crates/**/Cargo.toml", dtype=ry.FsPath).collect()

    crates: dict[str, ry.FsPath] = {}
    deps_map: dict[str, set[str]] = defaultdict(set)

    for toml_path in cargo_files:
        data = tomllib.loads(toml_path.read_text())
        pkg = data.get("package", {})
        name = pkg.get("name")
        if name:
            crates[name] = toml_path

    for name, toml_path in crates.items():
        data = tomllib.loads(toml_path.read_text())

        for section in ("dependencies", "build-dependencies"):
            section_deps = data.get(section, {})
            for dep_name, _info in section_deps.items():
                if dep_name in crates:
                    deps_map[name].add(dep_name)

    return CrateDepGraph(
        crates=[
            Ryo3Crate(
                name=name,
                deps=sorted(deps_map.get(name, [])),
            )
            for name in sorted(crates.keys())
        ]
    )


def main() -> None:
    graph = _build_graph()
    mmd = graph.to_mermaid()
    sys.stdout.write(mmd)


if __name__ == "__main__":
    main()
