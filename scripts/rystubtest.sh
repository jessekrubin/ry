#!/usr/bin/env bash

maturin develop
stubtest \
  --mypy-config-file pyproject.toml \
  --whitelist scripts/allowlist.txt \
  --ignore-disjoint-bases ry
# stubtest --mypy-config-file pyproject.toml ry
