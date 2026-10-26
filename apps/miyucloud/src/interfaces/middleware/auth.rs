//! @id     miyucloud_interfaces_middleware_auth
//! @do     definir_cookie_authenticated
//! @role   securite
//! @layer  app
//! @human  miyucloud — module « interfaces middleware auth ».

use axum::{
    extract::{FromRequestParts, Request, State},
    http::{HeaderMap, StatusCode, header, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::convert::Infallible;
use std::sync::Arc;
use uuid::Uuid;

use crate::common::di::AppState;

// Re-export CurrentUser from application layer for use in handlers
pub use crate::application::dtos::user_dto::CurrentUser;
use crate::application::ports::auth_ports::TokenServicePort;

/// Marker inserted into request extensions when the user was authenticated
/// via the `__Host-miyucloud_session` HttpOnly cookie rather than a header.
/// The CSRF middleware uses this to decide whether CSRF validation is required.
#[derive(Clone, Copy, Debug)]
pub struct CookieAuthenticated;

// Newtype over Arc<CurrentUser> for zero-allocation extraction.
// `Deref<Target = CurrentUser>` lets handlers access `.id`, `.username`,
// `.email`, `.role` transparently — no signature changes needed.
#[derive(Clone, Debug)]
pub struct AuthUser(pub Arc<CurrentUser>);

impl std::ops::Deref for AuthUser {
    type Target = CurrentUser;
    #[inline]
    fn deref(&self) -> &CurrentUser {
        &self.0
    }
}

/// Reusable extractor that gets the user_id of the authenticated user.
/// Automatically extracted from the `CurrentUser` inserted by the auth middleware.
///
/// Usage in handlers:
/// ```ignore
/// async fn my_handler(CurrentUserId(user_id): CurrentUserId) -> impl IntoResponse { ... }
/// ```
#[derive(Clone, Debug)]
pub struct CurrentUserId(pub Uuid);

// Implement FromRequestParts for AuthUser — allows using `auth_user: AuthUser` in handlers.
// Cost: 1 atomic increment (~1 ns) instead of 3 String clones (~100 ns + 3 mallocs).
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Arc<CurrentUser>>()
            .cloned()
            .map(AuthUser)
            .ok_or(AuthError::UserNotFound)
    }
}

// Implement FromRequestParts for CurrentUserId — lightweight extractor for user_id only
impl<S> FromRequestParts<S> for CurrentUserId
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Arc<CurrentUser>>()
            .map(|cu| CurrentUserId(cu.id))
            .ok_or(AuthError::UserNotFound)
    }
}

/// Optional user ID extractor – never fails.
/// Yields `Some(id)` when auth middleware ran, `None` otherwise.
#[derive(Clone, Debug)]
pub struct OptionalUserId(pub Option<Uuid>);

impl<S> FromRequestParts<S> for OptionalUserId
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(OptionalUserId(
            parts.extensions.get::<Arc<CurrentUser>>().map(|cu| cu.id),
        ))
    }
}

// Error for authentication operations
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Token not provided")]
    TokenNotProvided,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("User not found")]
    UserNotFound,

    #[error("Access denied: {0}")]
    AccessDenied(String),

    #[error("Authentication service unavailable")]
    AuthServiceUnavailable,

    #[error("Payload too large for a signed request")]
    PayloadTooLarge,

    #[error("Too many failed authentications")]
    TooManyFailures,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::TokenNotProvided => {
                (StatusCode::UNAUTHORIZED, "Token not provided".to_string())
            }
            AuthError::InvalidToken(msg) => (StatusCode::UNAUTHORIZED, msg),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired".to_string()),
            AuthError::UserNotFound => (StatusCode::UNAUTHORIZED, "User not found".to_string()),
            AuthError::AccessDenied(msg) => (StatusCode::FORBIDDEN, msg),
            AuthError::AuthServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Authentication service unavailable".to_string(),
            ),
            AuthError::PayloadTooLarge => (
                StatusCode::PAYLOAD_TOO_LARGE,
                "Body too large for a signed request; use chunked uploads".to_string(),
            ),
            AuthError::TooManyFailures => (
                StatusCode::TOO_MANY_REQUESTS,
                "Too many failed authentications".to_string(),
            ),
        };

        let body = axum::Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

