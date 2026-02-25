//! Rôles et permissions JayManga.

/// Type d'utilisateur JayManga.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    /// Propriétaire du COG, vendeur.
    Admin,
    /// Lecteur authentifié avec un COG.
    Reader,
    /// Visiteur sans COG (démo uniquement).
    Visitor,
    /// Rôle inconnu.
    Unknown,
}

impl UserType {
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Admin => "admin",
            Self::Reader => "reader",
            Self::Visitor => "visitor",
            Self::Unknown => "unknown",
        }
    }

    #[must_use]
    pub fn from_str(s: &str) -> Self {
        match s {
            "admin" => Self::Admin,
            "reader" => Self::Reader,
            "visitor" => Self::Visitor,
            _ => Self::Unknown,
        }
    }
}

/// Type de résultat d'authentification.
pub type AuthResult<T> = Result<T, AuthError>;

/// Erreur d'authentification.
#[derive(Debug)]
pub struct AuthError(pub String);

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Auth error: {}", self.0)
    }
}

impl std::error::Error for AuthError {}

/// Vérifie si un utilisateur peut accéder aux fonctions d'administration.
#[must_use]
pub fn auth_can_admin(user_type: UserType) -> bool {
    user_type == UserType::Admin
}

/// Vérifie si un utilisateur peut acheter.
#[must_use]
pub fn auth_can_purchase(user_type: UserType) -> bool {
    user_type == UserType::Reader || user_type == UserType::Admin
}

/// Vérifie si un utilisateur peut mettre en favoris.
#[must_use]
pub fn auth_can_favorite(user_type: UserType) -> bool {
    user_type == UserType::Reader || user_type == UserType::Admin
}
