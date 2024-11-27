from __future__ import annotations

import dataclasses

import pytest

import ry


@dataclasses.dataclass
class Shplitest:
    string: str
    expected: list[str]
    skip: bool = False


# typos:off
SHPLITESTS = [
    Shplitest(string="x", expected=["x"]),
    Shplitest(string="foo bar", expected=["foo", "bar"]),
    Shplitest(string=" foo bar", expected=["foo", "bar"]),
    Shplitest(string=" foo bar ", expected=["foo", "bar"]),
    Shplitest(
        string="foo   bar    bla     fasel",
        expected=["foo", "bar", "bla", "fasel"],
        skip=False,
    ),
    Shplitest(string="x y  z              xxxx", expected=["x", "y", "z", "xxxx"]),
    Shplitest(string="\\x bar", expected=["x", "bar"]),
    Shplitest(string="\\ x bar", expected=[" x", "bar"]),
    Shplitest(string="\\ bar", expected=[" bar"]),
    Shplitest(string="foo \\x bar", expected=["foo", "x", "bar"]),
    Shplitest(string="foo \\ x bar", expected=["foo", " x", "bar"]),
    Shplitest(string="foo \\ bar", expected=["foo", " bar"]),
    Shplitest(string='foo "bar" bla', expected=["foo", "bar", "bla"]),
    Shplitest(string='"foo" "bar" "bla"', expected=["foo", "bar", "bla"]),
    Shplitest(string='"foo" bar "bla"', expected=["foo", "bar", "bla"]),
    Shplitest(string='"foo" bar bla', expected=["foo", "bar", "bla"]),
    Shplitest(string="foo 'bar' bla", expected=["foo", "bar", "bla"]),
    Shplitest(string="'foo' 'bar' 'bla'", expected=["foo", "bar", "bla"]),
    Shplitest(string="'foo' bar 'bla'", expected=["foo", "bar", "bla"]),
    Shplitest(string="'foo' bar bla", expected=["foo", "bar", "bla"]),
    Shplitest(
        string='blurb foo"bar"bar"fasel" baz',
        expected=["blurb", "foobarbarfasel", "baz"],
        skip=False,
    ),
    Shplitest(
        string="blurb foo'bar'bar'fasel' baz",
        expected=["blurb", "foobarbarfasel", "baz"],
        skip=False,
    ),
    Shplitest(string='""', expected=[""]),
    Shplitest(string="''", expected=[""]),
    Shplitest(string='foo "" bar', expected=["foo", "", "bar"]),
    Shplitest(string="foo '' bar", expected=["foo", "", "bar"]),
    Shplitest(string='foo "" "" "" bar', expected=["foo", "", "", "", "bar"]),
    Shplitest(string="foo '' '' '' bar", expected=["foo", "", "", "", "bar"]),
    Shplitest(string='\\"', expected=['"']),
    Shplitest(string='"\\""', expected=['"']),
    Shplitest(string='"foo\\ bar"', expected=["foo\\ bar"]),
    Shplitest(string='"foo\\\\ bar"', expected=["foo\\ bar"]),
    Shplitest(string='"foo\\\\ bar\\""', expected=['foo\\ bar"']),
    Shplitest(string='"foo\\\\" bar\\"', expected=["foo\\", 'bar"']),
    Shplitest(string='"foo\\\\ bar\\" dfadf"', expected=['foo\\ bar" dfadf']),
    Shplitest(string='"foo\\\\\\ bar\\" dfadf"', expected=['foo\\\\ bar" dfadf']),
    Shplitest(string='"foo\\\\\\x bar\\" dfadf"', expected=['foo\\\\x bar" dfadf']),
    Shplitest(string='"foo\\x bar\\" dfadf"', expected=['foo\\x bar" dfadf']),
    Shplitest(string="\\'", expected=["'"]),
    Shplitest(string="'foo\\ bar'", expected=["foo\\ bar"]),
    Shplitest(string="'foo\\\\ bar'", expected=["foo\\\\ bar"]),
    Shplitest(
        string='"foo\\\\\\x bar\\" df\'a\\ \'df"',
        expected=["foo\\\\x bar\" df'a\\ 'df"],
        skip=False,
    ),
    Shplitest(string='\\"foo', expected=['"foo']),
    Shplitest(string='\\"foo\\x', expected=['"foox']),
    Shplitest(string='"foo\\x"', expected=["foo\\x"]),
    Shplitest(string='"foo\\ "', expected=["foo\\ "]),
    Shplitest(string="foo\\ xx", expected=["foo xx"]),
    Shplitest(string="foo\\ x\\x", expected=["foo xx"]),
    Shplitest(string='foo\\ x\\x\\"', expected=['foo xx"']),
    Shplitest(string='"foo\\ x\\x"', expected=["foo\\ x\\x"]),
    Shplitest(string='"foo\\ x\\x\\\\"', expected=["foo\\ x\\x\\"]),
    Shplitest(string='"foo\\ x\\x\\\\""foobar"', expected=["foo\\ x\\x\\foobar"]),
    Shplitest(
        string='"foo\\ x\\x\\\\"\\\'"foobar"',
        expected=["foo\\ x\\x\\'foobar"],
        skip=False,
    ),
    Shplitest(
        string='"foo\\ x\\x\\\\"\\\'"fo\'obar"',
        expected=["foo\\ x\\x\\'fo'obar"],
        skip=False,
    ),
    Shplitest(
        string="\"foo\\ x\\x\\\\\"\\'\"fo'obar\" 'don'\\''t'",
        expected=["foo\\ x\\x\\'fo'obar", "don't"],
        skip=False,
    ),
    Shplitest(
        string="\"foo\\ x\\x\\\\\"\\'\"fo'obar\" 'don'\\''t' \\\\",
        expected=["foo\\ x\\x\\'fo'obar", "don't", "\\"],
        skip=False,
    ),
    Shplitest(string="'foo\\ bar'", expected=["foo\\ bar"]),
    Shplitest(string="'foo\\\\ bar'", expected=["foo\\\\ bar"]),
    Shplitest(string="foo\\ bar", expected=["foo bar"]),
    Shplitest(string="foo#bar\nbaz", expected=["foo", "baz"], skip=True),
    Shplitest(string=":-) ;-)", expected=[":-)", ";-)"]),
    Shplitest(string="áéíóú", expected=["áéíóú"]),
]


# typos:on


@pytest.mark.parametrize("data", SHPLITESTS)
def test_shplit(data: Shplitest) -> None:
    if data.skip:
        pytest.skip(f"skipping {data}")
    assert ry.shplit(data.string) == data.expected
