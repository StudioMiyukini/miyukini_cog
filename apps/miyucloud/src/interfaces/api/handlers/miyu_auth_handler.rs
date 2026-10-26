//! @id     miyucloud_interfaces_api_handlers_miyu_auth_handler
//! @do     exposer_defi_et_liaisons_miyu
//! @role   securite
//! @layer  app
//! @human  miyucloud — défis de possession, liaison d'une identité Miyukini (visa) et des clés d'appareil à un compte.

//! Points d'entrée de l'authentification Miyukini (lot 2, #18 ; I6) :
//!
//! Publics (débit limité par l'appelant) :
//!
//! - `POST /api/auth/miyu/defi` → `{defi_id, defi, audience, expire_a, cog_id}` :
//!   défi de 32 o, usage unique, 60 s, à signer par la clé d'appareil dans
//!   `X-Miyu-PoP` de la requête suivante ;
//! - `GET /api/auth/miyu/liaison-appareil` → `{cog_id, format, duree_max_secs}` :
//!   ce que l'appareil doit signer pour produire une demande de liaison `MLA2`.
//!
//! Session du navigateur **uniquement** (auth + CSRF + même origine) :
//!
//! - `POST /api/auth/miyu/liaisons` : lie l'identité prouvée par
//!   `X-Miyu-Visa: MiyuVisa <visa>.<passeport>` + `X-Miyu-PoP` (+
//!   `X-Miyu-Delegations`) — preuve liée à `POST /api/auth/miyu/liaisons` et au
//!   corps — au compte connecté. Libellé : `{"label": "…"}` ou `?label=…` ;
//! - `GET /api/auth/miyu/liaisons`, `DELETE /api/auth/miyu/liaisons/{id}` ;
//! - `GET /api/auth/miyu/appareils` : clés d'appareil actives du compte ;
//! - `POST /api/auth/miyu/appareils` `{"demande": "<MLA2>", "nom": "…", "portees": "webdav,api"}` :
//!   lie une nouvelle clé d'appareil (demande simple, possession prouvée) → 201 ;
//! - `DELETE /api/auth/miyu/appareils/{id}` : retire la clé ; toute requête
//!   signée par elle **et** tout visa portant cette clé d'appareil sont refusés
//!   dès la requête suivante (aucun cache) → 204.
//!
//! Requête **signée par l'ancienne clé d'appareil** uniquement :
//!
//! - `POST /api/auth/miyu/appareils/rotation` `{"demande": "<MLA2 rotation>"}` :
//!   la nouvelle clé remplace l'ancienne (même nom, mêmes portées), l'ancienne
//!   est révoquée dans la même transaction → 201.
//!
//! La liaison exige une session ouverte par mot de passe (ou OIDC) : un visa, une
//! clé d'appareil ou un mot de passe d'application ne peuvent pas lier une
//! nouvelle identité ni une nouvelle clé (la rotation remplace, elle n'ajoute pas).

use axum::{
    Extension, Json, Router,
    extract::{OriginalUri, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::infrastructure::services::miyu_auth_service::{
    AUDIENCE_MIYUCLOUD, CleAppareil, MiyuAuthService, MiyuIdentityStore, RefusMiyu, RequeteProuvee,
    maintenant,
};
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::{AuthMethod, SessionUser};

/// En-tête portant le visa à lier (l'en-tête `Authorization` reste libre).
pub const ENTETE_VISA_A_LIER: &str = "x-miyu-visa";

type Etat<S> = State<Arc<MiyuAuthService<S>>>;

/// Routes publiques (débit limité par l'appelant).
pub fn miyu_public_routes<S: MiyuIdentityStore>() -> Router<Arc<MiyuAuthService<S>>> {
    Router::new()
        .route("/defi", post(emettre_defi::<S>))
        .route("/liaison-appareil", get(contexte_liaison::<S>))
}

/// Routes protégées (auth, CSRF et même origine appliqués par l'appelant).
pub fn miyu_protected_routes<S: MiyuIdentityStore>() -> Router<Arc<MiyuAuthService<S>>> {
    Router::new()
        .route("/liaisons", post(lier::<S>).get(lister::<S>))
        .route("/liaisons/{id}", delete(delier::<S>))
        .route(
            "/appareils",
            post(lier_appareil::<S>).get(lister_appareils::<S>),
        )
        .route("/appareils/rotation", post(faire_tourner::<S>))
        .route("/appareils/{id}", delete(retirer_appareil::<S>))
}

fn refus(r: RefusMiyu) -> AppError {
    let statut = StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED);
    if let RefusMiyu::Stockage(e) = &r {
        tracing::error!("Stockage d'identités : {}", e);
    }
    AppError::new(statut, r.to_string(), "MiyuAuthRefused")
}

async fn emettre_defi<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
) -> Result<impl IntoResponse, AppError> {
    let d = svc.emettre_defi(maintenant()).map_err(refus)?;
    Ok(Json(serde_json::json!({
        "defi_id": d.id_b64(),
        "defi": d.defi_b64(),
        "audience": AUDIENCE_MIYUCLOUD,
        "expire_a": d.expire_a,
        "cog_id": svc.cog_id(),
    })))
}

