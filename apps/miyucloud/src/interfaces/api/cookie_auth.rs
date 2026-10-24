//! @id     miyucloud_interfaces_api_cookie_auth
//! @do     definir_access_cookie
//! @role   securite
//! @layer  app
//! @human  miyucloud — cookies de session du navigateur (identifiant opaque, __Host-, Strict).

//! Cookies de session du navigateur (inventaire des secrets partagés #18, lot 2).
//!
//! Le cookie de session porte un **identifiant aléatoire opaque** résolu côté
//! serveur ([`crate::infrastructure::services::session_token_service`]) : aucune
//! signature, aucun secret partagé, révocable à la déconnexion.
//!
//! Attributs (fail-closed) :
//! - session : `__Host-miyucloud_session` — `Secure; HttpOnly; SameSite=Strict; Path=/`,
//!   sans `Domain` (préfixe `__Host-` : le navigateur refuse toute autre forme) ;
//! - rafraîchissement : `__Secure-miyucloud_refresh` — `Secure; HttpOnly;
//!   SameSite=Strict; Path=/api/auth` ;
//! - CSRF (double soumission, lisible par le JS) : `miyucloud_csrf` —
//!   `Secure; SameSite=Strict; Path=/`.
//!
//! `Secure` est **activé par défaut**. Seul `MIYUCLOUD_COOKIE_SECURE=false`
//! (développement en HTTP hors `localhost`) le retire ; les cookies perdent alors
//! leurs préfixes (`miyucloud_session`, `miyucloud_refresh`), le navigateur
//! refusant `__Host-`/`__Secure-` sans `Secure`.
//!
//! Les clients DAV utilisent `Authorization: Basic` (mot de passe d'application)
//! et ne sont pas concernés.

use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, HeaderValue};
use std::sync::OnceLock;

/// Nom du cookie de session (mode sécurisé, par défaut).
pub const ACCESS_COOKIE_SECURE: &str = "__Host-miyucloud_session";
/// Nom du cookie de session si `MIYUCLOUD_COOKIE_SECURE=false`.
pub const ACCESS_COOKIE_INSECURE: &str = "miyucloud_session";
/// Nom du cookie de rafraîchissement (mode sécurisé, par défaut).
pub const REFRESH_COOKIE_SECURE: &str = "__Secure-miyucloud_refresh";
/// Nom du cookie de rafraîchissement si `MIYUCLOUD_COOKIE_SECURE=false`.
pub const REFRESH_COOKIE_INSECURE: &str = "miyucloud_refresh";
/// Ancien cookie JWT (effacé à la connexion et à la déconnexion).
pub const LEGACY_ACCESS_COOKIE: &str = "miyucloud_access";
/// Cookie name for the CSRF double-submit token (readable by JS).
pub const CSRF_COOKIE: &str = "miyucloud_csrf";
/// Header the frontend must send with the CSRF token value.
pub const CSRF_HEADER: &str = "x-csrf-token";

/// Chemin du cookie de rafraîchissement.
const REFRESH_PATH: &str = "/api/auth";

/// Vrai sauf si `MIYUCLOUD_COOKIE_SECURE` vaut explicitement `false`/`0`.
pub fn secure_from_env(valeur: Option<&str>) -> bool {
    !matches!(
        valeur.map(|v| v.trim().to_ascii_lowercase()).as_deref(),
        Some("false") | Some("0") | Some("no") | Some("off")
    )
}

/// Mode des cookies, lu une seule fois.
pub fn cookie_secure() -> bool {
    static SECURE: OnceLock<bool> = OnceLock::new();
    *SECURE.get_or_init(|| {
        let secure = secure_from_env(std::env::var("MIYUCLOUD_COOKIE_SECURE").ok().as_deref());
        if !secure {
            tracing::warn!(
                "MIYUCLOUD_COOKIE_SECURE=false — cookies de session sans Secure ni préfixe \
                 __Host-. Ne PAS utiliser en production."
            );
        }
        secure
    })
}

/// Nom du cookie de session pour le mode donné.
pub fn access_cookie_name_for(secure: bool) -> &'static str {
    if secure {
        ACCESS_COOKIE_SECURE
    } else {
        ACCESS_COOKIE_INSECURE
    }
}

/// Nom du cookie de rafraîchissement pour le mode donné.
pub fn refresh_cookie_name_for(secure: bool) -> &'static str {
    if secure {
        REFRESH_COOKIE_SECURE
    } else {
        REFRESH_COOKIE_INSECURE
    }
}

/// Nom du cookie de session actif.
pub fn access_cookie_name() -> &'static str {
    access_cookie_name_for(cookie_secure())
}

/// Nom du cookie de rafraîchissement actif.
pub fn refresh_cookie_name() -> &'static str {
    refresh_cookie_name_for(cookie_secure())
}

