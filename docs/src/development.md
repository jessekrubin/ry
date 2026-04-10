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
  - use `SCREAMING_SNAKE_CASE` for constants and static-class-attributes
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
- pyo3-signature-formatting; put spaces around `=`:
  - ok: `#[pyo3(signature = (data = None, *, mode = Foo::BAR, _check = false))]`
  - bad: `#[pyo3(signature=(data=None, *, mode=Foo::BAR, _check=false))]`

## Creating a new library/wrapper-thing

- copy the template library `ryo3-quick-maths` library to your new library name
- refer to the above style guide for naming conventions

---

## tools

### python

- we use `maturin` for building the python wheels
- we support `python-3.11+`
- we use `pytest` for testing as well as the following plugins:
  - `anyio` (which I am increasingly thinking is actually a bit of a turd)
  - `hypothesis`
  - `pytest-benchmark`
  - `pytest-cov`

### just

**`cargo install just`**

- we use `just` for task running
- to see all tasks run `just` or `just --list` (our default task echos the list
  of tasks)

just-recipes `just -l` (ca. 2026-04-10):

```txt
Available recipes:
    bench                       # benchmark ry python package
    build                       # build
    build-release               # build release
    cargo-doc                   # generate cargo docs for all crates (in workspace)
    cargo-fmt                   # cargo format
    cargo-fmtc                  # cargo format check
    cargo-test                  # cargo test
    check-features              # run cargo check with feature-powerset via cargo-hack
    ci                          # ci rust checks
    clean                       # clean out local caches/artifacts/stuff
    clippy                      # run clippy
    clippy-features             # run clippy with feature-powerset via cargo-hack
    depgraph-svg                # generate depgraph for docs
    dev                         # dev run build + tests
    develop                     # maturin develop
    devrel                      # maturin develop release
    doctest                     # run pytest
    fmt                         # format
    fmtc                        # format check
    fmtcpy                      # python format check
    fmtpy                       # python format
    gen                         # generate code tasks
    justfilefmt                 # justfile format
    justfilefmtc                # justfile format check
    lint                        # lint python and rust
    mat *ARGS                   # maturin develop (shorthand)
    mdfmt                       # format markdown
    mypy                        # run mypy type checker
    pip-compile                 # pip compile requirements
    pyprojectfmt                # pyproject-fmt
    pyright                     # run pyright
    pytest +ARGS='python tests' # run pytest
    pytestv                     # run pytest (printing captured output)
    repl                        # run ry.dev python repl
    ruff                        # run ruff linter
    ruff-fmt                    # ruff format
    ruff-fmtc                   # ruff format check
    ruffix                      # run ruff + fix
    sort-all                    # ruff sort '__all__'
    sort-all-check              # ruff check sorting of '__all__'
    sync                        # uv sync
    test                        # run all test
    test-release                # test ry package
```
