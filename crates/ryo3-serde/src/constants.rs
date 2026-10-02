pub(crate) type Depth = u8;
pub(crate) const MAX_DEPTH: Depth = 255;

/// Message of the serde error raised when the max depth is exceeded.
///
/// Serializers can compare against this to detect the recursion error.
pub const RECURSION_ERR_MSG: &str = "recursion";