fn suffixe_secure(secure: bool) -> &'static str {
    if secure { "; Secure" } else { "" }
}

/// Cookie de session (`Path=/`, sans `Domain`).
pub fn build_access_cookie(secure: bool, value: &str, max_age_secs: i64) -> String {
    format!(
        "{}={value}; HttpOnly; SameSite=Strict; Path=/; Max-Age={max_age_secs}{}",
        access_cookie_name_for(secure),
        suffixe_secure(secure)
    )
}

/// Cookie de rafraîchissement (`Path=/api/auth`).
pub fn build_refresh_cookie(secure: bool, value: &str, max_age_secs: i64) -> String {
    format!(
        "{}={value}; HttpOnly; SameSite=Strict; Path={REFRESH_PATH}; Max-Age={max_age_secs}{}",
        refresh_cookie_name_for(secure),
        suffixe_secure(secure)
    )
}

/// Cookie CSRF (lisible par le JS).
pub fn build_csrf_cookie(secure: bool, value: &str, max_age_secs: i64) -> String {
    format!(
        "{CSRF_COOKIE}={value}; SameSite=Strict; Path=/; Max-Age={max_age_secs}{}",
        suffixe_secure(secure)
    )
}

fn append(headers: &mut HeaderMap, cookie: &str) {
    if let Ok(val) = HeaderValue::from_str(cookie) {
        headers.append(SET_COOKIE, val);
    }
}

/// Pose les cookies de session et de rafraîchissement.
pub fn append_auth_cookies(
    headers: &mut HeaderMap,
    access_token: &str,
    refresh_token: &str,
    access_expiry_secs: i64,
    refresh_expiry_secs: i64,
) {
    let secure = cookie_secure();
    append(
        headers,
        &build_access_cookie(secure, access_token, access_expiry_secs),
    );
    append(
        headers,
        &build_refresh_cookie(secure, refresh_token, refresh_expiry_secs),
    );
    // Efface l'ancien cookie JWT s'il traîne encore dans le navigateur.
    append(
        headers,
        &format!("{LEGACY_ACCESS_COOKIE}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"),
    );
}

/// Expire immédiatement les cookies de session (déconnexion).
pub fn append_clear_cookies(headers: &mut HeaderMap) {
    let secure = cookie_secure();
    append(headers, &build_access_cookie(secure, "", 0));
    append(headers, &build_refresh_cookie(secure, "", 0));
    append(
        headers,
        &format!("{LEGACY_ACCESS_COOKIE}=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0"),
    );
}

/// Extract a named cookie value from the `Cookie` request header(s).
///
/// Correspondance exacte du nom (un cookie `nomX=…` ne répond pas à `nom`).
pub fn extract_cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    for cookie_header in headers.get_all(axum::http::header::COOKIE) {
        let Ok(cookie_str) = cookie_header.to_str() else {
            continue;
        };
        for pair in cookie_str.split(';') {
            if let Some((n, v)) = pair.trim().split_once('=')
                && n == name
                && !v.is_empty()
            {
                return Some(v.to_string());
            }
        }
    }
    None
}

/// Identifiant de session présenté par le navigateur (cookie actif).
pub fn extract_session_cookie(headers: &HeaderMap) -> Option<String> {
    extract_cookie_value(headers, access_cookie_name())
}

/// Jeton de rafraîchissement présenté par le navigateur (cookie actif).
pub fn extract_refresh_cookie(headers: &HeaderMap) -> Option<String> {
    extract_cookie_value(headers, refresh_cookie_name())
}

// ────────────────────────────────────────────────────────────
// CSRF double-submit cookie helpers
// ────────────────────────────────────────────────────────────

/// Jeton CSRF aléatoire (UUIDv4 : 122 bits issus de l'aléa du système).
pub fn generate_csrf_token() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Append a CSRF double-submit cookie alongside the auth cookies.
pub fn append_csrf_cookie(headers: &mut HeaderMap, max_age_secs: i64) {
    let token = generate_csrf_token();
    append(
        headers,
        &build_csrf_cookie(cookie_secure(), &token, max_age_secs),
    );
}

