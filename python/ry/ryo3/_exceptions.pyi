import typing as t

class PanicException(BaseException):
    """python fatal panic"""

class FeatureNotEnabledError(RuntimeError):
    """Raised when a feature is not enabled in the current build"""

class UnreachableError(AssertionError):
    """Raised when unreachable code is reached"""

def unreachable(msg: str | None = None) -> t.NoReturn: ...
def panic(msg: str) -> t.NoReturn: ...
