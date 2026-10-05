pub(crate) type Depth = u8;
pub(crate) const MAX_DEPTH: Depth = 255;

/// serde err msg for max depth exceeded
pub const RECURSION_ERR_MSG: &str = "recursion";