/// Préfixes jamais accessibles par mot de passe d'application (M-05) :
/// gestion des secrets, MFA, sessions, administration, RGPD.
const BASIC_AUTH_FORBIDDEN_PREFIXES: &[&str] =
    &["/api/auth", "/api/admin", "/api/user/gdpr", "/api/setup"];

/// Vrai si la liste de portées `scopes` (séparées par des virgules) accorde
/// `scope`. `all` (mots de passe Nextcloud) accorde les portées DAV
/// (`webdav`, `caldav`, `carddav`) mais **pas** `api`.
pub fn scope_granted(scopes: &str, scope: &str) -> bool {
    scopes
        .split(',')
        .map(str::trim)
        .any(|s| s == scope || (s == "all" && matches!(scope, "webdav" | "caldav" | "carddav")))
}

/// M-05 : un mot de passe d'application de portées `scopes` peut-il servir
/// sur `path` ? Fail-closed : tout chemin non listé est refusé.
pub fn app_password_allows(path: &str, scopes: &str) -> bool {
    let under = |prefix: &str| {
        path == prefix
            || path
                .strip_prefix(prefix)
                .is_some_and(|rest| rest.starts_with('/'))
    };
    if BASIC_AUTH_FORBIDDEN_PREFIXES.iter().any(|p| under(p)) {
        return false;
    }
    if under("/webdav") || under("/remote.php") {
        return scope_granted(scopes, "webdav");
    }
    if under("/caldav") || under("/.well-known/caldav") {
        return scope_granted(scopes, "caldav");
    }
    if under("/carddav") || under("/.well-known/carddav") {
        return scope_granted(scopes, "carddav");
    }
    if under("/api") {
        return scope_granted(scopes, "api");
    }
    false
}

/// Rotation d'une clé d'appareil par elle-même (requête signée par l'ancienne clé).
pub const CHEMIN_ROTATION_APPAREIL: &str = "/api/auth/miyu/appareils/rotation";

/// Méthode par laquelle la requête a été authentifiée (posée dans les
/// extensions par [`auth_middleware`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthMethod {
    /// Session opaque (cookie du navigateur ou `Bearer`).
    Session,
    /// Visa + preuve de possession (`MiyuVisa`).
    Visa,
    /// Requête signée par une clé d'appareil enregistrée.
    Appareil,
    /// Basic + mot de passe d'application (exception DAV).
    MotDePasseApplication,
}

/// Chemin complet (hors imbrication des routeurs) et chemin + requête.
fn chemins(request: &Request) -> (String, String) {
    let uri = request
        .extensions()
        .get::<axum::extract::OriginalUri>()
        .map(|u| u.0.clone())
        .unwrap_or_else(|| request.uri().clone());
    let pq = uri
        .path_and_query()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| uri.path().to_string());
    (uri.path().to_string(), pq)
}

fn entete<'a>(headers: &'a HeaderMap, nom: &str) -> Option<&'a str> {
    headers.get(nom).and_then(|v| v.to_str().ok())
}

/// Lit le corps (borné) pour vérifier une preuve liée au corps, et rend la
/// requête reconstruite.
async fn lire_corps_borne(request: Request) -> Result<(Request, bytes::Bytes), AuthError> {
    use crate::infrastructure::services::miyu_auth_service::TAILLE_MAX_CORPS_PROUVE;
    let annonce = request
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());
    if annonce.is_some_and(|n| n > TAILLE_MAX_CORPS_PROUVE as u64) {
        return Err(AuthError::PayloadTooLarge);
    }
    let (parts, body) = request.into_parts();
    let octets = axum::body::to_bytes(body, TAILLE_MAX_CORPS_PROUVE)
        .await
        .map_err(|_| AuthError::PayloadTooLarge)?;
    let request = Request::from_parts(parts, axum::body::Body::from(octets.clone()));
    Ok((request, octets))
}

