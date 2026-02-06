//! Service d'authentification JayFestival (base fille SQLite via KindMother).
//!
//! @id: auth_sign_in
//! @do: authentifie un utilisateur par email et mot de passe (profiles SQLite).
//! @id: auth_sign_up
//! @do: crée un compte utilisateur (profiles SQLite).
//! @id: auth_sign_out
//! @do: déconnecte l'utilisateur (session côté app).

mod permissions;

pub use permissions::{auth_can_access_edition, auth_user_type_from_profile};

use crate::data::{JayFestivalDb, Profile};

/// Résultat d'une opération auth (succès ou erreur message).
pub type AuthResult<T> = Result<T, AuthError>;

/// Erreur d'authentification (refus, format, base).
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

/// Session utilisateur (après sign_in ou sign_up).
#[derive(Debug, Clone)]
pub struct AuthSession {
    /// Identifiant utilisateur (profiles.id).
    pub user_id: String,
    /// Email de l'utilisateur (si disponible).
    pub email: Option<String>,
    /// Jeton local (identifiant de session, pas JWT distant).
    pub access_token: String,
    /// Profil (table profiles) chargé optionnellement.
    pub profile: Option<Profile>,
}

/// Connexion par email et mot de passe (base fille KindMother).
///
/// @id: auth_sign_in
/// @do: authentifie un utilisateur par email et mot de passe (profiles SQLite).
pub fn auth_sign_in(db: &JayFestivalDb, email: &str, password: &str) -> AuthResult<AuthSession> {
    let profile = db
        .profile_by_email_password(email, password)
        .map_err(|e| AuthError {
            message: format!("base de données: {}", e),
        })?;
    let profile = profile.ok_or_else(|| AuthError {
        message: "Email ou mot de passe incorrect.".to_string(),
    })?;
    let user_id = profile.id.clone().unwrap_or_default();
    Ok(AuthSession {
        user_id: user_id.clone(),
        email: profile.email.clone(),
        access_token: user_id,
        profile: Some(profile),
    })
}

/// Inscription par email et mot de passe (base fille KindMother).
///
/// @id: auth_sign_up
/// @do: crée un compte utilisateur (profiles SQLite).
pub fn auth_sign_up(
    db: &JayFestivalDb,
    email: &str,
    password: &str,
    user_type: &str,
) -> AuthResult<AuthSession> {
    let user_id = db.profile_create(email, password, user_type).map_err(|e| AuthError {
        message: format!("création compte: {}", e),
    })?;
    let profile = db.profile_by_id(&user_id).map_err(|e| AuthError {
        message: format!("lecture profil: {}", e),
    })?;
    Ok(AuthSession {
        user_id: user_id.clone(),
        email: Some(email.to_string()),
        access_token: user_id.clone(),
        profile,
    })
}

/// Déconnecte l'utilisateur (la session est gérée par l'app).
///
/// @id: auth_sign_out
/// @do: déconnecte l'utilisateur.
pub fn auth_sign_out() -> AuthResult<()> {
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
        use crate::data::Profile;
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
}
