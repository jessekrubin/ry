#!/usr/bin/env just --justfile
# 'justfile'
# just-repo: https://github.com/casey/just
# just-docs: https://just.systems/man/en/

@_default:
    just --list --unsorted

# dev run build + tests
dev: develop test

# maturin develop
develop:
    maturin develop

# maturin develop (shorthand)
mat:
    maturin develop

# cargo test
cargo-test:
    cargo test

# build
build: cargo-test
    maturin build

# build release
build-release:
    maturin build --release

# maturin develop release
dev-rel:
    maturin develop --release

# run pytest
pytest:
    pytest --benchmark-skip

# run pytest (printing captured output)
pytestv:
    pytest --benchmark-skip -rP

# run all test
test: pytest

# test ry package
test-release: build-release
    pytest

# benchmark ry python package
bench: build-release
    pytest -vv

# ci rust checks
ci:
    cargo fmt -- --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

# ===========================================================================
# FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT ~ FMT
# ===========================================================================

# cargo format
cargo-fmt:
    cargo fmt --all

# cargo format check
cargo-fmtc:
    cargo fmt --all -- --check

# ruff check sorting of '__all__'
sort-all-check:
    ruff check . --select RUF022 --preview --output-format=full

# ruff sort '__all__'
sort-all:
    ruff check . --select RUF022 --preview --output-format=full --fix

# ruff format
ruff-fmt:
    ruff format .
    ruff check --select "I" --show-fixes --fix .

# ruff format check
ruff-fmtc:
    ruff format . --check

# python format black
black:
    black python

# python format
fmtpy: sort-all ruff-fmt

# python format check
fmtcpy: sort-all-check ruff-fmtc

# justfile format
justfilefmt:
    just --fmt --unstable

# justfile format check
justfilefmtc:
    just --check --fmt --unstable

# format
fmt: cargo-fmt fmtpy justfilefmt

# format check
fmtc: cargo-fmtc fmtcpy justfilefmtc

# ==========================================================================
# LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT
# ==========================================================================

# run ruff linter
ruff:
    ruff check .

# run ruff + fix
ruffix:
    ruff --fix --show-fixes

# run clippy
clippy:
    cargo clippy

# lint python and rust
lint: ruff clippy

# =====================================================================
# TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK
# =====================================================================

# run mypy type checker
mypy:
    mypy python/ry tests/ examples/

# run pyright
pyright:
    pyright

# =====================================================================
# PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON ~ PYTHON
# =====================================================================

# pip compile requirements
pip-compile:
    uv pip compile requirements.dev.in -n > requirements.dev.txt

_gen_init:
    python scripts/gen.py > python/ry/__init__.py

_gen-py: _gen_init fmtpy

# generate code tasks
gen: _gen-py

# =====================================================================
# docs
# =====================================================================

# generate cargo docs for all crates (in workspace)
cargo-doc:
    cargo doc --no-deps --workspace
