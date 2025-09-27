# DEVELOPMENT

## goals

1. Provide a really nice ergonomic API to work with (this is the highest
   priority)
2. Get naming right (this is a hard one!)
3. Be fast

## development-setup

- clone repo
- install `just` (`cargo install just`)
- create a virtual env (using ye olde `venv` or `uv` or dare I say `conda`) -- I
  am still working out the kinks of using `uv` with maturin
- install the dev-requirements (`pip install -r requirements.dev.txt`)
- run `just dev` to build and test the library
- run `just fmt` to format the python and rust code

## style guide

- Naming conventions:
  - For python classes/structs/types prefer `AsyncXYZ` over `XYZAsync`
  - use `snake_case` for functions and variables
  - use `CamelCase` for types and traits
  - use `SCREAMING_SNAKE_CASE` for constants and statics
- **NO UNWRAPPING** -- use `expect` over `unwrap`
- **NO PANICS** -- don't panic!
- **NO `blazingly-fast`** -- `ry` is fast and does not need an adverb
- **USE CLIPPY** `just clippy` or `just ci`
- **USE RUSTFMT AND RUFF** `just fmt`
- library style guide:
  - python objects/structs/classes defined in the library should be named either
    `Py<CLASSNAME>` or `Ry<CLASSNAME>` and the prefix should be consistent
    throughout the library (eg `ryo3-jiff` uses `Ry` as the internal prefix to
    not conflict with the `Py<CLASSNAME>` structs provided by `pyo3`)
  - For wrapper libraries, attempt to mirror the structure of the original
    library as much as possible
  - wrapper library names should be of the form `ryo3-<LIB_NAME>` where
    `<LIB_NAME>` is the name of the wrapped library
  - library directories should be `kebab-case` and should be `ryo3-<LIB_NAME>`

## Creating a new library/wrapper-thing

- copy the template library `ryo3-quick-maths` library to your new library name
- refer to the above style guide for naming conventions

---

## tools

### python

- we use `maturin` for building the python wheels
- we support `python-3.9+`
- we use `pytest` for testing as well as the following plugins:
  - `pytest-benchmark`
  - `pytest-asyncio` (may switch to `anyio` in the future)
  - `hypothesis`

### just

**`cargo install just`**

- we use `just` for task running
- to see all tasks run `just` or `just --list` (our default task echos the list
  of tasks)

tasks as of 2025-09-26:

```txt
Available recipes:
    repl            # run ry.dev python repl
    repl-uv         # run ry.dev python repl (if using uv)
    dev             # dev run build + tests
    dev-uv          # dev run build + tests (with uv)
    sync            # uv sync
    develop         # maturin develop
    develop-uv      # maturin develop (with uv)
    mat             # maturin develop (shorthand)
    cargo-test      # cargo test
    build           # build
    build-release   # build release
    dev-rel         # maturin develop release
    doctest         # run pytest
    pytest          # run pytest
    pytest-uv       # run pytest
    pytestv         # run pytest (printing captured output)
    test            # run all test
    test-release    # test ry package
    bench           # benchmark ry python package
    ci              # ci rust checks
    cargo-fmt       # cargo format
    cargo-fmtc      # cargo format check
    sort-all-check  # ruff check sorting of '__all__'
    sort-all        # ruff sort '__all__'
    ruff-fmt        # ruff format
    ruff-fmtc       # ruff format check
    fmtpy           # python format
    fmtcpy          # python format check
    justfilefmt     # justfile format
    justfilefmtc    # justfile format check
    mdfmt           # format markdown
    pyprojectfmt    # pyproject-fmt
    fmt             # format
    fmtc            # format check
    ruff            # run ruff linter
    ruffix          # run ruff + fix
    clippy          # run clippy
    clippy-features # run clippy with feature-powerset via cargo-hack
    lint            # lint python and rust
    mypy            # run mypy type checker
    pyright         # run pyright
    pip-compile     # pip compile requirements
    gen             # generate code tasks
    cargo-doc       # generate cargo docs for all crates (in workspace)
```
