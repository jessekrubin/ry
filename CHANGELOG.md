# CHANGELOG

## v0.0.24 [2024-12-24] (the night b4 xmas...)


- `http`
  - basic headers struct/obj -- WIP
- `reqwest`
  - reqwest client (currently root-export)
  - default client + root `fetch` function likely needs work...
  - response `byte_stream`!


___

## v0.0.23 [2024-12-19]

- `python -m ry.dev` repl for ipython/python repl ~ handy nifty secret tool makes it into repo
- internal
  - in process of renaming all python-rust `#[new]` functions to be named `fn py_new(...)`
- `unindent`
  - Added `unindent` module for unindenting strings will move to `ryo3-unindent`
- `FsPath` 
  - creeping ever closer to being a full-fledged pathlib.Path replacement 
  - Added bindings to all rust `std::path::Path(buf)` methods for `FsPath`
- sub-packaging
  - `xxhash` is own sub package now `ry.xxhash`
  - `JSON` is own subpackage right now -- named `ry.JSON` to avoid conflict with `json` module but maybe will change...
  - food-for-thought-ing how `ryo3` and `ry` should be organized w/ respsect to sub-packages and where that organization should be
- type-annotations
  - required to break up the type annotations due to migration to sub-packages
  - breaking up the type annotations file into smaller files under `<REPO>/python/ry/ryo3/*.pyi`

___

## v0.0.22 [2024-12-16]

- `regex`
  - Super simple regex wrapper (must to do here, but was added for `ryo3-which::which_re`)
- `jiff`
  - `until`/`since`
    - Basic `until`/`since` implementation but I do not like them and they confusingly named `*Difference` structs/py-objects, so I may change how they work...
  - `jiff` seems to be about as performant as `whenever` ~ yay! also the whenever dude appears to be watching this repo (as of 2024-12-16)
- `walkdir`
  - `collect` added to `WalkdirGen` to collect the results into a list
- deps
  - `thiserror` version `2.0.7` -> `2.0.8`

___

## v0.0.21 [2024-12-13] (friday the 13th... spoogidy oogidity)

- `walkdir`
  - add `glob` kwarg that takes a `ry.Glob` or `ry.GlobSet` or `ry.Globster` obj to filter the walk on
- `globset`
  - Internal refactoring
  - added `globster()` method to `ry.Glob` and `ry.GlobSet` to return a `ry.Globster` obj
  - added `globset()` method to `ry.Glob` to return a `ry.GlobSet` obj from a `ry.Glob` obj
- `url`
  - python `Url` changed name `URL`; aligns with jawascript and other python libs
- `bzip2`
  - update to v5
- `jiff`
  - conversions for jiff-round-mode/unit/weekday
  - not-implemented placeholders and new impls
    - [x] `RyDateTime`
    - [x] `RyDate`
    - [x] `RyOffset`
    - [x] `RySignedDuration`
    - [x] `RySpan`
    - [x] `RyTimeZone`
    - [x] `RyTime`
    - [x] `RyZoned`
  - span builder functions use form `s._hours(1)` for panic-inducing building, and `s.try_hours(1)` for non-panic-inducing building
- type-annotations
  - fixes and updates and a hacky script I wrote to check for discrepancies

---

## v0.0.20 [2024-12-10]

- `regex`
  - Templated out regex package but nothing added
- `ry`
  - python 3.13 yay!
- `jiter`
  - Updated jiter version thanks depbot!

---

## v0.0.19 [2024-12-05]

- `jiff`
  - py-conversions
    - [x] `JiffDateTime`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffDate`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffOffset`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffSignedDuration`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffSpan`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffTimeZone`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffTime`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)
    - [x] `JiffZoned`
      - [x] FromPyObject
      - [x] IntoPyObject
      - [x] IntoPyObject (REF)

---

## v0.0.18 [2024-12-03]

- `jiff`
  - Renamed `ry.Span` to `ry.TimeSpan`
  - Renamed `ry.Zoned` to `ry.ZonedDateTime`
  - Updated type stubs to reflect renames
- docs
  - init-ed the docs
  - style guide under `DEVELOPMENT.md` file

---

## v0.0.17 [2024-12-02]

- `jiff`
  - `ry.TimeZone` testing and to/from `datetime.tzinfo` conversions
  - Using nu-types for `jiff` intermediate types bc of the classic orphans problem (aka batman) w/ traits
  - hypothesis tests
- `jiter`
  - Updated to `jiter` v0.8.1

---

## v0.0.16 [2024-11-29]

- Moved walkdir to `ryo3-walkdir`
- added `ryo3-types` for custom and shared types
- `heck` wrapper(s)
- jiff
  - Added operators `+`/`+=`/`-`/`-=` to date/time/datetime/etc
  - TODO: figure out how to take refs in the union enum for the operators
- fspath
  - further beefing out as well as testing

---

## v0.0.15 [2024-11-20]

- `from __future__ import annotations` added to all modules
- cicd updated to include more targets

---

## v0.0.14 [2024-11-20]

- Primitive/crude wrappers around Mr. Sushi's `jiff` library
- Updated to use pyo3 (had to use jiter git repo dep)
- `ry.FsPath` beefed out
- Added iterdir gen wrapper
- (todo undo when jiter + pyo3 23 is public)

---

## v0.0.13 [2024-11-20]

- **VERSION SKIPPED DUE TO `13` BEING SPOOKY AND ME BEING MODERATELY-STITCHOUS (AKA fully 'superstitchous')**

---

## v0.0.12 [2024-11-14]

- sqlformat wrapper(s) (this is the first `ryo3-*` sub-crate)

---

## v0.0.11 [2024-09-22]

- dependencies updated
- prepare for python 3.13

---

## v0.0.10 [2024-09-22]

- dependencies updated

---

## v0.0.9 [2024-08-22]

- Added `globset` wrapper(s)
- Added `__init__.py` generator

---

- Upgraded to pyo3-v0.22

## v0.0.8 [2024-06-25]

- Upgraded to pyo3-v0.22

---

## v0.0.7 [2024-06-08]

- internal refactoring

---

## v0.0.6 [2024-06-05]

- Added zstd (`zstd_encode`/`zstd` and `zstd_decode`)
- Added gzip (`gzip_encode`/`gzip` and `gzip_decode`/`gunzip`)
- Added bzip2 (`bzip2_encode`/`bzip2` and `bzip2_decode`)
- Added walkdir
- Reorg libs

---

## v0.0.5 [2024-04-19]

- Added brotli (`brotli_encode` and `brotli_decode`)
- xxhash
  - const functions
  - hasher streaming objects
