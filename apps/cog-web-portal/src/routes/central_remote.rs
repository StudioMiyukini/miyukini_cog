//! Routes CentralRemote — contrôle à distance de Central depuis le portail web.
//!
//! @id     cog_web_portal_route_central_remote
//! @do serve_central_remote_ui
//! @role   api
//! @layer app
//! @human  Routes /central-remote : pilotage par navigateur retiré (sortie du lot 2), message explicatif.

//! Sortie du lot 2 : CentralRemote n'authentifie plus par courriel + mot de
//! passe ni par jeton au porteur (`/auth`, `/ws?token=` retirés). Seuls les
//! appareils appairés (passeport d'appareil + visa + preuve de possession) sont
//! acceptés ; un navigateur sans clé d'appareil ne peut pas piloter Central.
//! Les routes restent pour expliquer la situation plutôt que d'échouer en silence.

use axum::extract::State;
use axum::response::Html;
use axum::Extension;

use crate::security_headers::CspNonce;
use crate::templates::render_error;
use crate::AppState;

/// Message affiché à la place du pilotage par navigateur.
pub const MESSAGE_PILOTAGE_RETIRE: &str = "Le pilotage de Central depuis le navigateur est retiré : \
     CentralRemote n'accepte plus que des appareils appairés (passeport d'appareil). \
     Utilisez Central Mobile, appairé depuis Central Desktop.";

/// GET /central-remote — information : pilotage par navigateur retiré.
pub async fn central_remote_login(
    State(state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> Html<String> {
    match state.central_remote_addr.as_deref() {
        Some(_) => Html(render_error(410, MESSAGE_PILOTAGE_RETIRE, &nonce)),
        None => Html(render_error(
            503,
            "Le remote Central n'est pas configuré sur ce portail. Configurez PORTAL_CENTRAL_REMOTE_ADDR.",
            &nonce,
        )),
    }
}

/// GET /central-remote/app — information : pilotage par navigateur retiré.
pub async fn central_remote_app(
    State(_state): State<AppState>,
    Extension(CspNonce(nonce)): Extension<CspNonce>,
) -> Html<String> {
    Html(render_error(410, MESSAGE_PILOTAGE_RETIRE, &nonce))
}
