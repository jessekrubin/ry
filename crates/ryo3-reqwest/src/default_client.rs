use crate::client::RyHttpClient;
use parking_lot::Mutex;
use std::sync::OnceLock;

static DEFAULT_CLIENT: OnceLock<Mutex<RyHttpClient>> = OnceLock::new();

#[inline]
pub(crate) fn default_client() -> &'static Mutex<RyHttpClient> {
    DEFAULT_CLIENT.get_or_init(|| {
        let client = RyHttpClient::new(None)
            .expect("Failed to create default client. This should never happen.");
        Mutex::new(client)
    })
}
