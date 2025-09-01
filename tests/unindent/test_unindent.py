from __future__ import annotations

import ry


def test_unindent_docs_example() -> None:
    indented = """
    line one
    line two"""
    unindented = ry.unindent(indented)
    assert unindented == "line one\nline two"


def test_unindent_docs_example_final_new_line() -> None:
    indented = """
    line one
    line two
    """
    unindented = ry.unindent(indented)
    assert unindented == "line one\nline two\n"


def test_unindent_docs_example_bytes() -> None:
    indented = b"""
    line one
    line two"""
    unindented = ry.unindent_bytes(indented)
    assert unindented == b"line one\nline two"


def test_unindent_docs_example_final_new_line_bytes() -> None:
    indented = b"""
    line one
    line two
    """
    unindented = ry.unindent_bytes(indented)
    assert unindented == b"line one\nline two\n"
