# `ryo3-jiff`

ryo3-wrapper for `jiff` crate

[//]: # "<GENERATED>"

## Ref:

- docs.rs: [https://docs.rs/jiff](https://docs.rs/jiff)
- crates: [https://crates.io/crates/jiff](https://crates.io/crates/jiff)

[//]: # "</GENERATED>"

---

## Reference

### Span ranges

| Unit           | Minimum Value                | Maximum Value               |
| -------------- | ---------------------------- | --------------------------- |
| `years`        | `-19_998`                    | `19_998`                    |
| `months`       | `-239_976`                   | `239_976`                   |
| `weeks`        | `-1_043_497`                 | `1_043_497`                 |
| `days`         | `-7_304_484`                 | `7_304_484`                 |
| `hours`        | `-175_307_616`               | `175_307_616`               |
| `minutes`      | `-10_518_456_960`            | `10_518_456_960`            |
| `seconds`      | `-631_107_417_600`           | `631_107_417_600`           |
| `milliseconds` | `-631_107_417_600_000`       | `631_107_417_600_000`       |
| `microseconds` | `-631_107_417_600_000_000`   | `631_107_417_600_000_000`   |
| `nanoseconds`  | `-9_223_372_036_854_775_807` | `9_223_372_036_854_775_807` |

### Round options

| Type                  | Min Unit     | Max Unit | Defaults (smallest / mode / increment) |
| --------------------- | ------------ | -------- | -------------------------------------- |
| `TimestampRound`      | `nanosecond` | `hour`   | `nanosecond` / `half-expand` / `1`     |
| `TimeRound`           | `nanosecond` | `hour`   | `nanosecond` / `half-expand` / `1`     |
| `DateTimeRound`       | `nanosecond` | `day`    | `nanosecond` / `half-expand` / `1`     |
| `ZonedDateTimeRound`  | `nanosecond` | `day`    | `nanosecond` / `half-expand` / `1`     |
| `SignedDurationRound` | `nanosecond` | `hour`   | `nanosecond` / `half-expand` / `1`     |
| `OffsetRound`         | `second`     | `hour`   | `second` / `half-expand` / `1`         |
| `SpanRound`           | `nanosecond` | `year`   | `nanosecond` / `half-expand` / `1`     |

### Difference options

| Type            | Min Unit     | Max Unit | Default Mode |
| --------------- | ------------ | -------- | ------------ |
| `Timestamp`     | `nanosecond` | `hour`   | `trunc`      |
| `Time`          | `nanosecond` | `hour`   | `trunc`      |
| `Date`          | `nanosecond` | `year`   | `trunc`      |
| `DateTime`      | `nanosecond` | `year`   | `trunc`      |
| `ZonedDateTime` | `nanosecond` | `year`   | `trunc`      |

### `strftime`/`__format__` specifiers/directives & flags ([ref](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html#conversion-specifications))

| Specifier        | Example                      | Description                                                                     |
| ---------------- | ---------------------------- | ------------------------------------------------------------------------------- |
| `%%`             | `%%`                         | A literal `%`.                                                                  |
| `%A`, `%a`       | `Sunday`, `Sun`              | The full and abbreviated weekday, respectively.                                 |
| `%B`, `%b`, `%h` | `June`, `Jun`, `Jun`         | The full and abbreviated month name, respectively.                              |
| `%C`             | `20`                         | The century of the year. No padding.                                            |
| `%c`             | `2024 M07 14, Sun 17:31:59`  | The date and clock time via [`Custom`]. Supported when formatting only.         |
| `%D`             | `7/14/24`                    | Equivalent to `%m/%d/%y`.                                                       |
| `%d`, `%e`       | `25`, ` 5`                   | The day of the month. `%d` is zero-padded, `%e` is space padded.                |
| `%F`             | `2024-07-14`                 | Equivalent to `%Y-%m-%d`.                                                       |
| `%f`             | `000456`                     | Fractional seconds, up to nanosecond precision.                                 |
| `%.f`            | `.000456`                    | Optional fractional seconds, with dot, up to nanosecond precision.              |
| `%G`             | `2024`                       | An [ISO 8601 week-based] year. Zero padded to 4 digits.                         |
| `%g`             | `24`                         | A two-digit [ISO 8601 week-based] year. Represents only 1969-2068. Zero padded. |
| `%H`             | `23`                         | The hour in a 24 hour clock. Zero padded.                                       |
| `%I`             | `11`                         | The hour in a 12 hour clock. Zero padded.                                       |
| `%j`             | `060`                        | The day of the year. Range is `1..=366`. Zero padded to 3 digits.               |
| `%k`             | `15`                         | The hour in a 24 hour clock. Space padded.                                      |
| `%l`             | ` 3`                         | The hour in a 12 hour clock. Space padded.                                      |
| `%M`             | `04`                         | The minute. Zero padded.                                                        |
| `%m`             | `01`                         | The month. Zero padded.                                                         |
| `%N`             | `123456000`                  | Fractional seconds, up to nanosecond precision. Alias for `%9f`.                |
| `%n`             | `\n`                         | Formats as a newline character. Parses arbitrary whitespace.                    |
| `%P`             | `am`                         | Whether the time is in the AM or PM, lowercase.                                 |
| `%p`             | `PM`                         | Whether the time is in the AM or PM, uppercase.                                 |
| `%Q`             | `America/New_York`, `+0530`  | An IANA time zone identifier, or `%z` if one doesn't exist.                     |
| `%:Q`            | `America/New_York`, `+05:30` | An IANA time zone identifier, or `%:z` if one doesn't exist.                    |
| `%q`             | `4`                          | The quarter of the year. Supported when formatting only.                        |
| `%R`             | `23:30`                      | Equivalent to `%H:%M`.                                                          |
| `%r`             | `8:30:00 AM`                 | The 12-hour clock time via [`Custom`]. Supported when formatting only.          |
| `%S`             | `59`                         | The second. Zero padded.                                                        |
| `%s`             | `1737396540`                 | A Unix timestamp, in seconds.                                                   |
| `%T`             | `23:30:59`                   | Equivalent to `%H:%M:%S`.                                                       |
| `%t`             | `\t`                         | Formats as a tab character. Parses arbitrary whitespace.                        |
| `%U`             | `03`                         | Week number. Week 1 is the first week starting with a Sunday. Zero padded.      |
| `%u`             | `7`                          | The day of the week beginning with Monday at `1`.                               |
| `%V`             | `05`                         | Week number in the [ISO 8601 week-based] calendar. Zero padded.                 |
| `%W`             | `03`                         | Week number. Week 1 is the first week starting with a Monday. Zero padded.      |
| `%w`             | `0`                          | The day of the week beginning with Sunday at `0`.                               |
| `%X`             | `17:31:59`                   | The clock time via [`Custom`]. Supported when formatting only.                  |
| `%x`             | `2024 M07 14`                | The date via [`Custom`]. Supported when formatting only.                        |
| `%Y`             | `2024`                       | A full year, including century. Zero padded to 4 digits.                        |
| `%y`             | `24`                         | A two-digit year. Represents only 1969-2068. Zero padded.                       |
| `%Z`             | `EDT`                        | A time zone abbreviation. Supported when formatting only.                       |
| `%z`             | `+0530`                      | A time zone offset in the format `[+-]HHMM[SS]`.                                |
| `%:z`            | `+05:30`                     | A time zone offset in the format `[+-]HH:MM[:SS]`.                              |
| `%::z`           | `+05:30:00`                  | A time zone offset in the format `[+-]HH:MM:SS`.                                |
| `%:::z`          | `-04`, `+05:30`              | A time zone offset in the format `[+-]HH:[MM[:SS]]`.                            |

| Flag | Description                                        |
| ---- | -------------------------------------------------- |
| `-`  | Do not pad a numeric result.                       |
| `_`  | Pad a numeric result to the left with spaces.      |
| `0`  | Pad a numeric result to the left with zeros.       |
| `^`  | Use alphabetic uppercase for all relevant strings. |
| `#`  | Swap the case of the result string.                |