async fn contexte_liaison<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
) -> Result<impl IntoResponse, AppError> {
    let cog = svc
        .cog_id()
        .ok_or_else(|| refus(RefusMiyu::VisaDesactive))?;
    Ok(Json(serde_json::json!({
        "cog_id": cog,
        "format": "MLA2",
        "duree_max_secs": miyauth::DUREE_MAX_LIAISON_SECS,
    })))
}

#[derive(serde::Deserialize, Default)]
struct CorpsLiaison {
    #[serde(default)]
    label: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct RequeteLibelle {
    #[serde(default)]
    label: Option<String>,
}

async fn lier<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    auth_user: SessionUser,
    OriginalUri(uri): OriginalUri,
    Query(requete): Query<RequeteLibelle>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, AppError> {
    let entete = |nom: &str| headers.get(nom).and_then(|v| v.to_str().ok());
    let identite = svc
        .verifier_visa_avec_delegations(
            RequeteProuvee {
                methode: "POST",
                chemin: uri.path(),
                chemin_et_requete: uri.path(),
                corps: &body,
            },
            entete(ENTETE_VISA_A_LIER),
            entete(miyauth::http::ENTETE_POP),
            entete(miyauth::http::ENTETE_DELEGATIONS),
            maintenant(),
        )
        .map_err(refus)?;
    // Une identité dont la clé d'appareil a été retirée ne se relie pas.
    if svc
        .store()
        .device_key_revoked(&identite.cle_appareil)
        .await
        .map_err(|e| refus(RefusMiyu::Stockage(e)))?
    {
        return Err(refus(RefusMiyu::AppareilRetire));
    }
    let corps: CorpsLiaison = if body.is_empty() {
        CorpsLiaison::default()
    } else {
        serde_json::from_slice(&body).map_err(|_| AppError::bad_request("Invalid JSON body"))?
    };
    let label: String = corps
        .label
        .or(requete.label)
        .unwrap_or_else(|| "Identité Miyukini".to_string())
        .chars()
        .filter(|c| !c.is_control())
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

async fn lister<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    auth_user: SessionUser,
) -> Result<impl IntoResponse, AppError> {
    let liens = svc.store().list_identities(auth_user.id).await?;
    Ok(Json(liens))
}

fn analyser_id(id: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(id).map_err(|_| AppError::bad_request("Invalid id"))
}

async fn delier<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    auth_user: SessionUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let id = analyser_id(&id)?;
    if svc.store().unlink_identity(auth_user.id, id).await? {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("Identity link not found"))
    }
}

async fn lister_appareils<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    auth_user: SessionUser,
) -> Result<impl IntoResponse, AppError> {
    let cles = svc.store().list_device_keys(auth_user.id).await?;
    Ok(Json(cles))
}

#[derive(serde::Deserialize)]
struct CorpsLiaisonAppareil {
    demande: String,
    #[serde(default)]
    nom: String,
    portees: String,
}

async fn lier_appareil<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    auth_user: SessionUser,
    Json(corps): Json<CorpsLiaisonAppareil>,
) -> Result<impl IntoResponse, AppError> {
    let cle = svc
        .lier_appareil(
            auth_user.id,
            &corps.demande,
            &corps.nom,
            &corps.portees,
            maintenant(),
        )
        .await
        .map_err(|r| {
            tracing::warn!(user = %auth_user.username, "Liaison d'appareil refusée : {}", r);
            refus(r)
        })?;
    tracing::info!(user = %auth_user.username, appareil = %cle.id, "Clé d'appareil liée au compte");
    Ok((StatusCode::CREATED, Json(cle)))
}

async fn retirer_appareil<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    auth_user: SessionUser,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let id = analyser_id(&id)?;
    if svc.store().revoke_device_key(auth_user.id, id).await? {
        tracing::info!(user = %auth_user.username, appareil = %id, "Clé d'appareil retirée du compte");
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found("Device key not found"))
    }
}

#[derive(serde::Deserialize)]
struct CorpsRotation {
    demande: String,
}

async fn faire_tourner<S: MiyuIdentityStore>(
    State(svc): Etat<S>,
    methode: Option<Extension<AuthMethod>>,
    signataire: Option<Extension<Arc<CleAppareil>>>,
    Json(corps): Json<CorpsRotation>,
) -> Result<impl IntoResponse, AppError> {
    // Seule une requête signée par la clé à remplacer peut la faire tourner.
    let (Some(Extension(AuthMethod::Appareil)), Some(Extension(signataire))) =
        (methode, signataire)
    else {
        return Err(AppError::forbidden(
            "Rotation requires a request signed by the device key being replaced",
        ));
    };
    let cle = svc
        .faire_tourner_appareil(&signataire, &corps.demande, maintenant())
        .await
        .map_err(|r| {
            tracing::warn!(appareil = %signataire.id, "Rotation de clé d'appareil refusée : {}", r);
            refus(r)
        })?;
    tracing::info!(
        ancienne = %signataire.id,
        nouvelle = %cle.id,
        "Clé d'appareil remplacée par rotation (ancienne révoquée)"
    );
    Ok((StatusCode::CREATED, Json(cle)))
}

#[cfg(test)]
mod tests;