/// Compte actif → `CurrentUser` (relu en base : rôle et statut à jour).
async fn utilisateur_actif(state: &AppState, user_id: Uuid) -> Result<CurrentUser, AuthError> {
    let auth = state
        .auth_service
        .as_ref()
        .ok_or(AuthError::AuthServiceUnavailable)?;
    let u = auth
        .auth_application_service
        .get_user_by_id(user_id)
        .await
        .map_err(|_| AuthError::UserNotFound)?;
    if !u.active {
        return Err(AuthError::AccessDenied("Account deactivated".to_string()));
    }
    Ok(CurrentUser {
        id: user_id,
        username: u.username,
        email: u.email,
        role: u.role,
    })
}

fn refus_miyu(r: crate::infrastructure::services::miyu_auth_service::RefusMiyu) -> AuthError {
    tracing::warn!("Authentification Miyukini refusée : {}", r);
    match r.statut() {
        503 => AuthError::AuthServiceUnavailable,
        _ => AuthError::InvalidToken(r.to_string()),
    }
}

/// Authentification (fail-closed), dans l'ordre :
///
/// 1. **`MiyuVisa` + `X-Miyu-PoP`** — clients natifs : visa du COG d'origine
///    épinglé, défi à usage unique, preuve liée à la requête (méthode, chemin,
///    corps), identité liée à un compte ;
/// 2. **requête signée par clé d'appareil** (`X-Miyu-Signature`…) — appareils
///    enrôlés par le flux RFC 8628 ; portées appliquées ;
/// 3. **`Bearer`** — identifiant de session opaque (côté serveur, révocable) ;
/// 4. **`Basic` + mot de passe d'application** — exception DAV : TLS obligatoire,
///    portées, échecs limités par IP ;
/// 5. **cookie `__Host-miyucloud_session`** — navigateur (CSRF exigé ensuite).
///
/// Un schéma présent mais invalide est refusé : jamais de repli sur le suivant.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    use crate::infrastructure::services::miyu_auth_service::{RequeteProuvee, maintenant};
    use crate::interfaces::middleware::basic_policy;

    let auth_header = entete(&headers, header::AUTHORIZATION.as_str());

    // ── 1. Visa + preuve de possession ───────────────────────────
    if let Some(valeur) = auth_header
        && valeur.trim_start().starts_with("MiyuVisa ")
    {
        let miyu = state.miyu_auth.as_ref().ok_or_else(|| {
            AuthError::InvalidToken("MiyuVisa is not enabled on this server".to_string())
        })?;
        // Titres vérifiés (signatures, ancre, audience, révocation) avant de
        // lire le moindre octet du corps : un anonyme ne fait pas lire 16 Mio.
        let delegations = entete(&headers, miyauth::http::ENTETE_DELEGATIONS);
        miyu.precontroler_visa(
            Some(valeur),
            entete(&headers, miyauth::http::ENTETE_POP),
            delegations,
            maintenant(),
        )
        .map_err(refus_miyu)?;
        let (chemin, chemin_et_requete) = chemins(&request);
        let methode = request.method().as_str().to_string();
        let (req, corps) = lire_corps_borne(request).await?;
        request = req;
        let (user_id, portees) = miyu
            .authentifier_visa(
                RequeteProuvee {
                    methode: &methode,
                    chemin: &chemin,
                    chemin_et_requete: &chemin_et_requete,
                    corps: &corps,
                },
                Some(valeur),
                entete(&headers, miyauth::http::ENTETE_POP),
                delegations,
                maintenant(),
            )
            .await
            .map_err(refus_miyu)?;
        // Portées du visa (fail-closed : portée inconnue ⇒ rien ; jamais la
        // gestion des secrets ni l'administration).
        if !app_password_allows(&chemin, &portees) {
            tracing::warn!("Visa scopes ({}) do not allow {}", portees, chemin);
            return Err(AuthError::AccessDenied(
                "Visa scope does not allow this endpoint".to_string(),
            ));
        }
        let user = utilisateur_actif(&state, user_id).await?;
        request.extensions_mut().insert(Arc::new(user));
        request.extensions_mut().insert(AuthMethod::Visa);
        return Ok(next.run(request).await);
    }

    // ── 2. Requête signée par clé d'appareil ─────────────────────
    if headers.contains_key(miyauth::http::ENTETE_SIGNATURE)
        || headers.contains_key(miyauth::http::ENTETE_SIGNATAIRE)
    {
        let miyu = state
            .miyu_auth
            .as_ref()
            .ok_or(AuthError::AuthServiceUnavailable)?;
        let t = maintenant();
        let (entetes, cle) = miyu
            .preparer_appareil(
                entete(&headers, miyauth::http::ENTETE_SIGNATAIRE),
                entete(&headers, miyauth::http::ENTETE_HORODATAGE),
                entete(&headers, miyauth::http::ENTETE_NONCE),
                entete(&headers, miyauth::http::ENTETE_SIGNATURE),
                t,
            )
            .await
            .map_err(refus_miyu)?;
        let (chemin, chemin_et_requete) = chemins(&request);
        // Portées de l'appareil (mêmes règles que les mots de passe d'application) ;
        // seule exception : une clé peut toujours se faire remplacer elle-même
        // (rotation autorisée par sa propre signature, I6).
        let rotation =
            request.method() == axum::http::Method::POST && chemin == CHEMIN_ROTATION_APPAREIL;
        if !rotation && !app_password_allows(&chemin, &cle.scopes) {
            return Err(AuthError::AccessDenied(
                "Device key scope does not allow this endpoint".to_string(),
            ));
        }
        let methode = request.method().as_str().to_string();
        let (req, corps) = lire_corps_borne(request).await?;
        request = req;
        miyu.verifier_appareil(
            &entetes,
            RequeteProuvee {
                methode: &methode,
                chemin: &chemin,
                chemin_et_requete: &chemin_et_requete,
                corps: &corps,
            },
            t,
        )
        .map_err(refus_miyu)?;
        let user = utilisateur_actif(&state, cle.user_id).await?;
        request.extensions_mut().insert(Arc::new(user));
        request.extensions_mut().insert(AuthMethod::Appareil);
        // Clé signataire, pour la rotation (seule route qui la lit).
        request.extensions_mut().insert(Arc::new(cle));
        return Ok(next.run(request).await);
    }

    if let Some(header_value) = auth_header {
        // ── 3. Bearer : identifiant de session opaque ────────────
        if let Some(token_str) = header_value.strip_prefix("Bearer ") {
            let token_str = token_str.trim();
            if !token_str.is_empty() {
                let auth_service = state
                    .auth_service
                    .as_ref()
                    .ok_or(AuthError::AuthServiceUnavailable)?;
                let claims = auth_service
                    .token_service
                    .validate_token(token_str)
                    .map_err(|e| {
                        tracing::warn!("Bearer session refused: {}", e);
                        AuthError::InvalidToken("Invalid or expired session".to_string())
                    })?;
                let user_id = Uuid::parse_str(&claims.sub)
                    .map_err(|_| AuthError::InvalidToken("Invalid session".to_string()))?;
                request.extensions_mut().insert(Arc::new(CurrentUser {
                    id: user_id,
                    username: claims.username,
                    email: claims.email,
                    role: claims.role,
                }));
                request.extensions_mut().insert(AuthMethod::Session);
                return Ok(next.run(request).await);
            }
        }

        // ── 4. Basic + mot de passe d'application (exception DAV) ─
        if let Some(basic_encoded) = header_value.strip_prefix("Basic ") {
            let basic_encoded = basic_encoded.trim();
            if !basic_encoded.is_empty() {
                // #19 : TLS obligatoire (sauf boucle locale).
                let peer = request
                    .extensions()
                    .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                    .map(|c| c.0.ip());
                if !basic_policy::transport_securise(
                    state.core.config.tls.enabled,
                    peer,
                    entete(&headers, "x-forwarded-proto"),
                    crate::interfaces::middleware::rate_limit::trusted_proxies(),
                ) {
                    tracing::warn!("Basic auth refused over plain HTTP (peer {:?})", peer);
                    return Err(AuthError::AccessDenied(
                        "Basic authentication requires HTTPS".to_string(),
                    ));
                }
                let ip = crate::interfaces::middleware::rate_limit::extract_client_ip(&request);
                if basic_policy::echecs_basic().bloque(&ip) {
                    tracing::warn!(ip = %ip, "Basic auth: too many failures from this IP");
                    return Err(AuthError::TooManyFailures);
                }

                use base64::Engine;
                let decoded = base64::engine::general_purpose::STANDARD
                    .decode(basic_encoded)
                    .map_err(|_| {
                        AuthError::InvalidToken("Invalid Basic auth encoding".to_string())
                    })?;
                let credentials = String::from_utf8(decoded).map_err(|_| {
                    AuthError::InvalidToken("Invalid Basic auth encoding".to_string())
                })?;
                let (username, password) = credentials.split_once(':').ok_or_else(|| {
                    AuthError::InvalidToken("Invalid Basic auth format".to_string())
                })?;

                let (path, _) = chemins(&request);

                let app_pw_service = state.app_password_service.as_ref().ok_or_else(|| {
                    tracing::warn!("Basic auth attempted but app password service not configured");
                    AuthError::InvalidToken("App passwords are not enabled".to_string())
                })?;
                let identity = match app_pw_service.verify_basic_auth(username, password).await {
                    Ok(identity) => identity,
                    Err(e) => {
                        basic_policy::echecs_basic().noter_echec(&ip);
                        tracing::warn!("App password verification failed: {}", e);
                        return Err(AuthError::InvalidToken(
                            "Invalid username or app password".to_string(),
                        ));
                    }
                };
                // M-05 : la portée du mot de passe d'application est appliquée ;
                // gestion des secrets jamais en Basic.
                if !app_password_allows(&path, &identity.scopes) {
                    tracing::warn!(
                        "App password of '{}' used outside its scopes ({}) on {}",
                        identity.username,
                        identity.scopes,
                        path
                    );
                    return Err(AuthError::AccessDenied(
                        "App password scope does not allow this endpoint".to_string(),
                    ));
                }
                request.extensions_mut().insert(Arc::new(CurrentUser {
                    id: identity.user_id,
                    username: identity.username,
                    email: identity.email,
                    role: identity.role,
                }));
                request
                    .extensions_mut()
                    .insert(AuthMethod::MotDePasseApplication);
                return Ok(next.run(request).await);
            }
        }
    }

    // ── 5. Cookie de session (navigateur) ────────────────────────
    if let Some(token_str) = crate::interfaces::api::cookie_auth::extract_session_cookie(&headers)
        && let Some(auth_service) = state.auth_service.as_ref()
    {
        match auth_service.token_service.validate_token(&token_str) {
            Ok(claims) => {
                let user_id = Uuid::parse_str(&claims.sub)
                    .map_err(|_| AuthError::InvalidToken("Invalid session".to_string()))?;
                request.extensions_mut().insert(Arc::new(CurrentUser {
                    id: user_id,
                    username: claims.username,
                    email: claims.email,
                    role: claims.role,
                }));
                request.extensions_mut().insert(CookieAuthenticated);
                request.extensions_mut().insert(AuthMethod::Session);
                return Ok(next.run(request).await);
            }
            Err(e) => {
                // 401 ci-dessous : le navigateur rafraîchit ou retourne à /login.
                tracing::debug!("Session cookie refused: {}", e);
            }
        }
    }

    // No valid credentials found via any method
    if state.auth_service.is_none() {
        tracing::error!("Auth middleware invoked but auth service is not configured");
        return Err(AuthError::AuthServiceUnavailable);
    }

    Err(AuthError::TokenNotProvided)
}

