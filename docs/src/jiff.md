# jiff

docs.rs: [https://docs.rs/jiff](https://docs.rs/jiff)

crates: [https://crates.io/crates/jiff](https://crates.io/crates/jiff)

---

The `jiff` crate has a super excellent API that is very ergonomic to use and
`ry` provides a nearly complete wrapper around it.

A good amount of time, and a greater amount of thought has gone into balancing
the `jiff` python api to be both ergonomic and performant.

## python-conversions

The structs under `ryo3-jiff` are convertible to/from python's `datetime.*`
types and the conversions are pretty well tested (ty hypothesis).

### hand rolled -> `jiff-02` -> hand rolled (again (2026-10-15))

The conversions to/from python `datetime.*` types were originally hand rolled (
by me (jesse)) using the 'new-type' pattern and were copy-pasta-ed from the pyo3 chrono conversions.
`pyo3-v0.24.0` added conversions via the `jiff-02` feature flag, which `ry-v0.0.38` thru `ry-v0.0.102` used pieces of internally, but they were never FULLY used.

As of `ry-v0.0.103` the `jiff-02` feature is no longer used and ALL conversions
are hand rolled new-types again (`JiffDate`, `JiffZoned`, `JiffTimeZoneRef`,
...). Why:

- a `jiff` bindings wrapper really should have its own conversions (imo)
- `Zoned` -> `datetime.datetime` in pyo3's `jiff-02` sets the wrong `fold` for
  the 2nd occurrence of a repeated (fall-back) hour, so python reads it as the
  1st occurrence (an hour off); the `ry` conversion gets it right
- `jiff-02` pins the `jiff` version to whatever `pyo3` supports
- `pyo3` does not convert `Span` -> `datetime.timedelta`, but `ry` does
- the hand rolled versions are as fast or faster (eg `datetime` w/
  `timezone.utc` -> `Zoned` skips the tz lookup)

^those optimizations should be PR-ed into pyo3 at some point soon (TODO<@jesse>)

---

## `ry` vs `whenever`

There is another library called
[`whenever`](https://github.com/ariebovenberg/whenever) that provides a similar
datetime library to that of `ryo3-jiff` (both `jiff` and `whenever` are based on
the [temporal](https://tc39.es/proposal-temporal/docs/) API).

No formal benchmarks between `ry` and `whenever` have been done, but I have
copy-pasta-ed some of the benchmarks from the `whenever` repo and translated
them to `ry` and the results were pretty similar; `whenever` is faster for some
things, `ry` is faster for others, but both are wildly more performant than
python's built in `datetime` module and `pendulum` -- differences in performance
are almost all measured in nanoseconds.

Big shoutout to "Mr. Dutch Airlines" guy
([@ariebovenberg](https://github.com/ariebovenberg)) who wrote `whenever`! Love
the name of the library too!
