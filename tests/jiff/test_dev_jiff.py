from __future__ import annotations

from hypothesis import strategies as st

import ry.dev as ry

timedelta_strategy = st.timedeltas()
"""
use jiff::{civil::date, ToSpan};

let earlier = date(2006, 8, 24).at(22, 30, 0, 0).in_tz("America/New_York")?;
let later = date(2019, 1, 31).at(21, 0, 0, 0).in_tz("America/New_York")?;
assert_eq!(earlier.until(&later)?, 109_031.hours().minutes(30));

// Flipping the dates is fine, but you'll get a negative span.
assert_eq!(later.until(&earlier)?, -109_031.hours().minutes(30));
"""


def test_dev() -> None:
    assert True
