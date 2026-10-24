//! @id     miyucloud_interfaces_api_handlers_miyu_auth_handler
//! @do     exposer_defi_et_liaisons_miyu
//! @role   securite
//! @layer  app
//! @human  miyucloud — défis de possession et liaison d'une identité Miyukini (visa) à un compte.

//! Points d'entrée de l'authentification Miyukini (lot 2, #18) :
//!
//! - `POST /api/auth/miyu/defi` (public, débit limité) → `{defi_id, defi, audience,
//!   expire_a, cog_id}` : défi de 32 o, usage unique, 60 s, à signer par la clé
//!   d'appareil dans `X-Miyu-PoP` de la requête suivante ;
//! - `POST /api/auth/miyu/liaisons` (session du navigateur **uniquement**, CSRF) :
//!   lie l'identité prouvée par `X-Miyu-Visa: MiyuVisa <visa>.<passeport>` +
//!   `X-Miyu-PoP` (preuve liée à `POST /api/auth/miyu/liaisons` et au corps) au
//!   compte connecté. Corps optionnel `{"label": "…"}` ;
//! - `GET /api/auth/miyu/liaisons`, `DELETE /api/auth/miyu/liaisons/{id}`.
//!
//! La liaison exige une session ouverte par mot de passe (ou OIDC) : un visa, une
//! clé d'appareil ou un mot de passe d'application ne peuvent pas lier une
//! nouvelle identité.

use axum::{
    Json, Router,
    extract::{OriginalUri, Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, post},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::di::{AppState, MiyuAuth};
use crate::infrastructure::services::miyu_auth_service::{
    AUDIENCE_MIYUCLOUD, MiyuIdentityStore, RefusMiyu, RequeteProuvee, maintenant,
};
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::SessionUser;

/// En-tête portant le visa à lier (l'en-tête `Authorization` reste libre).
pub const ENTETE_VISA_A_LIER: &str = "x-miyu-visa";

/// Route publique (débit limité par l'appelant).
pub fn miyu_public_routes() -> Router<Arc<AppState>> {
    Router::new().route("/defi", post(emettre_defi))
}

/// Routes protégées (auth + CSRF appliqués par l'appelant).
pub fn miyu_protected_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/liaisons", post(lier).get(lister))
        .route("/liaisons/{id}", delete(delier))
}

fn service(state: &AppState) -> Result<&Arc<MiyuAuth>, AppError> {
    state
        .miyu_auth
        .as_ref()
        .ok_or_else(|| AppError::internal_error("Miyukini authentication not configured"))
}

fn refus(r: RefusMiyu) -> AppError {
    let statut = StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED);
    AppError::new(statut, r.to_string(), "MiyuAuthRefused")
}

async fn emettre_defi(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, AppError> {
    let svc = service(&state)?;
    let d = svc.emettre_defi(maintenant()).map_err(refus)?;
    Ok(Json(serde_json::json!({
        "defi_id": d.id_b64(),
        "defi": d.defi_b64(),
        "audience": AUDIENCE_MIYUCLOUD,
        "expire_a": d.expire_a,
        "cog_id": svc.cog_id(),
    })))
}

#[derive(serde::Deserialize, Default)]
struct CorpsLiaison {
    #[serde(default)]
    label: Option<String>,
}

async fn lier(
    State(state): State<Arc<AppState>>,
    auth_user: SessionUser,
    OriginalUri(uri): OriginalUri,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, AppError> {
    let svc = service(&state)?;
    let entete = |nom: &str| headers.get(nom).and_then(|v| v.to_str().ok());
    let identite = svc
        .verifier_visa(
            RequeteProuvee {
                methode: "POST",
                chemin: uri.path(),
                chemin_et_requete: uri.path(),
                corps: &body,
            },
            entete(ENTETE_VISA_A_LIER),
            entete(miyauth::http::ENTETE_POP),
            maintenant(),
        )
        .map_err(refus)?;
    let corps: CorpsLiaison = if body.is_empty() {
        CorpsLiaison::default()
    } else {
        serde_json::from_slice(&body).map_err(|_| AppError::bad_request("Invalid JSON body"))?
    };
    let label: String = corps
        .label
        .unwrap_or_else(|| "Identité Miyukini".to_string())
        .chars()
        .take(255)
        .collect();
    let lien = svc
        .store()
        .link_identity(auth_user.id, &identite.cog_id, &identite.sujet, &label)
        .await
        .map_err(|e| {
            tracing::warn!("Liaison d'identité refusée : {}", e);
            AppError::from(e)
        })?;
    tracing::info!(
        user = %auth_user.username,
        cog = %identite.cog_id,
        "Identité Miyukini liée au compte"
    );
    Ok((StatusCode::CREATED, Json(lien)))
}

async fn lister(
    State(state): State<Arc<AppState>>,
    auth_user: SessionUser,
) -> Result<impl IntoResponse, AppError> {
    let svc = service(&state)?;
    let liens = svc.store().list_identities(auth_user.id).await?;
    Ok(Json(liens))
}

async fn delier(
    State(state): State<Arc<AppState>>,
    auth_user: SessionUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let svc = service(&state)?;
    let id = Uuid::parse_str(&id).map_err(|_| AppError::bad_request("Invalid id"))?;
    if svc.store().unlink_identity(auth_user.id, id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("Identity link not found"))
    }
}
