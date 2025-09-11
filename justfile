#!/usr/bin/env just --justfile
# 'justfile'
# just-repo: https://github.com/casey/just
# just-docs: https://just.systems/man/en/

@_default:
    just --list --unsorted

repl:
    python -m ry.dev

repl-uv:
    uv run python -m ry.dev

# dev run build + tests
dev: develop test

# dev run build + tests (with uv)
dev-uv: develop-uv pytest-uv

# uv sync
sync:
    uv sync --inexact

# maturin develop
develop:
    maturin develop

# maturin develop (with uv)
develop-uv:
    uv run maturin develop --uv

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
doctest:
    pytest --benchmark-skip --doctest-modules --doctest-glob="*.pyi" python

pytest:
    pytest --benchmark-skip

# run pytest
pytest-uv:
    uv run pytest --benchmark-skip

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
    uv run ruff check . --select RUF022 --preview --output-format=full

# ruff sort '__all__'
sort-all:
    uv run ruff check . --select RUF022 --preview --output-format=full --fix

# ruff format
ruff-fmt:
    uv run ruff format .
    uv run ruff check --select "I" --show-fixes --fix .

# ruff format check
ruff-fmtc:
    uv run ruff format . --check

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

# format markdown
mdfmt:
    pnpm dlx prettier@latest --cache --prose-wrap=always -w CHANGELOG.md

# pyproject-fmt
pyprojectfmt:
    uvx pyproject-fmt . --keep-full-version

# format
fmt: cargo-fmt fmtpy justfilefmt mdfmt pyprojectfmt

# format check
fmtc: cargo-fmtc fmtcpy justfilefmtc

# ==========================================================================
# LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT ~ LINT
# ==========================================================================

# run ruff linter
ruff:
    uv run ruff check .

# run ruff + fix
ruffix:
    uv run ruff --fix --show-fixes

# run clippy
clippy:
    cargo clippy

# run clippy with feature-powerset via cargo-hack
clippy-features:
    cargo hack --feature-powerset clippy --package ryo3-bytes
    cargo hack --feature-powerset clippy --package ryo3-fspath
    cargo hack --feature-powerset clippy --package ryo3-http
    cargo hack --feature-powerset clippy --package ryo3-jiff
    cargo hack --feature-powerset clippy --package ryo3-std
    cargo hack --feature-powerset clippy --package ryo3-twox-hash
    cargo hack --feature-powerset clippy --package ryo3-ulid
    cargo hack --feature-powerset clippy --package ryo3-url
    cargo hack --feature-powerset clippy --package ryo3-uuid
    cargo hack --feature-powerset clippy --package ryo3-which

# lint python and rust
lint: ruff clippy

# =====================================================================
# TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK ~ TYPECHECK
# =====================================================================

# run mypy type checker
mypy:
    uv run mypy python/ry tests/ examples/ scripts/

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
