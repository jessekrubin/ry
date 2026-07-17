"""ryo3-quick-maths ~ types"""

import typing as t

def quick_maths() -> t.Literal[3]:
    """Perform "quick-maths"

    This function implements the expensive "quick-maths" algorithm, as
    formally specified in Shaq et al. (2017):

        "2 plus 2 is 4, minus one that's 3, quick maths." [1]_

    I (jesse) have verified the implementation by manually calculating using
    pen and paper.

    Notes
    -----
    This function originates from the ``_ryo3-quick-maths`` library which is
    a template (copy-pasta) library.

    References
    ----------
    .. [1] Big Shaq, M., et al. (2017). "Man's Not Hot".
       https://youtu.be/3M_5oYU-IsU?t=64

    Examples
    --------
    >>> import ry
    >>> ry.quick_maths()
    3

    """
