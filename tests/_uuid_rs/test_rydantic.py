"""Test ry-uuid pydantic integration

copy-pasta-ed bits and bobs from pydantic's own uuid related tests

REF: https://github.com/pydantic/pydantic/blob/main/tests/test_types.py

"""

import uuid as pyuuid
from typing import Any

import pydantic
import pytest

from ry import uuid as ryuuid

_TEST_CASES = [
    (
        "ebcdab58-6eb8-46fb-a190-d07a33e9eac8",
        pyuuid.UUID("ebcdab58-6eb8-46fb-a190-d07a33e9eac8"),
    ),
    (
        pyuuid.UUID("ebcdab58-6eb8-46fb-a190-d07a33e9eac8"),
        pyuuid.UUID("ebcdab58-6eb8-46fb-a190-d07a33e9eac8"),
    ),
    (
        b"ebcdab58-6eb8-46fb-a190-d07a33e9eac8",
        pyuuid.UUID("ebcdab58-6eb8-46fb-a190-d07a33e9eac8"),
    ),
    (b"\x12\x34\x56\x78" * 4, pyuuid.UUID("12345678-1234-5678-1234-567812345678")),
    ("ebcdab58-6eb8-46fb-a190-", pydantic.ValidationError),
    (123, pydantic.ValidationError),
]


class PyUuidModel(pydantic.BaseModel):
    uu: pyuuid.UUID


class RyUuidModel(pydantic.BaseModel):
    uu: ryuuid.UUID


class TestJsonSchemas:
    def _diff_schemas(self, left: dict[str, Any], right: dict[str, Any]) -> None:
        left_no_title = {k: v for k, v in left.items() if k != "title"}
        right_no_title = {k: v for k, v in right.items() if k != "title"}
        assert left_no_title == right_no_title

    def test_uuid_json_schema(self) -> None:
        py_model = PyUuidModel.model_json_schema()
        ry_model = RyUuidModel.model_json_schema()
        self._diff_schemas(py_model, ry_model)


@pytest.mark.parametrize("value,expected", _TEST_CASES)
def test_uuid_model(
    value: str | bytes | pyuuid.UUID,
    expected: pyuuid.UUID | type[pydantic.ValidationError],
) -> None:
    model_cls = RyUuidModel
    if expected is pydantic.ValidationError:
        with pytest.raises(pydantic.ValidationError):
            _m = model_cls(uu=value)  # type: ignore[arg-type]
    else:
        m = model_cls(uu=value)  # type: ignore[arg-type]
        assert isinstance(m.uu, ryuuid.UUID)
        assert m.uu == expected
        as_json = m.model_dump_json()
        from_json = model_cls.model_validate_json(as_json)
        assert from_json.uu == expected
