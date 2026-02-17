#!/usr/bin/env bash

# INSTALL STUFF WITH:
# ```
# rustup component add llvm-tools-preview
# cargo install cargo-llvm-cov
# ```

eval "$(cargo llvm-cov show-env --export-prefix)"
export CARGO_INCREMENTAL=1

rm -rfv ./target/llvm-cov/html

cargo llvm-cov clean --workspace
# cargo test
uv run -- maturin develop --uv
uv run -- pytest tests python/ry --doctest-modules --cov=ry --cov-report xml
cargo llvm-cov report \
  --package ryo3 \
  --package ryo3-aws-lc \
  --package ryo3-brotli \
  --package ryo3-bytes \
  --package ryo3-bzip2 \
  --package ryo3-core \
  --package ryo3-dirs \
  --package ryo3-flate2 \
  --package ryo3-fnv \
  --package ryo3-fspath \
  --package ryo3-glob \
  --package ryo3-globset \
  --package ryo3-heck \
  --package ryo3-http \
  --package ryo3-jiff \
  --package ryo3-jiter \
  --package ryo3-json \
  --package ryo3-macro-rules \
  --package ryo3-memchr \
  --package ryo3-pydantic \
  --package ryo3-regex \
  --package ryo3-reqwest \
  --package ryo3-same-file \
  --package ryo3-serde \
  --package ryo3-shlex \
  --package ryo3-size \
  --package ryo3-sqlformat \
  --package ryo3-std \
  --package ryo3-tokio \
  --package ryo3-twox-hash \
  --package ryo3-ulid \
  --package ryo3-unindent \
  --package ryo3-url \
  --package ryo3-uuid \
  --package ryo3-walkdir \
  --package ryo3-which \
  --package ryo3-zstd \
  --lcov \
  --output-path coverage.lcov
cargo llvm-cov report \
  --package ryo3 \
  --package ryo3-aws-lc \
  --package ryo3-brotli \
  --package ryo3-bytes \
  --package ryo3-bzip2 \
  --package ryo3-core \
  --package ryo3-dirs \
  --package ryo3-flate2 \
  --package ryo3-fnv \
  --package ryo3-fspath \
  --package ryo3-glob \
  --package ryo3-globset \
  --package ryo3-heck \
  --package ryo3-http \
  --package ryo3-jiff \
  --package ryo3-jiter \
  --package ryo3-json \
  --package ryo3-macro-rules \
  --package ryo3-memchr \
  --package ryo3-pydantic \
  --package ryo3-regex \
  --package ryo3-reqwest \
  --package ryo3-same-file \
  --package ryo3-serde \
  --package ryo3-shlex \
  --package ryo3-size \
  --package ryo3-sqlformat \
  --package ryo3-std \
  --package ryo3-tokio \
  --package ryo3-twox-hash \
  --package ryo3-ulid \
  --package ryo3-unindent \
  --package ryo3-url \
  --package ryo3-uuid \
  --package ryo3-walkdir \
  --package ryo3-which \
  --package ryo3-zstd \
  --html
serve ./target/llvm-cov/html
