//! Middleware de headers HTTP de securite.
//!
//! @id: miyucloud_security_headers
//! @do: add_security_headers_to_api_and_web_responses
//! @role: security
//! @layer: app

use axum::body::Body;
use axum::http::header::{
    HeaderName, HeaderValue, CACHE_CONTROL, CONTENT_SECURITY_POLICY, PRAGMA, REFERRER_POLICY,
    X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
};
use axum::http::Request;
use axum::response::Response;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

static PERMISSIONS_POLICY: HeaderName = HeaderName::from_static("permissions-policy");
static STRICT_TRANSPORT_SECURITY: HeaderName = HeaderName::from_static("strict-transport-security");

#[derive(Clone, Copy)]
enum SecurityProfile {
    Api,
    Web,
}

#[derive(Clone)]
pub struct SecurityHeadersLayer {
    profile: SecurityProfile,
}

impl SecurityHeadersLayer {
    pub fn for_api() -> Self {
        Self {
            profile: SecurityProfile::Api,
        }
    }

    pub fn for_web() -> Self {
        Self {
            profile: SecurityProfile::Web,
        }
    }
}

impl<S> Layer<S> for SecurityHeadersLayer {
    type Service = SecurityHeadersMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SecurityHeadersMiddleware {
            inner,
            profile: self.profile,
        }
    }
}

#[derive(Clone)]
pub struct SecurityHeadersMiddleware<S> {
    inner: S,
    profile: SecurityProfile,
}

impl<S> Service<Request<Body>> for SecurityHeadersMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let profile = self.profile;

        Box::pin(async move {
            let mut response = inner.call(req).await?;
            let headers = response.headers_mut();

            headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
            headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
            headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
            headers.insert(
                PERMISSIONS_POLICY.clone(),
                HeaderValue::from_static("geolocation=(), camera=(), microphone=()"),
            );
            headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
            headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));

            if matches!(profile, SecurityProfile::Web) {
                headers.insert(
                    CONTENT_SECURITY_POLICY,
                    HeaderValue::from_static(
                        "default-src 'self'; base-uri 'none'; frame-ancestors 'none'; \
                         object-src 'none'; img-src 'self' data:; style-src 'self' 'unsafe-inline'; \
                         script-src 'self' 'unsafe-inline'; connect-src 'self'; form-action 'self'",
                    ),
                );
                headers.insert(
                    STRICT_TRANSPORT_SECURITY.clone(),
                    HeaderValue::from_static("max-age=31536000; includeSubDomains"),
                );
            }

            Ok(response)
        })
    }
}