/// Vrai si la requête a été authentifiée par une session interactive
/// (mot de passe / OIDC, cookie ou `Bearer` de session).
fn session_interactive(parts: &Parts) -> bool {
    parts.extensions.get::<AuthMethod>() == Some(&AuthMethod::Session)
}

/// Extracteur : utilisateur authentifié par une **session interactive**.
///
/// Exigé pour la gestion des titres (mots de passe d'application, clés
/// d'appareil, approbation d'un appareil, liaison d'identité) : un visa, une clé
/// d'appareil ou un mot de passe d'application ne peuvent pas créer ni révoquer
/// d'autres titres.
#[derive(Clone, Debug)]
pub struct SessionUser(pub Arc<CurrentUser>);

impl std::ops::Deref for SessionUser {
    type Target = CurrentUser;
    fn deref(&self) -> &CurrentUser {
        &self.0
    }
}

impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<Arc<CurrentUser>>()
            .cloned()
            .ok_or(AuthError::TokenNotProvided)?;
        if !session_interactive(parts) {
            tracing::warn!(
                "Interactive session required for user {} (method {:?})",
                user.username,
                parts.extensions.get::<AuthMethod>()
            );
            return Err(AuthError::AccessDenied(
                "An interactive session (password/OIDC) is required".to_string(),
            ));
        }
        Ok(SessionUser(user))
    }
}

