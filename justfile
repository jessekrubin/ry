#!/usr/bin/env just --justfile
# 'justfile'
# just-repo: https://github.com/casey/just
# just-docs: https://just.systems/man/en/

@_default:
    just --list --unsorted

# run ry.dev python repl
repl:
    python -m ry.dev

# run ry.dev python repl (if using uv)
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
    maturin develop --features "mimalloc"

# maturin develop (with uv)
develop-uv:
    uv run maturin develop --features "mimalloc" --uv

# maturin develop (shorthand)
mat *ARGS:
    uv run maturin develop --uv {{ ARGS }}

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
    uv run pytest --benchmark-skip --doctest-modules --doctest-glob="*.pyi" python

# run pytest
pytest +ARGS='python tests':
    uv run pytest --benchmark-skip --doctest-modules --doctest-glob="*.pyi" {{ ARGS }}

# run pytest
pytest-uv:
    uv run pytest --benchmark-skip

# run pytest (printing captured output)
pytestv:
    uv run pytest --benchmark-skip -rP

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
    cargo clippy --all-targets --features mimalloc -- -D warnings
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
    cargo hack --feature-powerset clippy --package ryo3-tokio
    cargo hack --feature-powerset clippy --package ryo3-twox-hash
    cargo hack --feature-powerset clippy --package ryo3-ulid
    cargo hack --feature-powerset clippy --package ryo3-url
    cargo hack --feature-powerset clippy --package ryo3-uuid
    cargo hack --feature-powerset clippy --package ryo3-which
    cargo hack --feature-powerset clippy --package ryo3-serde
    cargo hack --feature-powerset clippy --exclude-no-default-features -F ring --package ryo3-reqwest

# run cargo check with feature-powerset via cargo-hack
check-features:
    cargo hack --feature-powerset check --package ryo3-bytes
    cargo hack --feature-powerset check --package ryo3-fspath
    cargo hack --feature-powerset check --package ryo3-http
    cargo hack --feature-powerset check --package ryo3-jiff
    cargo hack --feature-powerset check --package ryo3-std
    cargo hack --feature-powerset check --package ryo3-tokio
    cargo hack --feature-powerset check --package ryo3-twox-hash
    cargo hack --feature-powerset check --package ryo3-ulid
    cargo hack --feature-powerset check --package ryo3-url
    cargo hack --feature-powerset check --package ryo3-uuid
    cargo hack --feature-powerset check --package ryo3-which
    cargo hack --feature-powerset check --package ryo3-serde
    cargo hack --feature-powerset check --exclude-no-default-features -F ring --package ryo3-reqwest

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

# generate depgraph for docs
depgraph-svg:
    python scripts/dep-graph-mmd.py | mmdc -i - -o docs/src/assets/dep-graph.svg -t dark -b transparent

# =====================================================================
# CLEAN ~ CLEAN ~ CLEAN ~ CLEAN ~ CLEAN ~ CLEAN ~ CLEAN ~ CLEAN ~ CLEAN
# =====================================================================

# clean out local caches/artifacts/stuff
clean:
    cargo clean
    rm -rfv .mypy_cache .venv .pytest_cache
    find . -name "__pycache__" | xargs -n1 -t rm -frv
    find . -name '*.py[co]' | xargs -n1 -t rm -fv
    find . -name '*~' | xargs -n1 -t rm -fv
    find . -name '.*~' | xargs -n1 -t rm -fv
