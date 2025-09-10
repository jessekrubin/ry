//! "new" types for the http crate
//!
//! Used for the py-conversions mod

use http::{HeaderMap, HeaderValue};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderMap(pub HeaderMap<HeaderValue>);
#[derive(Debug, Clone, PartialEq)]
pub struct HttpMethod(pub http::Method);
#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderName(pub http::HeaderName);
#[derive(Debug)]
pub struct HttpHeaderNameRef<'a>(pub &'a http::HeaderName);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HttpStatusCode(pub http::StatusCode);
#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderValue(pub HeaderValue);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HttpVersion(pub http::Version);

// ============================================================================
//  FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM
// ============================================================================

impl From<HeaderMap> for HttpHeaderMap {
    fn from(h: HeaderMap) -> Self {
        Self(h)
    }
}

impl From<HttpHeaderName> for http::HeaderName {
    fn from(h: HttpHeaderName) -> Self {
        h.0
    }
}

impl From<HttpHeaderValue> for HeaderValue {
    fn from(h: HttpHeaderValue) -> Self {
        h.0
    }
}

impl From<HttpMethod> for http::Method {
    fn from(h: HttpMethod) -> Self {
        h.0
    }
}

impl From<http::HeaderName> for HttpHeaderName {
    fn from(h: http::HeaderName) -> Self {
        Self(h)
    }
}

impl From<HeaderValue> for HttpHeaderValue {
    fn from(h: HeaderValue) -> Self {
        Self(h)
    }
}

impl From<http::Method> for HttpMethod {
    fn from(h: http::Method) -> Self {
        Self(h)
    }
}

impl From<http::StatusCode> for HttpStatusCode {
    fn from(h: http::StatusCode) -> Self {
        Self(h)
    }
}

impl From<http::Version> for HttpVersion {
    fn from(h: http::Version) -> Self {
        Self(h)
    }
}

// impl from ref
impl From<&HeaderValue> for HttpHeaderValue {
    // clone should be totally fine bc the http lib uses the `Bytes` crate
    fn from(h: &HeaderValue) -> Self {
        Self(h.clone())
    }
}

// ============================================================================
// DEREF
// ============================================================================
impl Deref for HttpHeaderName {
    type Target = http::HeaderName;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for HttpHeaderValue {
    type Target = HeaderValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for HttpMethod {
    type Target = http::Method;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ============================================================================
