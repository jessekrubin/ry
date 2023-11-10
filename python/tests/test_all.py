import pytest
import ry


def test_sum_as_string():
    assert ry.sum_as_string(1, 1) == "2"
