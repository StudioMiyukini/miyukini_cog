//! Service d'authentification JayFestival (Supabase Auth alpha).
//!
//! @id: auth_sign_in
//! @do: authentifie un utilisateur par email et mot de passe (Supabase Auth).
//! @layer: domain
//!
//! @id: auth_sign_up
//! @do: crée un compte utilisateur par email et mot de passe (Supabase Auth).
//! @layer: domain
//!
//! @id: auth_session_current
//! @do: récupère la session courante (utilisateur + token) si connecté.
//! @layer: domain
//!
//! @id: auth_sign_out
//! @do: déconnecte l'utilisateur et invalide la session côté client.
//! @layer: domain

mod permissions;

pub use permissions::{auth_can_access_edition, auth_user_type_from_profile};

use crate::supabase::{SupabaseClient, Profile};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Résultat d'une opération auth (succès ou erreur message).
pub type AuthResult<T> = Result<T, AuthError>;

/// Erreur d'authentification (réseau, refus, format).
#[derive(Debug, Clone)]
pub struct AuthError {
    /// Message d'erreur affiché à l'utilisateur.
    pub message: String,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthError {}

/// Session utilisateur (après sign_in ou récupérée via session_current).
#[derive(Debug, Clone)]
pub struct AuthSession {
    /// Identifiant utilisateur Supabase Auth.
    pub user_id: String,
    /// Email de l'utilisateur (si disponible).
    pub email: Option<String>,
    /// Token JWT pour les requêtes authentifiées.
    pub access_token: String,
    /// Profil étendu (table profiles) chargé optionnellement.
    pub profile: Option<Profile>,
}

/// Corps pour sign in (grant_type password).
#[derive(Serialize)]
struct TokenRequest {
    grant_type: String,
    email: String,
    password: String,
}

/// Réponse token Supabase (extrait access_token).
#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    #[allow(dead_code)]
    refresh_token: Option<String>,
    #[allow(dead_code)]
    expires_in: Option<u64>,
    user: Option<SupabaseAuthUser>,
}

#[derive(Deserialize)]
struct SupabaseAuthUser {
    id: Option<String>,
    email: Option<String>,
}

/// Corps pour sign up.
#[derive(Serialize)]
struct SignUpRequest {
    email: String,
    password: String,
}

/// Réponse signup (user + optional session).
#[derive(Deserialize)]
struct SignUpResponse {
    id: Option<String>,
    email: Option<String>,
    access_token: Option<String>,
}

/// Connexion par email et mot de passe.
///
/// @id: auth_sign_in
/// @do: authentifie un utilisateur par email et mot de passe (Supabase Auth).
/// @layer: domain
pub fn auth_sign_in(
    client: &Arc<SupabaseClient>,
    email: &str,
    password: &str,
) -> AuthResult<AuthSession> {
    let body = serde_json::to_vec(&TokenRequest {
        grant_type: "password".to_string(),
        email: email.to_string(),
        password: password.to_string(),
    })
    .map_err(|e| AuthError {
        message: format!("serialize token request: {}", e),
    })?;
    let resp = client.auth_post("token?grant_type=password", &body).map_err(|e| AuthError {
        message: format!("request: {}", e),
    })?;
    let status = resp.status();
    let bytes = resp.bytes().map_err(|e| AuthError {
        message: format!("read response: {}", e),
    })?;
    if !status.is_success() {
        let msg = String::from_utf8_lossy(&bytes);
        return Err(AuthError {
            message: format!("auth failed ({}): {}", status, msg),
        });
    }
    let data: TokenResponse = serde_json::from_slice(&bytes).map_err(|e| AuthError {
        message: format!("parse token response: {}", e),
    })?;
    let access_token = data
        .access_token
        .ok_or_else(|| AuthError {
            message: "no access_token in response".to_string(),
        })?;
    let user_id = data
        .user
        .as_ref()
        .and_then(|u| u.id.as_ref())
        .cloned()
        .unwrap_or_else(|| "".to_string());
    let email = data.user.and_then(|u| u.email);
    client.set_access_token(Some(access_token.clone()));
    Ok(AuthSession {
        user_id,
        email,
        access_token,
        profile: None,
    })
}