/// Extracteur : **administrateur** authentifié par une session interactive
/// (visa, clé d'appareil et mot de passe d'application refusés).
#[derive(Clone, Debug)]
pub struct AdminUser(pub Arc<CurrentUser>);

impl AdminUser {
    /// `(id, rôle)` de l'administrateur.
    pub fn ids(&self) -> (Uuid, String) {
        (self.0.id, self.0.role.clone())
    }
}

impl<S> FromRequestParts<S> for AdminUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let SessionUser(user) = SessionUser::from_request_parts(parts, state).await?;
        if user.role != "admin" {
            tracing::warn!("Admin access denied for user: {}", user.username);
            return Err(AuthError::AccessDenied("Admin role required".to_string()));
        }
        Ok(AdminUser(user))
    }
}

/// Middleware to verify that the authenticated user has an admin role.
///
/// Must be applied AFTER auth_middleware, as it depends on
/// `CurrentUser` being present in the request extensions.
pub async fn require_admin(request: Request, next: Next) -> Response {
    // Get the CurrentUser inserted by auth_middleware
    let session = request.extensions().get::<AuthMethod>() == Some(&AuthMethod::Session);
    if let Some(current_user) = request.extensions().get::<Arc<CurrentUser>>() {
        if current_user.role == "admin" && session {
            tracing::debug!("Admin access granted for user: {}", current_user.username);
            return next.run(request).await;
        }
        tracing::warn!(
            "Admin access denied for user: {} (role: {})",
            current_user.username,
            current_user.role
        );
    } else {
        tracing::warn!("Admin check failed: no authenticated user in request");
    }

    // Access denied
    let error = AuthError::AccessDenied("Admin role required".to_string());
    error.into_response()
}

