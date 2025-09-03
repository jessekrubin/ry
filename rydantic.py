from pydantic.functional_validators import BeforeValidator
import ry
import pytest
from typing import Any
from rich import print
import pydantic
import datetime as pydt
from pydantic import AfterValidator, GetPydanticSchema, PlainSerializer, WithJsonSchema
from typing_extensions import Annotated
from pydantic_core import core_schema


def _to_ry_date(v):
    if isinstance(v, ry.Date):
        return v
    if isinstance(v, pydt.date):
        # If ry.Date has a dedicated ctor, use it; otherwise parse ISO.
        # Replace with ry.Date.from_ymd(v.year, v.month, v.day) if you prefer.
        return ry.Date.parse(v.isoformat())
    if isinstance(v, str):
        return ry.Date.parse(v)
    raise TypeError(f"Cannot coerce {type(v).__name__} to ry.Date")


class PyDateModel(pydantic.BaseModel):
    date: pydt.date


class RyDateModel(pydantic.BaseModel):
    date: ry.Date


@pytest.mark.parametrize(
    "data",
    [
        pydt.date(2020, 1, 1),
        pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.timezone.utc),
        ry.Date(2020, 1, 1),
        "2020-01-01",
    ],
)
def test_date_inputs(data: Any):
    print(f"Input: {data!r}")
    # py_model = PyDateModel(date=data)
    ry_model = RyDateModel(date=data)
    # print(f"  PyDateModel: {py_model.date!r} ({type(py_model.date).__name__})")
    print(f"  RyDateModel: {ry_model.date!r} ({type(ry_model.date).__name__})")
    # assert py_model.date.isoformat() == ry_model.date.isoformat()

    assert isinstance(ry_model.date, ry.Date)

    # py_model_dump = PyDateModel(date=data).model_dump()


@pytest.mark.parametrize(
    "data",
    [
        pydt.date(2020, 1, 1),
        pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.timezone.utc),
        ry.Date(2020, 1, 1),
        ry.Date(2020, 1, 1).at(1, 2, 3, 4),
        ry.Date(2020, 1, 1).at(1, 2, 3, 4).in_tz("America/Los_Angeles"),
        "2020-01-01",
    ],
)
def test_date_inputs2(data: Any):
    print(f"Input: {data!r}")
    # py_model = PyDateModel(date=data)
    ry_model = RyDateModel(date=data)
    # print(f"  PyDateModel: {py_model.date!r} ({type(py_model.date).__name__})")
    print(f"  RyDateModel: {ry_model.date!r} ({type(ry_model.date).__name__})")
    # assert py_model.date.isoformat() == ry_model.date.isoformat()

    assert isinstance(ry_model.date, ry.Date)

    model_dumped_json = ry_model.model_dump_json()

    print(f"  RyDateModel JSON: {model_dumped_json}")

    from_json = RyDateModel.model_validate_json(model_dumped_json)

    print(f"  RyDateModel from JSON: {from_json!r} ({type(from_json).__name__})")
    assert from_json == ry_model

    #
    assert False


def _diff_schemas(left, right):
    left_no_title = {k: v for k, v in left.items() if k != "title"}
    right_no_title = {k: v for k, v in right.items() if k != "title"}
    assert left_no_title == right_no_title


def test_date_json_schema():
    py_model = PyDateModel.model_json_schema()
    ry_model = RyDateModel.model_json_schema()
    _diff_schemas(py_model, ry_model)
