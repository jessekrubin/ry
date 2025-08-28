# food-4-thought

thinking out loud...

## staticmethod vs classmethod [2025-08-28]

Nowhere in ry are any of the `classmethod` functions actually used as classmethods, they are effectively staticmethods; they don't access the class or instance in any way.
Classes in `ry` do not (for the most part) support being subclassed.
Benchmarking shows that `staticmethod` is slightly faster than `classmethod`,
sooooo all classmethods will be removed, but added back in if needed later...

Benchmarking code:

```python
from __future__ import annotations

import json
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

import ry as ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture


def test_classmethod(benchmark: BenchmarkFixture):
    # this is the current (as of 2025-08-28) `#[staticmethod]` parse function
    benchmark(ry.Date.parse, "2023-03-15")


def test_staticmethod(benchmark: BenchmarkFixture):
    # This is a crudely copy-pasted version using `#[staticmethod]` instead
    benchmark(ry.Date.parse2, "2023-03-15")
```

Benchmark results:

```
---------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in ns)         Min                    Max                Mean              StdDev              Median               IQR             Outliers  OPS (Mops/s)            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_staticmethod     57.5000 (1.0)       2,290.0000 (1.0)       63.4931 (1.0)       16.2941 (1.0)       62.0000 (1.0)      1.5000 (824.63)  1048;11454       15.7497 (1.0)      100000         200
test_classmethod      99.9989 (1.74)     29,200.0004 (12.75)    128.8420 (2.03)     146.9563 (9.02)     100.0008 (1.61)     0.0018 (1.0)      911;24504        7.7614 (0.49)     100000           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
```


---

## fugue-state-jesse [2025-08-18]

The most sophisticated rust code in this repository was written by
`fugue-state-jesse` (aka FSJ), not me (`normal-jesse`) and not an AI/LLM.
`fugue-state-jesse` only shows up at random about 1-2 times a week and
seemingly has a vastly better understanding of rust and rust-macro-rules than I
do.

Throughout the repository I (normal-jesse) will sometimes leave notes to
`fugue-state-jesse` with the hope that he might do what I ask, but he tends to
do his own thing.

---

## `ry.dev`

For people who find `ry.dev` it is a module that exports all the things in ry as
well as can be used as a repl; `python -m ry.dev` will start a repl (with
ipython if installed else python-repl) with all of ry already imported. I
(jesse) use this super often for testing things out.

---

## string-bridge?

The `jiter` crate uses a string-cache to store python-strings to avoid the
overhead of converting strings to python strings. A global string bridge and/or
caching setup for other types of structs that often convert to strings might be
worth considering?

---

## Naming

Coming up with names is hard... I (jesse) want to strike a balance between being
clear but also close to the wrapped libraries...

- Should jiff's `Zoned` be `Zoned` in python? or `ZonedDateTime`? (currently
  `ZonedDateTime`)
- Should jiff's `Span` be `Span` in python? or `TimeSpan`? (currently
  `TimeSpan`)
- Should reqwest's `Client` be `Client` in python? or `HttpClient`? (currently
  `HttpClient`)

---

## Flat? Nested submodules?

I like flat more, but nesting submodules might be preferable for some people and
would allow for more flexibility in naming...

pros & cons:

- flat:
  - pros:
    - easier to import
    - easier to work on
    - no need to remember where things are
    - type annotations are easier to setup/dist
  - cons:
    - name conflicts
    - type annotations are harder to read bc of huge file
    - harder to remember where things are
- nested:
  - pros:
    - no name conflicts
    - easier to remember where things are
    - type annotations are easier to read
    - importing `ry.jiff` (or `ry.ryo3.jiff` tbd) is more explicitly the `jiff`
      wrapper(s)
  - cons:
    - Don't know how type annotations should be laid out... if there is a
      submodule called `ry.ryo3.reqwest`, do you import from `ry.ryo3.reqwest`
      or do we reexport from `ry.reqwest`? Then were doe the type-annotations
      live and how are they laid out without having to duplicate/shim them?

---

## pypi size limit

The pypi project size limit of 10gb was reached. I (jesse) won't request a limit
raise until the package is more stable and hits some sort of `v0.1.x`, SOOOOOO
for now I will be:

- deleting older versions of ry from pypi as needed
- update the release gh-action to push the built wheels to the releases page so
  they are not lost into the ether...