/// Inscription par email et mot de passe.
///
/// @id: auth_sign_up
/// @do: crée un compte utilisateur par email et mot de passe (Supabase Auth).
/// @layer: domain
pub fn auth_sign_up(
    client: &Arc<SupabaseClient>,
    email: &str,
    password: &str,
) -> AuthResult<AuthSession> {
    let body = serde_json::to_vec(&SignUpRequest {
        email: email.to_string(),
        password: password.to_string(),
    })
    .map_err(|e| AuthError {
        message: format!("serialize signup request: {}", e),
    })?;
    let resp = client.auth_post("signup", &body).map_err(|e| AuthError {
        message: format!("request: {}", e),
    })?;
    let status = resp.status();
    let bytes = resp.bytes().map_err(|e| AuthError {
        message: format!("read response: {}", e),
    })?;
    if !status.is_success() {
        let msg = String::from_utf8_lossy(&bytes);
        return Err(AuthError {
            message: format!("signup failed ({}): {}", status, msg),
        });
    }
    let data: SignUpResponse = serde_json::from_slice(&bytes).map_err(|e| AuthError {
        message: format!("parse signup response: {}", e),
    })?;
    let user_id = data.id.unwrap_or_else(|| "".to_string());
    let access_token = data.access_token;
    if let Some(ref token) = access_token {
        client.set_access_token(Some(token.clone()));
    }
    Ok(AuthSession {
        user_id,
        email: data.email,
        access_token: access_token.unwrap_or_default(),
        profile: None,
    })
}

/// Récupère la session courante (utilisateur + token) si un token est déjà stocké.
///
/// @id: auth_session_current
/// @do: récupère la session courante (utilisateur + token) si connecté.
/// @layer: domain
pub fn auth_session_current(client: &Arc<SupabaseClient>) -> AuthResult<Option<AuthSession>> {
    let token = match client.access_token() {
        Some(t) if !t.is_empty() => t,
        _ => return Ok(None),
    };
    let resp = client.auth_get("user", &token).map_err(|e| AuthError {
        message: format!("get user: {}", e),
    })?;
    let status = resp.status();
    let bytes = resp.bytes().map_err(|e| AuthError {
        message: format!("read response: {}", e),
    })?;
    if !status.is_success() {
        client.set_access_token(None);
        return Ok(None);
    }
    let user: SupabaseAuthUser = serde_json::from_slice(&bytes).map_err(|e| AuthError {
        message: format!("parse user: {}", e),
    })?;
    Ok(Some(AuthSession {
        user_id: user.id.unwrap_or_default(),
        email: user.email,
        access_token: token,
        profile: None,
    }))
}

/// Déconnecte l'utilisateur et invalide la session côté client.
///
/// @id: auth_sign_out
/// @do: déconnecte l'utilisateur et invalide la session côté client.
/// @layer: domain
pub fn auth_sign_out(client: &Arc<SupabaseClient>) -> AuthResult<()> {
    if let Some(token) = client.access_token() {
        let _ = client.auth_post_with_token("logout", b"{}", &token);
    }
    client.set_access_token(None);
    Ok(())
}

#[cfg(test)]
mod auth_tests {
    use super::{AuthError, AuthSession};

    #[test]
    fn auth_session_construction() {
        let s = AuthSession {
            user_id: "uid-1".to_string(),
            email: Some("a@b.com".to_string()),
            access_token: "tok".to_string(),
            profile: None,
        };
        assert_eq!(s.user_id, "uid-1");
        assert_eq!(s.email.as_deref(), Some("a@b.com"));
        assert_eq!(s.access_token, "tok");
        assert!(s.profile.is_none());
    }

    #[test]
    fn auth_session_with_profile() {
        use crate::supabase::Profile;
        let p = Profile {
            user_type: Some("manager".to_string()),
            ..Profile::default()
        };
        let s = AuthSession {
            user_id: "u".to_string(),
            email: None,
            access_token: "t".to_string(),
            profile: Some(p),
        };
        assert!(s.profile.is_some());
        assert_eq!(s.profile.as_ref().unwrap().user_type.as_deref(), Some("manager"));
    }

    #[test]
    fn auth_error_display() {
        let e = AuthError {
            message: "invalid credentials".to_string(),
        };
        assert_eq!(e.to_string(), "invalid credentials");
    }

    #[test]
    fn auth_error_from_io_like() {
        let e = AuthError {
            message: "request: connection refused".to_string(),
        };
        assert!(e.message.contains("connection refused"));
    }
}
