//! "new" types for the http crate
//!
//! Used for the py-conversions mod

use http::{HeaderMap, HeaderValue};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderMap(pub HeaderMap<HeaderValue>);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HttpMethod(pub http::Method);

impl HttpMethod {
    pub const GET: Self = Self(http::Method::GET);
    pub const POST: Self = Self(http::Method::POST);
    pub const PUT: Self = Self(http::Method::PUT);
    pub const DELETE: Self = Self(http::Method::DELETE);
    pub const HEAD: Self = Self(http::Method::HEAD);
    pub const OPTIONS: Self = Self(http::Method::OPTIONS);
    pub const PATCH: Self = Self(http::Method::PATCH);
    pub const TRACE: Self = Self(http::Method::TRACE);
    pub const CONNECT: Self = Self(http::Method::CONNECT);
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderName(pub http::HeaderName);
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HttpHeaderNameRef<'a>(pub &'a http::HeaderName);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HttpStatusCode(pub http::StatusCode);
#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderValue(pub HeaderValue);
#[derive(Debug, Clone, PartialEq)]
pub struct HttpHeaderValueRef<'a>(pub &'a HeaderValue);
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
impl From<HttpHeaderMap> for HeaderMap {
    fn from(h: HttpHeaderMap) -> Self {
        h.0
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

impl From<HttpVersion> for http::Version {
    fn from(h: HttpVersion) -> Self {
        h.0
    }
}

// impl from ref
impl From<&HeaderValue> for HttpHeaderValue {
    // clone should be totally fine bc the http lib uses the `Bytes` crate
    fn from(h: &HeaderValue) -> Self {
        Self(h.clone())
    }
}

impl<'a> From<&'a HeaderValue> for HttpHeaderValueRef<'a> {
    fn from(h: &'a HeaderValue) -> Self {
        Self(h)
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
