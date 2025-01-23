# food-4-thought

thinking out loud...

___

# `ry.dev`

For people who find `ry.dev` it is a module that exports all the things in ry as well as can be used as a repl; `python -m ry.dev` will start a repl (with ipython if installed else python-repl) with all of ry already imported. I use this super often for testing things out.

___

## string-bridge?

The `jiter` crate uses a string-cache to store python-strings to avoid the
overhead of converting strings to python strings. A global string bridge and/or
caching setup for other types of structs that often convert to strings might be
worth considering?

___

## Naming

Coming up with names is hard... I want to strike a balance between being clear
but also close to the wrapped libraries...

- Should jiff's `Zoned` be `Zoned` in python? or `ZonedDateTime`? (currently `ZonedDateTime`)
- Should jiff's `Span` be `Span` in python? or `TimeSpan`? (currently `TimeSpan`)
- Should reqwest's `Client` be `Client` in python? or `HttpClient`? (currently `HttpClient`)

___

## Flat? Nested submodules?

I like flat more, but nesting submodules might be preferable for some people and would allow for more flexibility in naming...

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
    - importing `ry.jiff` (or `ry.ryo3.jiff` tbd) is more explicitly the `jiff` wrapper(s)
  - cons:
    - Don't know how type annotations should be laid out... if there is a submodule called `ry.ryo3.reqwest`, do you import from `ry.ryo3.reqwest` or do I reexport from `ry.reqwest`? Then were doe the type-annotations live and how are they laid out without having to duplicate/shim them?
