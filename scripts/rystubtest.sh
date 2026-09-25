#!/usr/bin/env bash
uv run maturin develop

# =============================================================================
echo "=== MYPY VERSION ==="
uv run --with mypy==1.19.1 python -m mypy.stubtest --version

echo "=== MYPY STUBTEST ==="
uv run --with mypy==1.19.1 python -m mypy.stubtest \
  --mypy-config-file pyproject.toml \
  --whitelist scripts/stubtest-allowlist.txt \
  --ignore-disjoint-bases \
  --concise \
  ry | rg -v "stub is a classmethod but runtime is not"

# =============================================================================
echo "=== MYPY VERSION (PATCHED) ==="
uv run --with mypy python -m mypy.stubtest --version

echo "=== MYPY STUBTEST ==="
uv run --with mypy python scripts/stubtest-patch-v2.3.1.py \
  --mypy-config-file pyproject.toml \
  --whitelist scripts/stubtest-allowlist.txt \
  --ignore-disjoint-bases \
  --concise \
  ry | rg -v "stub is a classmethod but runtime is not"
