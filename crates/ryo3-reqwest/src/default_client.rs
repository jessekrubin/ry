use crate::client::RyHttpClient;
use std::sync::Mutex;
use std::sync::OnceLock;

static DEFAULT_CLIENT: OnceLock<Mutex<RyHttpClient>> = OnceLock::new();

#[inline]
pub(crate) fn default_client() -> &'static Mutex<RyHttpClient> {
    DEFAULT_CLIENT.get_or_init(|| Mutex::new(RyHttpClient(reqwest::Client::new())))
}