#[cfg(test)]
mod admin_extractor_tests {
    use super::*;

    async fn extraire<T: FromRequestParts<(), Rejection = AuthError>>(
        role: &str,
        methode: Option<AuthMethod>,
    ) -> Result<T, StatusCode> {
        let (mut parts, ()) = axum::http::Request::builder()
            .uri("/api/admin/users")
            .body(())
            .unwrap()
            .into_parts();
        parts.extensions.insert(Arc::new(CurrentUser {
            id: Uuid::new_v4(),
            username: "root".into(),
            email: "r@x".into(),
            role: role.into(),
        }));
        if let Some(m) = methode {
            parts.extensions.insert(m);
        }
        T::from_request_parts(&mut parts, &())
            .await
            .map_err(|e| e.into_response().status())
    }

    fn bloquer<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(f)
    }

    #[test]
    fn admin_requires_interactive_session() {
        assert!(bloquer(extraire::<AdminUser>("admin", Some(AuthMethod::Session))).is_ok());
        for m in [
            AuthMethod::Visa,
            AuthMethod::Appareil,
            AuthMethod::MotDePasseApplication,
        ] {
            assert_eq!(
                bloquer(extraire::<AdminUser>("admin", Some(m))).err(),
                Some(StatusCode::FORBIDDEN),
                "{m:?}"
            );
        }
        assert!(bloquer(extraire::<AdminUser>("admin", None)).is_err());
        assert_eq!(
            bloquer(extraire::<AdminUser>("user", Some(AuthMethod::Session))).err(),
            Some(StatusCode::FORBIDDEN)
        );
    }

    #[test]
    fn credential_management_requires_interactive_session() {
        assert!(bloquer(extraire::<SessionUser>("user", Some(AuthMethod::Session))).is_ok());
        for m in [
            AuthMethod::Visa,
            AuthMethod::Appareil,
            AuthMethod::MotDePasseApplication,
        ] {
            assert_eq!(
                bloquer(extraire::<SessionUser>("user", Some(m))).err(),
                Some(StatusCode::FORBIDDEN)
            );
        }
    }
}

