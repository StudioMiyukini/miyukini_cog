//! Service d'authentification JayXpose (base fille SQLite via KindMother).
//!
//! @id: jayxpose_auth_sign_in
//! @do: authentifie un exposant par email et mot de passe (exposants SQLite).
//! @id: jayxpose_auth_sign_up
//! @do: cree un compte exposant (exposants SQLite).
//! @id: jayxpose_auth_sign_out
//! @do: deconnecte l'utilisateur (session cote app).

use crate::data::{ExposantProfile, JayXposeDb};
use sha2::{Digest, Sha256};

/// Resultat d'une operation auth (succes ou erreur message).
pub type AuthResult<T> = Result<T, AuthError>;

/// Erreur d'authentification (refus, format, base).
#[derive(Debug, Clone)]
pub struct AuthError {
    /// Message d'erreur affiche a l'utilisateur.
    pub message: String,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AuthError {}

/// Session utilisateur JayXpose (apres sign_in ou sign_up).
#[derive(Debug, Clone)]
pub struct AuthSession {
    /// Identifiant utilisateur (exposants.id).
    pub user_id: String,
    /// Email de l'utilisateur (si disponible).
    pub email: Option<String>,
    /// Jeton local (identifiant de session, pas JWT distant).
    pub access_token: String,
    /// Profil exposant charge optionnellement.
    pub profile: Option<ExposantProfile>,
}

/// Calcule le hash SHA-256 hexadecimal d'un mot de passe.
fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    format!("{result:x}")
}

/// Connexion par email et mot de passe (base fille KindMother).
///
/// Cherche un exposant par contact_email, verifie le hash SHA-256 du mot de passe
/// via le champ password_hash en base, et retourne une AuthSession en cas de succes.
///
/// @id: jx_auth_sign_in
/// @do: authentifie un exposant par email et mot de passe (exposants SQLite).
/// @layer: app
pub fn auth_sign_in(db: &JayXposeDb, email: &str, password: &str) -> AuthResult<AuthSession> {
    let password_hash = hash_password(password);

    let profile = db
        .exposant_by_email_password(email, &password_hash)
        .map_err(|e| AuthError {
            message: format!("base de donnees: {e}"),
        })?;

    let profile = profile.ok_or_else(|| AuthError {
        message: "Email ou mot de passe incorrect.".to_string(),
    })?;

    let user_id = profile.id.clone().unwrap_or_default();
    Ok(AuthSession {
        user_id: user_id.clone(),
        email: profile.contact_email.clone(),
        access_token: user_id,
        profile: Some(profile),
    })
}

/// Inscription par email, mot de passe et nom d'entreprise (base fille KindMother).
///
/// Cree un nouvel exposant avec UUID v4, hash le mot de passe en SHA-256,
/// et retourne une AuthSession avec le profil nouvellement cree.
///
/// @id: jx_auth_sign_up
/// @do: cree un compte exposant (exposants SQLite).
/// @layer: app
pub fn auth_sign_up(
    db: &JayXposeDb,
    email: &str,
    password: &str,
    company_name: &str,
) -> AuthResult<AuthSession> {
    let password_hash = hash_password(password);

    let user_id = db
        .exposant_create(email, &password_hash, company_name)
        .map_err(|e| AuthError {
            message: format!("creation compte: {e}"),
        })?;

    let profile = db.exposant_by_id(&user_id).map_err(|e| AuthError {
        message: format!("lecture profil: {e}"),
    })?;

    Ok(AuthSession {
        user_id: user_id.clone(),
        email: Some(email.to_string()),
        access_token: user_id,
        profile,
    })
}

