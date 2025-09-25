# CHANGELOG

## v0.0.59 [2025-09-24]

- `ryo3-request`
  - switched query/form kwargs to accept anything that can be serde-d via
    `ryo3-serde` which is basically anything that is json-serializable via
    `ry.stringify`
- `ry.protocols`
  - moved all protocols to `ry.protocols`
- deprecations
  - deprecate all `obj.string()` methods in favor of `obj.to_string()` tho I
    still dont love it in the first place
- `ryo3-jiff`
  - added `isoformat` and `from_isoformat` methods to `ry.TimeSpan` and
    `ry.SignedDuration` structs
- `ryo3-sqlformat`
  - Updated to version 0.4.0 of `sqlformat` crate
  - Added sqlformat version 0.4.0 new options:
    - `ignore_case_convert: list[str] | None = None`
    - `inline: bool = False`
    - `max_inline_block: int = 50`
    - `max_inline_arguments: int | None = None`
    - `max_inline_top_level: int | None = None`
    - `joins_as_top_level: bool = False`
  - Changed `indent` arg/kwarg to accept either:
    - `int` (positive integer for number of spaces)
    - `str` ("tabs", "\t" or "spaces")
  - Changed `uppercase` arg/kwarg to default to `False` instead of `True` to be
    more inline with the default behaviour of `sqlformat` crate

---

## v0.0.58 [2025-09-18]

- `ryo3-jiff`
  - added `.__format__()` methods to several jiff structs to allow custom
    f-string formatting
  - Fixed `SignedDuration.__truediv__` operator
- internal
  - migrated all `downcast*` usages to `cast*`
- Min python version for ry is now 3.11+

---

## v0.0.57 [2025-09-12]

- `ryo3-jiff`
  - Added `TimeRound` python struct
  - Fixed types for all `*Round` operations limiting max "smallest" arg literal
    types.
  - Round-api changes
    - builder-y functions and getter functions have been flip-flopped:
      - Switched "builder"-y apis to start with prefix for all `*Round` structs:
        - `increment(n: int) -> Self` -> `_increment(n: int) -> Self`
        - `mode(m: str) -> Self` -> `_mode(m: str) -> Self`
        - `smallest(unit: str) -> Self` -> `_smallest(unit: str) -> Self`
      - Switched all getter functions to be properties:
        - `round_obj._increment() -> int` -> `round_obj.increment -> int`
        - `round_obj._mode() -> str` -> `round_obj.mode -> str`
        - `round_obj._smallest() -> str` -> `round_obj.smallest -> str`
- `to_dict()`
  - `.asdict()` renamed to `.to_dict()` all structs
  - renames structs:
    - `ry.DateTime.asdict()` -> `ry.DateTime.to_dict()`
    - `ry.Date.asdict()` -> `ry.Date.to_dict()`
    - `ry.TimeSpan.asdict()` -> `ry.TimeSpan.to_dict()`
    - `ry.Time.asdict()` -> `ry.Time.to_dict()`
    - `ry.Headers.asdict()` -> `ry.Headers.to_dict()`
  - Added `.to_dict()` to:
