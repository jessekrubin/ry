#!/usr/bin/env bash

# INSTALL STUFF WITH:
# ```
# rustup component add llvm-tools-preview
# cargo install cargo-llvm-cov
# ```

source <(cargo llvm-cov show-env --export-prefix)

export CARGO_TARGET_DIR=$CARGO_LLVM_COV_TARGET_DIR
export CARGO_INCREMENTAL=1

rm -rfv ./target/llvm-cov/html

cargo llvm-cov clean --workspace
# cargo test
uv run -- maturin develop --uv
uv run -- pytest tests python/ry --doctest-modules --cov=ry --cov-report xml
cargo llvm-cov report --lcov --output-path coverage.lcov
cargo llvm-cov report --html
serve ./target/llvm-cov/html
