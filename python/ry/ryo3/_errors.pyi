import typing as t

class PanicError(BaseException):
    """panic == fatal python error"""

class FeatureNotEnabledError(RuntimeError):
    """Raised when a feature is not enabled in the current build"""

class UnreachableError(AssertionError):
    """Raised when unreachable code is reached"""

def unreachable(msg: str | None = None) -> t.NoReturn:
    """raise `UnreachableError` with given message

    Parameters
    ----------
    msg : str | None, optional
        message to include in the error, by default None

    Returns
    -------
    t.NoReturn
        always raises UnreachableError

    Raises
    ------
    UnreachableError
        always
    """

def panic(msg: str | None = None) -> t.NoReturn:
    """raise `PanicError` with given message

    Parameters
    ----------
    msg : str | None, optional
        message to include in the error, by default None

    Returns
    -------
    t.NoReturn
        always raises PanicError

    Raises
    ------
    PanicError
        always
    """
