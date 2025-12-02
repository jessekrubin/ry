#!/usr/bin/env bash

maturin develop
stubtest \
  --mypy-config-file pyproject.toml \
  --whitelist scripts/allowlist.txt \
  --ignore-disjoint-bases \
  --concise \
  ry
# stubtest --mypy-config-file pyproject.toml ry
