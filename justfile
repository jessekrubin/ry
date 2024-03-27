dev: develop test

develop:
    maturin develop

cargo-test:
    cargo test

build: cargo-test
    maturin build

build-release:
    maturin build --release

dev-rel:
    maturin develop --release

test:
    pytest --benchmark-skip

test-release: build-release
    pytest

bench: build-release
    pytest -vv

# ===========================================================================
# FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT
# ===========================================================================

# cargo format
cargo-fmt:
    cargo fmt --all

# cargo format check
cargo-fmtc:
    cargo fmt --all -- --check

sort-all-check:
    ruff check . --select RUF022 --preview --output-format=full

sort-all:
    ruff check . --select RUF022 --preview --output-format=full --fix

# ruff format
ruff-fmt:
    ruff format .

# ruff format check
ruff-fmtc:
    ruff format . --check

# python format black
black:
    black python

# python format
pyfmt: sort-all ruff-fmt

# pythong format check
pyfmtc: sort-all-check ruff-fmtc

# justfile format
justfilefmt:
    just --fmt --unstable

# justfile format check
justfilefmtc:
    just --check --fmt --unstable

# format
fmt: cargo-fmt pyfmt justfilefmt

# format check
fmtc: cargo-fmtc pyfmtc justfilefmtc

# ==========================================================================
# LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT

# ==========================================================================
ruff:
    ruff .

ruffix:
    ruff --fix --show-fixes

clippy:
    cargo clippy

lintpy: ruff mypy

lintrs: clippy

lint: lintpy lintrs

# =====================================================================
# TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK

# =====================================================================
mypy:
    mypy python/ry tests/

pyright:
    pyright

# =====================================================================
# PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON

# =====================================================================
pip-compile:
    uv pip compile requirements.dev.in -n > requirements.dev.txt
