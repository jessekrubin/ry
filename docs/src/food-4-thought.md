# food-4-thought

thinking out loud...

___

## string-bridge?

The `jiter` crate uses a string-cache to store python-strings to avoid the
overhead of converting strings to python strings. A global string bridge and/or
caching setup for other types of structs that often convert to strings might be
worth considering?

## Naming

Coming up with names is hard... I want to strike a balance between being clear
but also close to the wrapped libraries...

- Should jiff's `Zoned` be `Zoned` in python? or `ZonedDateTime`? (currently `ZonedDateTime`)
- Should jiff's `Span` be `Span` in python? or `TimeSpan`? (currently `TimeSpan`)
- Should reqwest's `Client` be `Client` in python? or `HttpClient`? (currently `HttpClient`)
