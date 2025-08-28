"""
ulid tests (adapted from `python-ulid`)

ADAPTED FROM `python-ulid`'s TESTS; REF https://github.com/mdomke/python-ulid/blob/main/tests/test_ulid.py

removed freezegun as it is not really needed nor does pyo3 stuff respsect it
"""

from __future__ import annotations

import json
from typing import Annotated

import pytest

from ry.ulid import ULID


def _pydantic_installed() -> bool:
    try:
        import pydantic  # noqa: F401

        return True
    except ImportError:
        return False


@pytest.mark.skipif(not _pydantic_installed(), reason="pydantic is not installed")
class TestUlidPydantic:
    def test_pydantic_protocol(self) -> None:
        from pydantic import BaseModel, ValidationError

        ulid = ULID()

        class Model(BaseModel):
            ulid: ULID | None = None

        for value in [ulid, str(ulid), int(ulid), bytes(ulid)]:
            model = Model(ulid=value)  # type: ignore[arg-type]
            assert isinstance(model.ulid, ULID)
            assert model.ulid == ulid

        for value in [b"not-enough", "not-enough"]:
            with pytest.raises(ValidationError):
                Model(ulid=value)  # type: ignore[arg-type]

        model = Model(ulid=ulid)
        model_dict = model.model_dump()
        ulid_from_dict = model_dict["ulid"]
        assert ulid_from_dict == ulid
        assert isinstance(ulid_from_dict, ULID)
        assert Model(**model_dict) == model

        model_json = model.model_dump_json()
        assert isinstance(json.loads(model_json)["ulid"], str)
        assert Model.model_validate_json(model_json) == model

        model_json_schema = model.model_json_schema()
        assert "properties" in model_json_schema
        assert "ulid" in model_json_schema["properties"]
        assert "anyOf" in model_json_schema["properties"]["ulid"]
        assert {
            "maxLength": 26,
            "minLength": 26,
            "pattern": "[A-Z0-9]{26}",
            "type": "string",
        } in model_json_schema["properties"]["ulid"]["anyOf"]
        assert {
            "maxLength": 16,
            "minLength": 16,
            "type": "string",
            "format": "binary",
        } in model_json_schema["properties"]["ulid"]["anyOf"]
        assert {
            "type": "null",
        } in model_json_schema["properties"]["ulid"]["anyOf"]

    def test_pydantic_protocol_strict_errors(self) -> None:
        import pydantic

        ulid = ULID()

        class ModelStrict(pydantic.BaseModel):
            ulid: Annotated[ULID | None, pydantic.Field(strict=True)]

            model_config = {
                "arbitrary_types_allowed": True,
                "strict": True,
            }

        strict_ok_inputs = [ulid, str(ulid)]
        for value in strict_ok_inputs:
            model = ModelStrict(ulid=value)  # type: ignore[arg-type]
            assert isinstance(model.ulid, ULID)
            assert model.ulid == ulid

        for value in [int(ulid), bytes(ulid)]:
            with pytest.raises(pydantic.ValidationError):
                ModelStrict(ulid=value)  # type: ignore[arg-type]

    def test_pydantic_protocol_strict_ok(self) -> None:
        import pydantic

        ulid = ULID()

        class ModelStrict(pydantic.BaseModel):
            ulid: Annotated[ULID | None, pydantic.Field(strict=True)]

            model_config = {
                "arbitrary_types_allowed": True,
                "strict": True,
            }

        strict_ok_inputs = [ulid, str(ulid)]
        for value in strict_ok_inputs:
            model = ModelStrict(ulid=value)  # type: ignore[arg-type]
            assert isinstance(model.ulid, ULID)
            assert model.ulid == ulid
            model_dict = model.model_dump()
            ulid_from_dict = model_dict["ulid"]
            assert ulid_from_dict == ulid
            assert isinstance(ulid_from_dict, ULID)
            assert ModelStrict(**model_dict) == model

            model_json = model.model_dump_json()
            assert isinstance(json.loads(model_json)["ulid"], str)
            assert ModelStrict.model_validate_json(model_json) == model

            model_json_schema = model.model_json_schema()
            assert "properties" in model_json_schema
            assert "ulid" in model_json_schema["properties"]
            assert "anyOf" in model_json_schema["properties"]["ulid"]
            assert {
                "maxLength": 26,
                "minLength": 26,
                "pattern": "[A-Z0-9]{26}",
                "type": "string",
            } in model_json_schema["properties"]["ulid"]["anyOf"]
            assert {
                "type": "null",
            } in model_json_schema["properties"]["ulid"]["anyOf"]
