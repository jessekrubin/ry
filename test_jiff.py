import ry.dev as ry
from ry.dev import Timestamp

ts = Timestamp.now()

print(ts)

ts_string = ts.to_string()

ts_from_string = Timestamp.parse(ts_string)
print(ts_from_string)

print(ts == ts_from_string)
anotherone = "2024-11-11T18:47:34.639485Z"
print(Timestamp.parse(anotherone))
print(Timestamp.parse(anotherone) == ts_from_string)

print(Timestamp.__module__
      )

print(Timestamp.__name__, Timestamp.__qualname__)

d = ry.Duration.zero()
print(d)

somedur = ry.Duration(1999, 3)
print(somedur)


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
    assert span.to_string() == "PT29341h3m"


def test_span_negate() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    span = zdt2 - zdt1
    assert span.to_string() == "PT29341h3m"
    span_negated = -span
    assert span_negated.to_string() == "-PT29341h3m"

    span_inverted = ~span
    assert span_inverted.to_string() == "-PT29341h3m"


def test_span_2_duration() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    span = zdt2 - zdt1
    duration = span.to_jiff_duration(
        zdt2
    )
    print(duration)
