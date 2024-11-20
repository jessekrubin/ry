import itertools as it

import ry.dev as ry


# ====================
# SPAN
# ====================
def test_find_duration_between_datetimes() -> None:
    """
    ```rust
    let zdt1 = date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")?;
    let zdt2 = date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")?;
    let span = &zdt2 - &zdt1;
    assert_eq!(span.to_string(), "PT29341h3m");
    ```
    """
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    span = zdt2 - zdt1
    assert span.string() == "PT29341h3m"


def test_span_negate() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    span = zdt2 - zdt1
    assert span.string() == "PT29341h3m"
    span_negated = -span
    assert span_negated.string() == "-PT29341h3m"

    span_inverted = ~span
    assert span_inverted.string() == "-PT29341h3m"


def test_span_2_duration() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    span = zdt2 - zdt1
    duration = span.to_jiff_duration(zdt2)
    assert duration == ry.SignedDuration(secs=105627780, nanos=0)


# ====================
# round mode
# ====================

JIFF_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
]

JIFF_ROUND_MODES = [
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half_ceil",
    "half_floor",
    "half_expand",
    "half_trunc",
    "half_even",
]


def test_datetime_round_options() -> None:
    default = ry.DateTimeRound()
    expected_default_string = (
        'DateTimeRound(smallest="nanosecond", mode="half_expand", increment=1)'
    )
    assert str(default) == expected_default_string

    for unit, mode in it.product(JIFF_UNITS, JIFF_ROUND_MODES):
        options = ry.DateTimeRound(smallest=unit, mode=mode, increment=1)  # type: ignore[arg-type]

        options_chained = ry.DateTimeRound().smallest(unit).mode(mode).increment(1)  # type: ignore[arg-type]
        expected_string = (
            f'DateTimeRound(smallest="{unit}", mode="{mode}", increment=1)'
        )
        assert str(options) == expected_string
        assert options == options_chained
