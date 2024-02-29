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

cargo-fmt:
    cargo fmt

sort-all:
    sort-all python/ry/__init__.py

black:
    black python

fmt: cargo-fmt black

mypy:
    mypy python/ry tests/

pyright:
    pyright

ruff:
    ruff .

ruffix:
    ruff --fix --show-fixes

clippy:
    cargo clippy

lintpy: ruff mypy

lintrs: clippy

lint: lintpy lintrs


