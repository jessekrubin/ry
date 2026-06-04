import typing as t

class PanicError(BaseException):
    """panic == fatal python error"""

class FeatureNotEnabledError(RuntimeError):
    """Raised when a feature is not enabled in the current build"""

class UnreachableError(AssertionError):
    """Raised when unreachable code is reached"""

def unreachable(msg: str | None = None) -> t.NoReturn:
    """raise UnreachableError with the given message

    Raises:
        UnreachableError: always
    """

def panic(msg: str | None = None) -> t.NoReturn:
    """panic with the given message

    Raises:
        PanicException: always
    """
