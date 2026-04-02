//! constants/defaults
use ryo3_std::time::PyTimeout;
use std::num::NonZeroUsize;

/// default max payload length 64MiB (`67_108_864b`) (defined by tokio-websockets)
pub(crate) const DEFAULT_MAX_PAYLOAD_LEN: NonZeroUsize =
    NonZeroUsize::new(64 * 1024 * 1024).expect("wenodis");
/// default frame size 4MiB (`4_194_304b`) (as defined by tokio-websockets)
pub(crate) const DEFAULT_FRAME_SIZE: NonZeroUsize =
    NonZeroUsize::new(4 * 1024 * 1024).expect("wenodis");
/// default flush threshold for sending messages 8KiB (`8_192b`) (as defined by tokio-websockets)
pub(crate) const DEFAULT_FLUSH_THRESHOLD: NonZeroUsize =
    NonZeroUsize::new(8 * 1024).expect("wenodis");
/// default timeout for the closing handshake
pub(crate) const DEFAULT_CLOSE_TIMEOUT: PyTimeout = PyTimeout::from_secs(10);
/// websocket close-reason max length
pub(crate) const WS_MSG_CLOSE_REASON_MAX_LEN: usize = 123;
/// websocket ping/pong payload max length
pub(crate) const WS_MSG_PINGPONG_PAYLOAD_MAX_LEN: usize = 125;
