//! Middleware d'authentification interne via X-COG-Token.
//!
//! @id: miyucloud_api_auth
//! @do: verify_cog_token_on_internal_api_requests
//! @role: auth
//! @layer: app
//!
//! Toutes les routes `/api/*` exigent le header `X-COG-Token` avec une
//! valeur correspondant au token configure.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Header contenant le token COG.
pub const COG_TOKEN_HEADER: &str = "X-COG-Token";

/// Verifie un token en comparaison constant-time.
fn token_matches(provided: &str, expected: &str) -> bool {
    miyucloud::utils::constant_time::constant_time_eq(provided.as_bytes(), expected.as_bytes())
}

/// Layer pour verifier le token COG sur chaque requete.
#[derive(Clone)]
pub struct CogTokenLayer {
    expected_token: String,
}

impl CogTokenLayer {
    /// Cree un nouveau layer avec le token attendu.
    pub fn new(token: String) -> Self {
        Self {
            expected_token: token,
        }
    }
}

impl<S> Layer<S> for CogTokenLayer {
    type Service = CogTokenMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CogTokenMiddleware {
            inner,
            expected_token: self.expected_token.clone(),
        }
    }
}

/// Middleware qui verifie le header X-COG-Token.
#[derive(Clone)]
pub struct CogTokenMiddleware<S> {
    inner: S,
    expected_token: String,
}

impl<S> Service<Request<Body>> for CogTokenMiddleware<S>
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
        let token_header = req
            .headers()
            .get(COG_TOKEN_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let expected = self.expected_token.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            match token_header {
                Some(token) if token_matches(&token, &expected) => inner.call(req).await,
                _ => {
                    tracing::warn!(
                        path = %req.uri().path(),
                        "security: API request with missing or invalid X-COG-Token"
                    );
                    let response = error_response(
                        StatusCode::UNAUTHORIZED,
                        "UNAUTHORIZED",
                        "Missing or invalid X-COG-Token header",
                    );
                    Ok(response)
                }
            }
        })
    }
}

/// Construit une reponse d'erreur JSON standard.
pub fn error_response(status: StatusCode, code: &str, message: &str) -> Response {
    let body = serde_json::json!({
        "error": {
            "code": code,
            "message": message,
            "details": null
        }
    });
    (status, axum::Json(body)).into_response()
}

#[cfg(test)]
mod tests {
    use super::token_matches;

    #[test]
    fn test_token_matches_equal() {
        assert!(token_matches("abc123", "abc123"));
    }

    #[test]
    fn test_token_matches_different() {
        assert!(!token_matches("abc123", "abc124"));
        assert!(!token_matches("short", "much-longer-token"));
    }
}
