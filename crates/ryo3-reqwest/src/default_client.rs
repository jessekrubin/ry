use crate::async_client::RyAsyncClient;
use std::sync::Mutex;
use std::sync::OnceLock;

static DEFAULT_CLIENT: OnceLock<Mutex<RyAsyncClient>> = OnceLock::new();

#[inline]
pub(crate) fn default_client() -> &'static Mutex<RyAsyncClient> {
    DEFAULT_CLIENT.get_or_init(|| Mutex::new(RyAsyncClient(reqwest::Client::new())))
}
