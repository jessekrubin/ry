from __future__ import annotations

from hypothesis import strategies as st


def st_json(*, finite_only=True):
    """Helper function to describe JSON objects, with optional inf and nan.

    Taken from hypothesis docs

    REF: https://hypothesis.readthedocs.io/en/latest/tutorial/custom-strategies.html#writing-helper-functions
    """
    numbers = st.floats(allow_infinity=not finite_only, allow_nan=not finite_only)
    return st.recursive(
        st.none() | st.booleans() | st.integers() | numbers | st.text(),
        extend=lambda xs: st.lists(xs) | st.dictionaries(st.text(), xs),
    )
