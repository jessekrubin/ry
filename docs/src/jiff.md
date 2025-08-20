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

### `pyo3-v0.24.0` & `jiff-02`

The conversions to/from python `datetime.*` types were originally hand rolled (
by me (jesse)) using the 'new-type' pattern, HOWEVER `pyo3-v0.24.0` provides
conversions via the `jiff-02` feature flag, which is what is used now.

`ry-v0.0.37` will be the last version with the mostly hand rolled conversions.

`ry-v0.0.38` will be the first version with the `jiff-02` feature flag.

As of 2025-03-12 `pyo3` does not seem to support converting `Span` ->
`datetime.timedelta`, so that is still hand rolled.

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
