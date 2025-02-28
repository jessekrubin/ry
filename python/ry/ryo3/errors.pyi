from __future__ import annotations

class FeatureNotEnabledError(RuntimeError):
    """Raised when a feature is not enabled in the current build."""