/// Authentification déléguée via profil Central.
///
/// Cherche un exposant lié au `central_profile_id`. Si trouvé, retourne une
/// AuthSession sans vérification de mot de passe (la vérification a déjà eu
/// lieu côté Central/Origin).
///
/// @id: jx_auth_via_central
/// @do: authentifie un exposant via son profil Central lié.
/// @layer: app
pub fn auth_via_central(
    db: &JayXposeDb,
    central_profile_id: &str,
) -> AuthResult<Option<AuthSession>> {
    let profile = db
        .exposant_by_central_profile(central_profile_id)
        .map_err(|e| AuthError {
            message: format!("base de donnees: {e}"),
        })?;

    match profile {
        Some(p) => {
            let user_id = p.id.clone().unwrap_or_default();
            Ok(Some(AuthSession {
                user_id: user_id.clone(),
                email: p.contact_email.clone(),
                access_token: user_id,
                profile: Some(p),
            }))
        }
        None => Ok(None),
    }
}

/// Inscription déléguée via profil Central (crée un exposant lié sans password autonome).
///
/// @id: jx_auth_sign_up_central
/// @do: cree un compte exposant lié au profil Central.
/// @layer: app
pub fn auth_sign_up_central(
    db: &JayXposeDb,
    central_profile_id: &str,
    email: &str,
    company_name: &str,
) -> AuthResult<AuthSession> {
    // Vérifie qu'aucun exposant n'est déjà lié à ce profil Central
    if let Ok(Some(_)) = db.exposant_by_central_profile(central_profile_id) {
        return Err(AuthError {
            message: "Un exposant est déjà lié à ce profil Central.".to_string(),
        });
    }

    let user_id = db
        .exposant_create_from_central(central_profile_id, email, company_name)
        .map_err(|e| AuthError {
            message: format!("creation compte: {e}"),
        })?;

    let profile = db.exposant_by_id(&user_id).map_err(|e| AuthError {
        message: format!("lecture profil: {e}"),
    })?;

    Ok(AuthSession {
        user_id: user_id.clone(),
        email: Some(email.to_string()),
        access_token: user_id,
        profile,
    })
}

/// Deconnecte l'utilisateur (la session est geree par l'app, rien a faire cote serveur).
///
/// @id: jx_auth_sign_out
/// @do: deconnecte l'utilisateur.
/// @layer: app
pub fn auth_sign_out() -> AuthResult<()> {
    Ok(())
}

#[cfg(test)]
mod auth_tests {
    use super::{hash_password, AuthError, AuthSession};

    #[test]
    fn auth_session_construction() {
        let s = AuthSession {
            user_id: "exp-1".to_string(),
            email: Some("contact@entreprise.fr".to_string()),
            access_token: "tok-local".to_string(),
            profile: None,
        };
        assert_eq!(s.user_id, "exp-1");
        assert_eq!(s.email.as_deref(), Some("contact@entreprise.fr"));
        assert_eq!(s.access_token, "tok-local");
        assert!(s.profile.is_none());
    }

    #[test]
    fn auth_session_with_profile() {
        use crate::data::ExposantProfile;
        let p = ExposantProfile {
            company_name: Some("MonEntreprise SAS".to_string()),
            ..ExposantProfile::default()
        };
        let s = AuthSession {
            user_id: "exp-2".to_string(),
            email: None,
            access_token: "tok".to_string(),
            profile: Some(p),
        };
        assert!(s.profile.is_some());
        assert_eq!(
            s.profile.as_ref().unwrap().company_name.as_deref(),
            Some("MonEntreprise SAS")
        );
    }

    #[test]
    fn auth_error_display() {
        let e = AuthError {
            message: "identifiants invalides".to_string(),
        };
        assert_eq!(e.to_string(), "identifiants invalides");
    }

    #[test]
    fn auth_sign_out_ok() {
        assert!(super::auth_sign_out().is_ok());
    }

    #[test]
    fn hash_password_deterministic() {
        let h1 = hash_password("test123");
        let h2 = hash_password("test123");
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_password_different_inputs() {
        let h1 = hash_password("password1");
        let h2 = hash_password("password2");
        assert_ne!(h1, h2);
    }

    #[test]
    fn hash_password_not_plaintext() {
        let h = hash_password("mysecret");
        assert_ne!(h, "mysecret");
        assert_eq!(h.len(), 64, "SHA-256 hex digest should be 64 chars");
    }
}
