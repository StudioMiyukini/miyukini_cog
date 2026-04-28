//! Routes CentralRemote — contrôle à distance de Central depuis le portail web.
//!
//! @id: cog_web_portal_route_central_remote @do: serve_central_remote_ui
//! @role: api @layer: app
//! @human: Routes /central-remote : page de login, UI remote, proxy WebSocket.

use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use crate::AppState;
use crate::security_headers::CspNonce;
use crate::templates::{render_central_remote_login, render_central_remote_ui, render_error};

/// GET /central-remote — page de connexion CentralRemote.
/// Si Central n'a pas activé le remote, affiche un message d'information.
pub async fn central_remote_login(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> Html<String> {
    let remote_addr = state.central_remote_addr.as_deref();

    match remote_addr {
        Some(addr) => {
            // Vérifie que le serveur remote Central est joignable.
            let status_url = format!("http://{addr}/status");
            let reachable = reqwest::Client::new()
                .get(&status_url)
                .timeout(std::time::Duration::from_secs(3))
                .send()
                .await
                .is_ok();

            if reachable {
                Html(render_central_remote_login(addr, &nonce))
            } else {
                Html(render_error(
                    503,
                    "Central n'est pas joignable. Vérifiez que Central est lancé et que le remote est activé.",
                    &nonce,
                ))
            }
        }
        None => Html(render_error(
            503,
            "Le remote Central n'est pas configuré sur ce portail. Configurez PORTAL_CENTRAL_REMOTE_ADDR.",
            &nonce,
        )),
    }
}

/// GET /central-remote/app — interface de contrôle à distance (nécessite un token JS-side).
pub async fn central_remote_app(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> Html<String> {
    match state.central_remote_addr.as_deref() {
        Some(addr) => Html(render_central_remote_ui(addr, &nonce)),
        None => Html(render_error(
            503,
            "Le remote Central n'est pas configuré.",
            &nonce,
        )),
    }
}
