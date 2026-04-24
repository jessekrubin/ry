#!/usr/bin/env bash

uv run maturin develop

uv run python -m mypy.stubtest \
  --mypy-config-file pyproject.toml \
  --whitelist scripts/stubtest-allowlist.txt \
  --ignore-disjoint-bases \
  --concise \
  ry
