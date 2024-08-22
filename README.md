# ry

[![PyPI](https://img.shields.io/pypi/v/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Wheel](https://img.shields.io/pypi/wheel/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - Status](https://img.shields.io/pypi/status/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)
[![PyPI - License](https://img.shields.io/pypi/l/ry?style=flat-square&cacheSeconds=600)](https://pypi.org/project/ry/)

python bindings for rust crates I wish existed in python

**THIS IS A WORK IN PROGRESS**

## Install

```bash
pip install ry
poetry add ry
pdm add ry
rye add ry
```

**Check install:** `python -m ry`

## What and why?

This is a collection of pyo3-wrappers for rust crates I wish existed in python.

It all started with me wanting a fast python `xxhash` and `fnv-64`

## Who?

- jessekrubin <jessekrubin@gmail.com>
- possibly you!?

## FAQ

_(aka: questions that I have been asking myself)_

- **Q:** Why?
  - **A:** I (jesse) needed several hashing functions for python and then kept adding things as I needed them
- **Q:** Does this have anything to do with the (excellent) package manager `rye`?
  - **A:** short answer: no. long answer: no, it does not.
- **Q:** Why is the repo split into `ry` and `ryo3`?
  - **A:** `ry` is the python package, `ryo3` is a rust crate setup to let you "register" functions you may want if you
    were writing your own pyo3-python bindings library; maybe someday the `ryo3::libs` module will be split up into
    separate packages

## Crate bindings

- `brotli`
- `bzip2`
- `flate2`
- `fnv`
- `shlex`
- `walkdir`
- `which`
- `xxhash`
- `zstd`
- TBD:
  - `subprocess.redo` (subprocesses that are lessy finicky and support tee-ing)
  - `globset` (technically done, but not yet in `ry` -- [globsters](https://pypi.org/project/globsters/))
  - `regex`
  - `tokio` (fs + process)
  - `tracing` (could be nicer than python's awful logging lib -- currently a part of ry/ryo3 for my dev purposes)
  - `reqwest` (async http client / waiting on pyo3 asyncio to stablize and for me to have more time)

## API

```python
```

## DEV

- `just` is used to run tasks
- Do not use the phrase `blazing fast` or any emojis in any PRs or issues or docs
- type annotations are required
- `ruff` used for formatting and linting

## SEE ALSO

- utiles (web-map tile utils): https://github.com/jessekrubin/utiles
- jsonc2json (jsonc to json converter): https://github.com/jessekrubin/jsonc2json