- migrated from `xxhash-rust` to `twox-hash` ~ retiring `ryo3-xxhash` :( - the
  `xxhash-rust` hashers liked to sometimes crash, whereas the `twox-hash`
  py-hashers dont

---

## v0.0.56 [2025-09-05]

- `ryo3-serde`
  - refactoring and testing recursion and stitch
- `ryo3-tokio`
  - Fix: python open mode parsing for `aiopen` function
- `ryo3-reqwest`
  - Add `jiter` parsing options to `Response.json()`
- `ryo3-jiff`
  - use `#[pyo3(warn(...))]` for deprecation warnings instead of doing it
    manually
  - fixed utc methods to use `.with_time_zone(TimeZone::UTC)` instead of
    `.in_tz("UTC")`

---

## v0.0.55 [2025-09-03]

- upgrade pyo3 v0.26.x
- `ryo3-bytes`
  - Update buffer usage based on kyle barron `pyo3-bytes`
    [changes](https://github.com/developmentseed/obstore/commit/2ca22a8c3949ae51fbf750ef5a08e3a76f583819)
- `ryo3-std`
  - Make each sub-module a feature flag `std-net`, `std-fs`, `std-time`, etc...
- internal changes
  - Implemented `Display` for several types for use in their `__repr__` methods

---

## v0.0.54 [2025-08-28]

- `ryo3-std`
  - Serialization for std types
  - Speed run of socketaddr types (WIP); needs more testing and the socket types
    could be cleaner...
- `ryo3-memchr`
  - Basic functionality for `memchr` and `memrchr` operations
- `ryo3-jiff`
  - Changed `human` arg/kwarg in `ry.TimeSpan` and `ry.SignedDuration` to
    `friendly` and also make keyword only
  - Changed `strptime` and `strftime` functions to be more inline with python's
    `datetime` module by changing the order of args to be `(string, format)`
    instead of `(format, string)`; the strptime signature is
    `strptime(s: str, /, fmt: str) -> Self`
  - Added to `ry.TimeSpan` and `ry.SignedDuration` the `friendly` method for
    more natural string representations
  - Many internal refactors and cleanup
  - Converted all `__repr__` methods to use struct `Display` impls
  - Fixed rounding object repr function(s) and added pickling and tests for
    round objects
- type-annotations
  - Missing `lstrip`/`rstrip` method types for `ry.Bytes`
  - Updated types for `ry.TimeSpan` and `ry.SignedDuration` w/ correct
    `friendly` kwarg and `friendly()` methods
- Added ruff `A002` lint
- Added ruff `FBT` lints

---

## v0.0.53 [2025-08-18]

- `ry`
  - Bump min python version 3.10 -- this is a breaking change, but ry is still
    very much a WIP/in-beta, so the versioning schema is "yolo-versioning"
- `ryo3-serde`
  - internal refactoring and cleanup

---

## v0.0.52 [2025-07-30]

- `ryo3-bytes`
  - internal refactoring
  - added
    - `ry.Bytes.__rmul__`
    - `ry.Bytes.lstrip`
    - `ry.Bytes.rstrip`
- `ryo3-xxhash`
  - all xxhash-ing classes are now `frozen` pyclasses
    [#259](https://github.com/jessekrubin/ry/issues/259)

---

## v0.0.51 [2025-07-25]

- `ryo3-bytes`
  - Separated `pyo3-bytes` and `ryo3-bytes`
    - `pyo3-bytes` mirrors the official `pyo3-bytes` crate + extra methods, BUT
      it requires the `multiple-pymethods` feature to be enabled
    - `ryo3-bytes` is a crammed together version of the `pyo3-bytes`
      implementation and extra methods and does NOT require the
      `multiple-pymethods` feature to be enabled
  - Made `PythonBytesMethods` trait for the methods that are shared between
    `pyo3-bytes` and `ryo3-bytes`
- `ryo3-ulid`
  - strict + lax ulid parsing for pydantic
- `ryo3-jiff`
  - Renamed `checked_add` and `checked_sub` to `add` and `sub` where the
    checked_version can error; did not remove where the checked version returns
    an `Option` type (`ry.SignedDuration`). `.checked_add` may return later as a
    method that returns an `Option` type for all types (tbd). This is also meant
    to pave the way for `add`/`sub` functions with a more familiar api akin to
    `whenever`, `pendulum`, `arrow`, `insert-other-datetime-lib-here`
  - Added `replace` methods to `Date`, `DateTime` and `Time` structs that use
    the underlying jiff `with` functions

---

## v0.0.50 [2025-07-14]

- internal
  - clippy lint fixes `unused_self` (all but `ryo3-bytes` which needs its own
    cleanup)
- `ryo3-bytes`
  - Added (bc I need them) more python compat methods:
    - `title()`
    - `swapcase()`
    - `expandtabs()`
    - `strip()`
- `ryo3-fspath`
  - Added `open` method that forwards to `open` method of `pathlib.Path`
  - Added `mkdir` method that mimics `mkdir` method of `pathlib.Path`

---

## v0.0.49 [2025-07-04] (fourth o july)

- workspace
  - set rust edition to 2024
- `ryo3-serde`
  - Fixed recursive serialization w/ max depth of 255 (aligning with `orjson`)
  - support `PyEllipsis` for `None` values in serialization
- `ryo3-json`
  - `minify` function to remove whitespace/newlines from json-string/bytes
- `ryo3-jiff`
  - internal refactoring
  - `isoformat` methods aligned with python's `datetime` library methods
  - Freeze (make pyclass frozen) for all jiff types (changed `*Series`
    iterables)
- `ryo3-fspath`
  - `which` feature allowing `FsPath.which` and `FsPath.which_all`

---

## v0.0.48 [2025-06-24]

- `ryo3-json`
  - `pybytes` bool kwargs to return `builtins.bytes` if `True` and `ry.Bytes` if
    `False`; default is `False`
- `ryo3-serde`
  - support for types defined in `ryo3-http`
  - support for `default` kwarg that is passed to the serde serializer; like w/
    the stdlib-json and orjson serializers, this allows for serializing types
    that are not natively supported by ry/serde and if failure should occur, it
    should raise a `TypeError` or `ValueError` instead of returning `None` by
    default
- `ryo3-reqwest`
  - `json` kwarg added to request builders that auto-serializes via
    `ryo3-serde`; also because it uses the `reqwest::RequestBuilder` it auto
    sets the `Content-Type` header to `application/json`

---

## v0.0.47 [2025-06-17]

- pyo3 v0.25.1
- `ryo3-serde` (wip)
  - serializers for `PyAny` and more
  - this should theoretically allow for serializing any python object that is
    `serde` serializable with almost any `serde` serializer... that is the goal
- `ryo3-json`
  - Where json stuff + ry is going to live in the near future (may consolidate
    `ryo3-jiter` into this newer crate)
  - `ry.stringify()` uses `ryo3-serde` + `serde_json` to write json bytes/bufs
    and it is pretty fast, faster than ujson and rapidjson (not tested yyjson),
    BUT orjson is still fastest (read a bunch of their code and it is remarkably
    advanced and optimized)

---

## v0.0.46 [2025-06-06]

- version 0.0.46
- `ryo3-reqwest`
  - `ResponseStream`
    - Added `__repr__` method
    - Added `async def take(self, n: int=1): ...` method returns n chunks as a
      list
    - Added `async def collect(self: join = False) -> ...:` method that collects
      the stream into a single `ry.Bytes` object if `join=True` or a list of
      `ry.Bytes` objects if `join=False`
    - Added `async def take(self, n: int=1): ...` which returns n chunks as a
      list
- `ryo3-glob`
  - add `dtype` kwarg that takes either `dtype=str | ry.FsPath | pathlib.Path`
    as type of obj yielded by the iterable; something about this feels really
    icky, the default may be changed to `str` (from `pathlib.Path`)
- `ryo3-ulid`
  - Added mostly as a way to test how much pydantic + ry integration would be
- `ryo3-which`
  - upgrade which to version 8

---

## v0.0.45 [2025-05-30]

- added `__target__` to python package metadata in `ry.__about__` with the
  target triple of the current build
- `ryo3-std`
  - Buffering for `FileReadStream`
- `ryo3-jiter`
  - Add function `parse_jsonl` for parsing json lines
  - Add `lines` kwarg to `read_json` for parsing/reading json lines
- `ryo3-jiff`
  - `ZonedDateTime.__new__` takes more python-datetime like args/kwargs, old
    version of constructor moved to classmethod
    `ZonedDateTime.from_parts(timestamp: ry.Timestamp, tz: ry.TimeZone) -> ZonedDateTime`
  - `zoned` top level function
    - if `tz` is `None` then it uses the system timezone
    - SIGNATURE

    ```python
    def zoned(
        year: int,
        month: int,
        day: int,
        hour: int = 0,
        minute: int = 0,
        second: int = 0,
        nanosecond: int = 0,
        tz: str | None = None,
    ) -> ZonedDateTime: ...
    ```

---

## v0.0.44 [2025-05-23]

- internal:
  - renamed `ryo3-macros` to `ryo3-macro-rules`
- docs
  - Cleaned up `./README.md`
  - Removed type-annotations from `./README.md`
- pyo3-v0.25.0
- py-types
  - reqwest-request functions use `TypedDict` and `Unpack`
- `ryo3-jiff`
  - serde serialization features/support
  - `ry.ZonedDateTime.replace` method mirroring `ZonedWith` -- `with` is a
    python keyword, so used `replace` instead
  - example script based on jiff-docs examples
  - `test_jiff_examples_v2.py` test script (basis for example script)
    - Was tired/fried so I copy-pasta-ed the `ry/ryo3/_jiff.pyi` type
      annotations, the jiff-v2-docs-examples, and the jiff-v1-hand-translated
      `test_jiff_examples_v1.py` file into Chad-Gippity who was able to do most
      of the translation from `rust` to `ry`...
- `ryo3-xxhash`
  - Align with `xxhash` pypi library w/ respect to naming conventions

---

## v0.0.43 [2025-05-17]

- `ryo3-jiff`
  - panic-able functions to create new/altered (time)spans moved to use `try_*`
- fix: anyio marker flat issue in pytests for cicd
- `ryo3-uuid`
  - added `uuid` wrapper for `uuid` crate; ty to the maintainers of `uuid-utils`
    and `fastuuid` for helping figure out some of the nitty gritty bits and bobs
- `ryo3-tokio`
  - `AsyncFile` and `aiopen` experiment(s) added for async file reading/writing
    etc

---

## v0.0.42 [2025-05-12]

- panic=abort
  - panic is now (maybe will go back) `abort` for release builds
  - means smaller binaries and faster error handling (in theory)
- `ryo3-reqwest`
  - more type fixes to response
  - Got response type more inline with other python http-client libraries
  - try `parking_lot` for default `reqwest` client mutex
  - include missing kwargs for fetch functions
- `ryo3-glob`
  - freeze struct(s) to be frozen
- `ryo3-http`
  - http version python conversions to/from string/int
  - crude-ish serde implementation for `HeadersMap` for json
    encoding/decoding... was a lot of googling
  - status code reason(s) interned
  - intern all standard http header-names
- `ryo3-fnv`
  - align with hashlib style hashing
- deps-up
  - pyo3 version 0.24.2
  - brotli 8
  - jiff patch

---

## v0.0.41 [2025-04-18]

- `ryo3-jiter`
  - added `read_json` function to read from path-like obj
- `ryo3-bytes`
  - misc small improvements and tests
- `ryo3-std`
  - `ry.IpAddr` added to handle both ipv4/ipv6
  - `ry.read_dir` implemented
- `ryo3-walkdir`
  - added `objects` impl and example script
- `ryo3-tokio`
  - `ry.read_dir_async` implemented; also contains fancy async take/collect

---

## v0.0.40 [2025-04-11]

- scripts
  - `dl_versions.py` script to download all versions of ry while ry is still
    pre-1-point-oh and old version(s) are being nuked from pypi as needed
- types
  - fix types for few packages
- Updated several dependencies ~ most notably `pyo3` to `0.24.1`
- Fixed several new clippy lints that appear in CI stable rust builds
- `ryo3-std`
  - `std::net` ipv4/ipv6 wrappers speed run impl

---

## v0.0.39 [2025-03-14]

- internal
  - cleaned up several dependencies and features
- `ryo3-zstd`
  - actually changed to use py buffer protocol this time... I dont know how it
    got missed before...
  - re-factored a decent bit and made submodule with future plans to expand
    encoding/decoding dictionary support
  - submodule is `ry.zstd` and/or `ry.ryo3.zstd`

---

## v0.0.38 [2025-03-13]

- `ryo3-reqwest`
  - client configuration for pickling
  - allow buffer-protocol for `body` fetching methods (should add string maybe?)
- `ryo3-walkdir`
  - Few more options added
- `ryo3-glob`
  - new wrapper around `glob` crate
- `ryo3-jiff`
  - Switched to use conversions from `jiff` feature of `pyo3-v24` as opposed to
    hand-rolled conversions we had before

---

## v0.0.37 [2025-03-11]

- pyo3 version `0.24.0`
- `ryo3-which` functions return `pathlib.Path` now due to changes in pyo3-v24;
  this may change in the near future...

---

## v0.0.36 [2025-03-11]

- dependencies updated
- pickling support and tests for several types
- bytes/buffer-protocol support for several sub-packages/packages:
  - `ryo3-brotli`
  - `ryo3-bzip2`
  - `ryo3-flate2`
  - `ryo3-fnv`
  - `ryo3-xxhash`
  - `ryo3-zstd`

---

## v0.0.35 [2025-03-06]

- internal
  - types split up and cleaned up
- `ryo3-size`
  - `ry.Size` object
- `ryo3-jiff`
  - `series` iterators have `take` function that takes a `usize` returns a list
    of size `usize`
  - updated series types to be `JiffSeries` class

---

## v0.0.34 [2025-02-28]

- `ryo3-std`
  - `fs`:
    - `read_stream` function that returns an iterator of `ry.Bytes` objects from
      a `PathLike` object
    - Several more fs functions added
- `ryo3-tokio`
  - Several more tokio fs functions added
- internal
  - reorganized type annotations to be not a HUGE file...

---

## v0.0.33 [2025-02-26]

- update to pyo3 v0.23.5

---

## v0.0.32 [2025-02-25]

- `ryo3-jiter`
  - Allow `PyBytes` wrapper/buffer protocol to be given
  - renamed `jiter_cache_clear` to `json_cache_clear` and `jiter_cache_usage` to
    `json_cache_usage`
  - Removed `parse_json_str` just use `parse_json` with `str` input
- `ryo3-fspath`
  - Allow read/write to take `ry.Bytes` or `Bytes` objects

---

## v0.0.31 [2025-02-21]

- `ryo3-core`
  - got rid of `ryo3-types` and moved into `ryo3-core`
- `ryo3-tokio`
  - `read_async` and `write_async` async functions
- `ryo3-which`
  - `which_re` functions accepts `ry.Regex` or `str` now
- `ryo3-std`
  - `read` and `write` functions which take/return `ry.Bytes` objects
- `internal`
  - Changed many many many of the structs/classes to be pyo3 `frozen` behaviour
    should not be different

---

## v0.0.30 [2025-02-18]

- `jiff`
  - Upgraded jiff to version 2
- internal
  - Switch all lints from `#[allow(...)]`/`#![allow(...)]` to
    `#[expect(...)]`/`#![expect(...)]`
  - Removed a bunch o commented out code
- `ryo3-std`
  - added several `std::fs` structs
- `ryo3-fspath`
  - conversion to `pathlib.Path` by way of `FsPath.to_pathlib()`

---

## v0.0.29 [2025-02-03]

- internal
  - Made sure each `ryo3-*` crate has a `README.md`
- `ryo3-bytes` & `ryo3-fspath`
  - added `__hash__` dunders to both `Bytes` and `FsPath` structs

---

## v0.0.28 [2025-01-31]

- `jiff`
  - Per Mr. Sushi's thoughts changed all `until`/`since` methods to use kwargs
    instead of the rust-like tuples that impl `From`/`Into` as it does not
    translate well to python
  - Gets rid of the following inane types:

```python
IntoDateDifference = (
    DateDifference
    | Date
    | DateTime
    | ZonedDateTime
    | tuple[JIFF_UNIT_STRING, Date]
    | tuple[JIFF_UNIT_STRING, DateTime]
    | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)
IntoTimeDifference = (
    TimeDifference
    | Time
    | DateTime
    | ZonedDateTime
    | tuple[JIFF_UNIT_STRING, Time]
    | tuple[JIFF_UNIT_STRING, DateTime]
    | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)
IntoDateTimeDifference = (
    DateTimeDifference
    | Date
    | Time
    | DateTime
    | ZonedDateTime
    | tuple[JIFF_UNIT_STRING, Date]
    | tuple[JIFF_UNIT_STRING, Time]
    | tuple[JIFF_UNIT_STRING, DateTime]
    | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)
IntoTimestampDifference = (
    TimestampDifference
    | Timestamp
    | ZonedDateTime
    | tuple[JIFF_UNIT_STRING, Timestamp]
    | tuple[JIFF_UNIT_STRING, ZonedDateTime]
)
```

---

## v0.0.27 [2025-01-23]

- `ry`
  - Warning on `debug` build
- `reqwest`
  - headers-property response returns `Headers` object instead of python dict
- `same-file`
  - wrapper module added with `is_same_file` py-fn (yet another piece of burnt
    sushi)
- `jiff`
  - jiff-version `0.1.25` ~ add `in_tz` methods and point old `intz` at new
    `in_tz` methods and raise `DeprecationWarning` for old `intz` methods
  - Continued adding implementations that previously raised
    `NotImplementedError`
    - `Date.nth_weekday_of_month`
    - `Date.nth_weekday`
    - `DateTime.nth_weekday_of_month`
    - `DateTime.nth_weekday`
    - `TimeSpan.compare`
    - `TimeSpan.total`
    - `ZonedDateTime.nth_weekday_of_month`
    - `ZonedDateTime.nth_weekday`

---

## v0.0.26 [2025-01-13]

- `reqwest`
  - `AsyncClient` renamed to `HttpClient`
- `jiff`
  - human timespan strings for `TimeSpan` and `SignedDuration` objects:
    - `ry.TimeSpan.parse("P2M10DT2H30M").string(human=True) == "2mo 10d 2h 30m"`
    - `ry.SignedDuration.parse("PT2H30M").string(human=True) == "2h 30m"`
- internal
  - workspace-ified all the deps

---

## v0.0.25 [2024-01-07] (25 for 2025)

- `jiff`
  - Updated to `0.1.21` which has span and signed duration strings with capital
    letters

---

## v0.0.24 [2024-12-24] (the night b4 xmas...)

- `http`
  - basic headers struct/obj -- WIP
- `reqwest`
  - reqwest client (currently root-export)
  - default client + root `fetch` function likely needs work...
  - response `byte_stream`!

---

## v0.0.23 [2024-12-19]

- `python -m ry.dev` repl for ipython/python repl ~ handy nifty secret tool
  makes it into repo
- internal
  - in process of renaming all python-rust `#[new]` functions to be named
    `fn py_new(...)`
- `unindent`
  - Added `unindent` module for unindenting strings will move to `ryo3-unindent`
- `FsPath`
  - creeping ever closer to being a full-fledged pathlib.Path replacement
  - Added bindings to all rust `std::path::Path(buf)` methods for `FsPath`
- sub-packaging
  - `xxhash` is own sub package now `ry.xxhash`
  - `JSON` is own subpackage right now -- named `ry.JSON` to avoid conflict with
    `json` module but maybe will change...
  - food-for-thought-ing how `ryo3` and `ry` should be organized w/ respsect to
    sub-packages and where that organization should be
- type-annotations
  - required to break up the type annotations due to migration to sub-packages
  - breaking up the type annotations file into smaller files under
    `<REPO>/python/ry/ryo3/*.pyi`

---

## v0.0.22 [2024-12-16]

- `regex`
  - Super simple regex wrapper (must to do here, but was added for
    `ryo3-which::which_re`)
- `jiff`
  - `until`/`since`
    - Basic `until`/`since` implementation but I do not like them and they
      confusingly named `*Difference` structs/py-objects, so I may change how
      they work...
  - `jiff` seems to be about as performant as `whenever` ~ yay! also the
    whenever dude appears to be watching this repo (as of 2024-12-16)
- `walkdir`
  - `collect` added to `WalkdirGen` to collect the results into a list
- deps
  - `thiserror` version `2.0.7` -> `2.0.8`

---

## v0.0.21 [2024-12-13] (friday the 13th... spoogidy oogidity)

- `walkdir`
  - add `glob` kwarg that takes a `ry.Glob` or `ry.GlobSet` or `ry.Globster` obj
    to filter the walk on
- `globset`
  - Internal refactoring
  - added `globster()` method to `ry.Glob` and `ry.GlobSet` to return a
    `ry.Globster` obj
  - added `globset()` method to `ry.Glob` to return a `ry.GlobSet` obj from a
    `ry.Glob` obj
- `url`
  - python `Url` changed name `URL`; aligns with jawascript and other python
    libs
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
  - span builder functions use form `s._hours(1)` for panic-inducing building,
    and `s.try_hours(1)` for non-panic-inducing building
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
  - Using nu-types for `jiff` intermediate types bc of the classic orphans
    problem (aka batman) w/ traits
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

- **VERSION SKIPPED DUE TO `13` BEING SPOOKY AND ME BEING MODERATELY-STITCHOUS
  (AKA fully 'superstitchous')**

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
