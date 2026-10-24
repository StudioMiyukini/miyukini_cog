//! @id     miyucloud_domain_services_home_folder
//! @do     proteger_nom_dossier_personnel
//! @role   securite
//! @layer  app
//! @human  miyucloud — règles du dossier personnel « My Folder - {utilisateur} » (anti-squat, M-02/M-03).

//! Dossier personnel d'un utilisateur.
//!
//! Le nom `My Folder - {username}` est réservé : il n'est créé que par
//! `create_home_folder` (inscription / auto-réparation). Aucun dossier ne
//! peut être créé ni renommé avec ce préfixe, sinon un utilisateur pourrait
//! préparer l'arborescence d'un compte futur (M-02).

/// Préfixe réservé des dossiers personnels.
pub const HOME_FOLDER_PREFIX: &str = "My Folder - ";

/// Nom du dossier personnel de `username`.
pub fn home_folder_name(username: &str) -> String {
    format!("{HOME_FOLDER_PREFIX}{username}")
}

/// Vrai si `name` imite un dossier personnel (préfixe réservé, casse et
/// espaces de tête ignorés).
pub fn is_reserved_home_name(name: &str) -> bool {
    let candidate = name.trim_start().to_lowercase();
    let prefix = HOME_FOLDER_PREFIX.trim_end().to_lowercase();
    candidate.starts_with(&prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_name_is_reserved() {
        assert!(is_reserved_home_name("My Folder - bob"));
        assert!(is_reserved_home_name("my folder - bob"));
        assert!(is_reserved_home_name("  MY FOLDER -bob"));
        assert!(is_reserved_home_name(&home_folder_name("alice")));
    }

    #[test]
    fn ordinary_names_are_allowed() {
        assert!(!is_reserved_home_name("Photos"));
        assert!(!is_reserved_home_name("My Folders"));
        assert!(!is_reserved_home_name("Folder - bob"));
        assert!(!is_reserved_home_name("Not My Folder - bob"));
    }
}
