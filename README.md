# ry

rust + python kitchen sink library

## What and why?

This is a collection of pyo3-wrappers for rust crates I wish existed in python.

It all started with me wanting a fast `fnv1a-64`

## Crate bindings

- `which`
- `fnv`
- `shlex`
- `walkdir`
- TBD:
  - `globset`
  - `regex`
  - `ignore`
  - `tokio` (fs + process)
  - `tracing` (could be nicer than python's awful logging lib)

## DEV

`just` is used to run tasks.
