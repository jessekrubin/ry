//! "new" types for the http crate
//!
//! Used for the py-conversions mod

use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct HttpHeaderName(pub http::HeaderName);
#[derive(Debug, Clone)]
pub struct HttpHeaderValue(pub http::HeaderValue);
#[derive(Debug, Clone)]
pub struct HttpMethod(pub http::Method);

#[derive(Debug, Clone)]
pub struct HttpStatusCode(pub http::StatusCode);

// ============================================================================
//  FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM
// ============================================================================

impl From<HttpHeaderName> for http::HeaderName {
    fn from(h: HttpHeaderName) -> Self {
        h.0
    }
}

impl From<HttpHeaderValue> for http::HeaderValue {
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
        HttpHeaderName(h)
    }
}

impl From<http::HeaderValue> for HttpHeaderValue {
    fn from(h: http::HeaderValue) -> Self {
        HttpHeaderValue(h)
    }
}

impl From<http::Method> for HttpMethod {
    fn from(h: http::Method) -> Self {
        HttpMethod(h)
    }
}

impl From<http::StatusCode> for HttpStatusCode {
    fn from(h: http::StatusCode) -> Self {
        HttpStatusCode(h)
    }
}

// impl from ref
impl From<&http::HeaderValue> for HttpHeaderValue {
    // clone should be totally fine bc the http lib uses the `Bytes` crate
    fn from(h: &http::HeaderValue) -> Self {
        HttpHeaderValue(h.clone())
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
    type Target = http::HeaderValue;
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
