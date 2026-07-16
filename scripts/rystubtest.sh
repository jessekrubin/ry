#!/usr/bin/env bash

uv run maturin develop

uv run --with mypy==1.19.1 python -m mypy.stubtest --version
uv run --with mypy==1.19.1 python -m mypy.stubtest \
  --mypy-config-file pyproject.toml \
  --whitelist scripts/stubtest-allowlist.txt \
  --ignore-disjoint-bases \
  --concise \
  ry | rg -v "stub is a classmethod but runtime is not"