/// Clear the CSRF cookie (on logout).
pub fn append_clear_csrf_cookie(headers: &mut HeaderMap) {
    append(headers, &build_csrf_cookie(cookie_secure(), "", 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secure_is_the_default() {
        assert!(secure_from_env(None));
        assert!(secure_from_env(Some("true")));
        assert!(secure_from_env(Some("n'importe quoi")));
        assert!(!secure_from_env(Some("false")));
        assert!(!secure_from_env(Some(" 0 ")));
    }

    #[test]
    fn session_cookie_is_host_prefixed_strict_httponly() {
        let c = build_access_cookie(true, "mcs_abc", 3600);
        assert!(c.starts_with("__Host-miyucloud_session=mcs_abc;"));
        assert!(c.contains("; HttpOnly"));
        assert!(c.contains("; SameSite=Strict"));
        assert!(c.contains("; Path=/;"));
        assert!(c.ends_with("; Secure"));
        assert!(!c.to_ascii_lowercase().contains("domain="));
    }

    #[test]
    fn refresh_cookie_is_secure_prefixed_and_scoped() {
        let c = build_refresh_cookie(true, "mcr_x", 60);
        assert!(c.starts_with("__Secure-miyucloud_refresh=mcr_x;"));
        assert!(c.contains("Path=/api/auth"));
        assert!(c.contains("SameSite=Strict") && c.contains("HttpOnly") && c.ends_with("; Secure"));
    }

    #[test]
    fn insecure_mode_drops_prefixes() {
        let c = build_access_cookie(false, "v", 1);
        assert!(c.starts_with("miyucloud_session=v;") && !c.contains("Secure"));
        assert!(build_refresh_cookie(false, "v", 1).starts_with("miyucloud_refresh="));
    }

    #[test]
    fn csrf_cookie_is_readable_but_strict() {
        let c = build_csrf_cookie(true, "t", 10);
        assert!(c.starts_with("miyucloud_csrf=t;"));
        assert!(!c.contains("HttpOnly"));
        assert!(c.contains("SameSite=Strict") && c.ends_with("; Secure"));
    }

    #[test]
    fn cookie_extraction_matches_exact_names() {
        let mut h = HeaderMap::new();
        h.insert(
            axum::http::header::COOKIE,
            HeaderValue::from_static(
                "miyucloud_session_x=faux; __Host-miyucloud_session=vrai; miyucloud_csrf=c",
            ),
        );
        assert_eq!(
            extract_cookie_value(&h, "__Host-miyucloud_session").as_deref(),
            Some("vrai")
        );
        assert_eq!(extract_cookie_value(&h, "miyucloud_session"), None);
        assert_eq!(
            extract_cookie_value(&h, "miyucloud_csrf").as_deref(),
            Some("c")
        );
    }
}

#[cfg(test)]
mod session_flow_tests {
    //! Parcours navigateur : connexion (cookie posé), requête (cookie résolu côté
    //! serveur), déconnexion (session révoquée), identifiant forgé refusé.
    use super::*;
    use crate::application::ports::auth_ports::TokenServicePort;
    use crate::domain::entities::user::{User, UserRole};
    use crate::infrastructure::services::session_token_service::SessionTokenService;

    fn alice() -> User {
        User::from_data(
            uuid::Uuid::new_v4(),
            "alice".into(),
            "alice@example.com".into(),
            "h".into(),
            UserRole::User,
            1,
            0,
            chrono::Utc::now(),
            chrono::Utc::now(),
            None,
            true,
        )
    }

    /// Ce que le navigateur renverrait : `Set-Cookie` → `Cookie`.
    fn cookie_renvoye(set_cookie: &str) -> HeaderMap {
        let paire = set_cookie.split(';').next().unwrap();
        let mut h = HeaderMap::new();
        h.insert(
            axum::http::header::COOKIE,
            HeaderValue::from_str(paire).unwrap(),
        );
        h
    }

    #[test]
    fn login_logout_and_forged_session_id() {
        let store = SessionTokenService::new(3600, 86400);
        let jeton = store.generate_access_token(&alice()).unwrap();
        // Connexion : cookie __Host- posé.
        let set = build_access_cookie(true, &jeton, 3600);
        let requete = cookie_renvoye(&set);
        let lu = extract_cookie_value(&requete, ACCESS_COOKIE_SECURE).unwrap();
        assert_eq!(store.validate_token(&lu).unwrap().username, "alice");
        // Déconnexion : la session est révoquée côté serveur, le cookie rejoué échoue.
        assert!(store.revoke_token(&lu));
        assert!(store.validate_token(&lu).is_err());
        let efface = build_access_cookie(true, "", 0);
        assert!(efface.contains("Max-Age=0"));
        // Identifiant forgé (forme correcte, jamais émis).
        let forge = cookie_renvoye(&build_access_cookie(
            true,
            "mcs_AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            3600,
        ));
        let lu = extract_cookie_value(&forge, ACCESS_COOKIE_SECURE).unwrap();
        assert!(store.validate_token(&lu).is_err());
        // Ancien cookie JWT : ignoré (autre nom) et refusé (autre format).
        let ancien = cookie_renvoye("miyucloud_access=eyJhbGciOiJIUzI1NiJ9.e30.sig");
        assert!(extract_cookie_value(&ancien, ACCESS_COOKIE_SECURE).is_none());
    }
}