#[cfg(test)]
mod app_password_scope_tests {
    use super::*;

    const DAV_DEFAULT: &str = "webdav,caldav,carddav";

    #[test]
    fn dav_scopes_reach_their_protocols() {
        assert!(app_password_allows("/webdav/My Folder - a/x", DAV_DEFAULT));
        assert!(app_password_allows("/caldav/cal-1", DAV_DEFAULT));
        assert!(app_password_allows("/carddav/ab", DAV_DEFAULT));
        assert!(app_password_allows("/caldav", "caldav"));
    }

    #[test]
    fn scope_is_enforced_per_protocol() {
        assert!(!app_password_allows("/webdav/x", "caldav"));
        assert!(!app_password_allows("/caldav/x", "webdav"));
        assert!(!app_password_allows("/carddav/x", "webdav,caldav"));
    }

    #[test]
    fn dav_only_password_cannot_use_rest_api() {
        assert!(!app_password_allows("/api/files/upload", DAV_DEFAULT));
        assert!(!app_password_allows("/api/shares", "all"));
        assert!(app_password_allows("/api/files/upload", "api"));
    }

    #[test]
    fn secret_management_is_never_allowed_with_basic_auth() {
        for scopes in [DAV_DEFAULT, "all", "api", "api,webdav,caldav,carddav,all"] {
            assert!(!app_password_allows("/api/auth/app-passwords", scopes));
            assert!(!app_password_allows("/api/auth/mfa/setup", scopes));
            assert!(!app_password_allows("/api/admin/users", scopes));
            assert!(!app_password_allows("/api/user/gdpr/export", scopes));
        }
    }

    #[test]
    fn unknown_paths_and_prefix_tricks_are_refused() {
        assert!(!app_password_allows("/webdavx/a", DAV_DEFAULT));
        assert!(!app_password_allows("/other", "all"));
        assert!(!app_password_allows("/caldav/x", ""));
    }

    #[test]
    fn all_scope_covers_dav_only() {
        assert!(scope_granted("all", "webdav"));
        assert!(scope_granted("all", "carddav"));
        assert!(!scope_granted("all", "api"));
        assert!(scope_granted(" webdav , api ", "api"));
        assert!(!scope_granted("webdavs", "webdav"));
    }
}
