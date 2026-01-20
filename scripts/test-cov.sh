#!/usr/bin/env bash

# INSTALL STUFF WITH:
# ```
# rustup component add llvm-tools-preview
# cargo install cargo-llvm-cov
# ```

eval "$(cargo llvm-cov show-env --export-prefix)"
export CARGO_INCREMENTAL=1

rm -rfv ./target/llvm-cov/html

cargo llvm-cov clean --workspace --profraw-only
# cargo test
uv run -- maturin develop --uv
uv run -- pytest tests python/ry --doctest-modules --cov=ry --cov-report xml
cargo llvm-cov report --lcov --output-path coverage.lcov
cargo llvm-cov report --html
serve ./target/llvm-cov/html
