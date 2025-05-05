from __future__ import annotations

import ry


def test_validating_a_particular_date_format() -> None:
    re = ry.Regex(r"^\d{4}-\d{2}-\d{2}$")
    assert re.is_match("2010-03-14")
    assert re.is_match("𝟚𝟘𝟙𝟘-𝟘𝟛-𝟙𝟜")  # noqa: RUF001


def test_replacement_with_named_capture_groups() -> None:
    re = ry.Regex(r"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})")
    before = "1973-01-05, 1975-08-25 and 1980-10-18"
    after = re.replace_all(before, "$m/$d/$y")
    assert after == "01/05/1973, 08/25/1975 and 10/18/1980"


def test_replacement_with_named_capture_groups_verbose() -> None:
    re = ry.Regex(r"""(?x)
        (?P<y>\d{4}) # the year, including all Unicode digits
        -
        (?P<m>\d{2}) # the month, including all Unicode digits
        -
        (?P<d>\d{2}) # the day, including all Unicode digits
    """)
    before = "1973-01-05, 1975-08-25 and 1980-10-18"
    after = re.replace_all(before, "$m/$d/$y")
    assert after == "01/05/1973, 08/25/1975 and 10/18/1980"


def test_finding_dates_in_a_haystack() -> None:
    re = ry.Regex(r"\d{4}-\d{2}-\d{2}")
    hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?"
    dates = re.find_all(hay)
    dates_str = [hay[start:end] for start, end in dates]
    assert dates_str == [
        "1865-04-14",
        "1881-07-02",
        "1901-09-06",
        "1963-11-22",
    ]
