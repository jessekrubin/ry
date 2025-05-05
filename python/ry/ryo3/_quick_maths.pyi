"""ryo3-quick-maths types"""

import typing as t

def quick_maths() -> t.Literal[3]:
    """Performs quick-maths

    Implements the algorithm for performing "quick-maths" as described by
    Big Shaq in his PHD thesis, 2017, in which he states:

    > "2 plus 2 is 4, minus one that's 3, quick maths." (Big Shaq et al., 2017)

    Reference:
        https://youtu.be/3M_5oYU-IsU?t=60

    Example:
        >>> import ry
        >>> result = ry.quick_maths()
        >>> assert result == 3

    NOTE: THIS IS FROM MY TEMPLATE RY03-MODULE
    """
