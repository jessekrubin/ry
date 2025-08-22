from __future__ import annotations

import json

import pytest

import ry

_ORJSON_INSTALLED: bool = False
try:
    import orjson

    _ORJSON_INSTALLED = True
except ImportError:
    orjson = None  # type: ignore[assignment]

pytest_mark_skip_orjson = pytest.mark.skipif(
    not _ORJSON_INSTALLED,
    reason="orjson is not installed, skipping tests that require it",
)

RYTYPES_JSON_SER = {
    # uuid ~ ryo3-uuid
    "uuid": ry.uuid.UUID("88475448-f091-42ef-b574-2452952931c1"),
    # ulid ~ ryo3-ulid
    "ulid": ry.ulid.ULID("01H7Z5F8Y3V9G4J6K8D5E6F7G8"),
    # url ~ ryo3-url
    "url": ry.URL("https://example.com"),
    # http
    "headers": ry.Headers({
        "Content-Type": "application/json",
        "Accept": "application/json",
        "X-Content-Type-Options": "nosniff",
    }),
    "http-status": ry.HttpStatus(200),
    # jiff ~ ryo3-jiff
    "date": ry.date(2020, 8, 26),
    "datetime": ry.datetime(2020, 8, 26, 6, 27, 0, 0),
    "+signed_duration": ry.SignedDuration(3),
    "-signed_duration": -ry.SignedDuration(3),
    "time": ry.time(6, 27, 0, 0),
    "timespan": ry.timespan(weeks=1),
    "timestamp": ry.Timestamp.from_millisecond(1598438400000),
    "zoned": ry.datetime(2020, 8, 26, 6, 27, 0, 0).in_tz("America/New_York"),
}
EXPECTED = {
    "uuid": "88475448-f091-42ef-b574-2452952931c1",
    "ulid": "01H7Z5F8Y3V9G4J6K8D5E6F7G8",
    "url": "https://example.com/",
    "headers": {
        "accept": "application/json",
        "content-type": "application/json",
        "x-content-type-options": "nosniff",
    },
    "http-status": 200,
    "date": "2020-08-26",
    "datetime": "2020-08-26T06:27:00",
    "+signed_duration": "PT3S",
    "-signed_duration": "-PT3S",
    "time": "06:27:00",
    "timespan": "P1W",
    "timestamp": "2020-08-26T10:40:00Z",
    "zoned": "2020-08-26T06:27:00-04:00[America/New_York]",
}


@pytest_mark_skip_orjson
def test_orjson_fails_normally() -> None:
    """Ensure that orjson fails normally when it cannot serialize a type."""
    assert orjson is not None, "orjson should be installed for this test"
    with pytest.raises(orjson.JSONEncodeError):
        orjson.dumps(RYTYPES_JSON_SER)


@pytest_mark_skip_orjson
def test_orjson_default() -> None:
    """Test that orjson can serialize RY types with the default handler."""
    assert orjson is not None, "orjson should be installed for this test"
    result = orjson.dumps(RYTYPES_JSON_SER, default=ry.orjson_default)
    assert isinstance(result, bytes), "orjson.dumps should return bytes"
    parsed = json.loads(result.decode("utf-8"))
    assert parsed == EXPECTED, "orjson default serialization did not match expected"
