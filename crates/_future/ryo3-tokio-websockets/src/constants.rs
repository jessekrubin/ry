//! constants/defaults

/// default max payload length (as defined by tokio-websockets)
pub(crate) const DEFAULT_MAX_PAYLOAD_LEN: usize = 64 * 1024 * 1024; // 64 MiB
/// default frame size for sending messages (as defined by tokio-websockets)
pub(crate) const DEFAULT_FRAME_SIZE: usize = 4 * 1024 * 1024; // 4 MiB
/// default flush threshold for sending messages (as defined by tokio-websockets)
pub(crate) const DEFAULT_FLUSH_THRESHOLD: usize = 8 * 1024; // 8 KiB
