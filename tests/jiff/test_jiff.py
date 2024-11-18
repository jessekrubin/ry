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
    print(duration)
