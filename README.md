# ry

A growing collection of Python shims around Rust crates; fast, async-first, and
ergonomic.

[![PyPI](https://img.shields.io/pypi/v/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Wheel](https://img.shields.io/pypi/wheel/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Status](https://img.shields.io/pypi/status/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - License](https://img.shields.io/pypi/l/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)

**DOCS:** [ryo3.dev](https://ryo3.dev) (WIP)

**API:** [ryo3.dev/api](https://ryo3.dev/api)

**This is a work in progress ~ feedback and PRs are welcome.**

## Highlights

- **Async-first HTTP client:** Built on `reqwest`, with a `fetch`-like API.
  Supports streaming, zero-copy IO via the buffer protocol, timeouts,
  redirect-following, and native JSON parsing via `jiter`.
- **Async file I/O:** Built on `tokio`, with an `AsyncFile` API similar to
  `aiofiles` and `anyio`'s async-file api. Supports buffered reads/writes,
  truncation, streaming reads, and `anyio` compatibility.
- **(de)compression:** (de)compression tools for `zstd`, `brotli`, `gzip`, and
  `bzip2`.
- **Datetime utilities via `jiff`:** Fast, accurate, timezone-aware datetime
  parsing and formatting, with `datetime` interop and much more
- **Miscellaneous bindings:** Includes crates like `globset`, `walkdir`,
  `sqlformat`, `unindent`, `twox-hash`, and more.
- **Designed for ergonomics:** Async where it matters. Simple where possible.
  Python-native behavior with minimal friction.
- **Type Annotated:** All public APIs are (painstakingly) type annotated.
- **Performant:** Speed without the words "blazingly fast." [^1]

## Install

```bash
pip install ry
uv add ry

# check install
python -m ry
```

## Quickstart

Check out the [examples](https://github.com/jessekrubin/ry/tree/main/examples)
directory for some quickstart examples.

---

## What?

- `ry` -- the python package
- `ryo3-*` -- the rust crates that are used by `ry` and possibly your own
  `pyo3`-based python package

## Who?

- jessekrubin <jessekrubin@gmail.com>
- possibly you!?

## FAQ

_(aka: questions that I have been asking myself)_

- **Q:** Why?
  - **A:** I (jesse) needed several hashing functions for python and then kept
    adding things as I needed them
- **Q:** Does this have anything to do with the (excellent) package manager
  `rye`?
  - **A:** short answer: no. long answer: no, it does not.
- **Q:** Why is the repo split into `ry` and `ryo3`?
  - **A:** `ry` is the python package, `ryo3` is a rust crate setup to let you
    "register" functions you may want if you were writing your own pyo3-python
    bindings library; maybe someday the `ryo3::libs` module will be split up
    into separate packages

## Crate bindings

| **crate**       | **ryo3-crate**                                                                        |
| --------------- | ------------------------------------------------------------------------------------- |
| `std`           | [`ryo3-std`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-std)             |
| `bytes`         | [`ryo3-bytes`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-bytes)         |
| `bzip2`         | [`ryo3-bzip2`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-bzip2)         |
| `dirs`          | [`ryo3-dirs`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-dirs)           |
| `glob`          | [`ryo3-glob`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-glob)           |
| `heck`          | [`ryo3-heck`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-heck)           |
| `http`          | [`ryo3-http`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-http)           |
| `jiter`         | [`ryo3-jiter`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-jiter)         |
| `reqwest`       | [`ryo3-reqwest`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-reqwest)     |
| `serde`         | [`ryo3-serde`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-serde)         |
| `shlex`         | [`ryo3-shlex`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-shlex)         |
| `size`          | [`ryo3-size`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-size)           |
| `sqlformat`     | [`ryo3-sqlformat`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-sqlformat) |
| `tokio`         | [`ryo3-tokio`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-tokio)         |
| `ulid`          | [`ryo3-ulid`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-ulid)           |
| `unindent`      | [`ryo3-unindent`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-unindent)   |
| `url`           | [`ryo3-url`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-url)             |
| `uuid`          | [`ryo3-uuid`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-uuid)           |
| `which`         | [`ryo3-which`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-which)         |
| **Compression** | **~**                                                                                 |
| `brotli`        | [`ryo3-brotli`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-brotli)       |
| `flate2`        | [`ryo3-flate2`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-flate2)       |
| `zstd`          | [`ryo3-zstd`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-zstd)           |
| **Hashing**     | **~**                                                                                 |
| `fnv`           | [`ryo3-fnv`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-fnv)             |
| `twox-hash`     | [`ryo3-twox-hash`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-twox-hash) |
| **@BurntSushi** | **~**                                                                                 |
| `globset`       | [`ryo3-globset`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-globset)     |
| `jiff`          | [`ryo3-jiff`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-jiff)           |
| `memchr`        | [`ryo3-memchr`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-memchr)       |
| `regex`         | [`ryo3-regex`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-regex)         |
| `same-file`     | [`ryo3-same-file`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-same-file) |
| `walkdir`       | [`ryo3-walkdir`](https://github.com/jessekrubin/ry/tree/main/crates/ryo3-walkdir)     |

---

## DEV

- `just` is used to run tasks
- Do not use the phrase `blazing fast` or any emojis in any PRs or issues or
  docs
- type annotations are required
- `ruff` used for formatting and linting

---

## SEE ALSO

- [utiles](https://github.com/jessekrubin/utiles): web-map tile utils

[^1]:
    Release‑version benchmarks of `ry` (via `pytest-benchmark`) showed no real
    performance variance, regardless of whether "blazingly fast" appeared in the
    README or docs.
