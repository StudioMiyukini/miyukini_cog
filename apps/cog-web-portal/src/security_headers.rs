//! Middleware de headers HTTP de sécurité — COG Web Portal.
//!
//! @id: cog_web_portal_security_headers @do: add_security_headers_csp_hsts
//! @role: security @layer: app
//! @human: Middleware sécurité Portal — CSP nonce per-request, HSTS, X-Frame-Options, Permissions-Policy.
//!
//! Pattern copié de apps/miyucloud/src/security_headers.rs (même codebase).

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
use uuid::Uuid;

/// CSP nonce injecté par requête — accessible via `Extension<CspNonce>` dans les handlers.
#[derive(Clone)]
pub struct CspNonce(pub String);

static PERMISSIONS_POLICY: HeaderName = HeaderName::from_static("permissions-policy");
static STRICT_TRANSPORT_SECURITY: HeaderName =
    HeaderName::from_static("strict-transport-security");

/// Layer de headers de sécurité pour les routes web du Portal.
#[derive(Clone, Copy)]
pub struct SecurityHeadersLayer;

impl<S> Layer<S> for SecurityHeadersLayer {
    type Service = SecurityHeadersMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SecurityHeadersMiddleware { inner }
    }
}

/// Middleware ajoutant CSP nonce, HSTS et headers de sécurité sur chaque réponse.
#[derive(Clone)]
pub struct SecurityHeadersMiddleware<S> {
    inner: S,
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

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();

        // Nonce CSP per-request (UUID v4 sans tirets).
        let nonce = Uuid::new_v4().simple().to_string();
        req.extensions_mut().insert(CspNonce(nonce.clone()));

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

            // connect-src élargi pour CentralRemote (WS + HTTP vers le serveur remote).
            let connect_src = match std::env::var("PORTAL_CENTRAL_REMOTE_ADDR") {
                Ok(addr) => format!("connect-src 'self' http://{addr} ws://{addr}", addr = addr),
                Err(_) => "connect-src 'self'".to_string(),
            };
            let csp = format!(
                "default-src 'self'; base-uri 'none'; frame-ancestors 'none'; \
                 object-src 'none'; img-src 'self' data:; \
                 style-src 'self' 'nonce-{nonce}'; \
                 script-src 'self' 'nonce-{nonce}'; \
                 {connect_src}; form-action 'self'"
            );
            if let Ok(value) = HeaderValue::from_str(&csp) {
                headers.insert(CONTENT_SECURITY_POLICY, value);
            }
            headers.insert(
                STRICT_TRANSPORT_SECURITY.clone(),
                HeaderValue::from_static("max-age=31536000; includeSubDomains"),
            );

            Ok(response)
        })
    }
}
