//! Arborescence virtuelle — chemins canoniques + anti path-traversal.
//!
//! Toute écriture / lecture / suppression passe par `canonical_path` qui :
//! - normalise les séparateurs en `/`,
//! - rejette les composants `..`,
//! - rejette les chemins absolus (commençant par `/` ou `<lettre>:`),
//! - rejette les caractères de contrôle et octets nuls,
//! - rejette les composants vides.

/// Erreur de validation de chemin.
#[derive(Debug, thiserror::Error)]
pub enum TreeError {
    /// Tentative de path traversal (composant `..`).
    #[error("path traversal détecté : composant '..' interdit")]
    PathTraversal,
    /// Chemin absolu (commence par `/` ou `<lettre>:`).
    #[error("chemin absolu interdit : '{0}'")]
    AbsolutePath(String),
    /// Caractère interdit (contrôle ou nul).
    #[error("caractère interdit dans le chemin : {0:?}")]
    ForbiddenChar(char),
    /// Composant vide ou réservé.
    #[error("composant invalide : '{0}'")]
    InvalidComponent(String),
    /// Chemin vide après normalisation.
    #[error("chemin vide")]
    Empty,
}

/// Normalise un chemin et le rejette s'il est dangereux.
///
/// Renvoie une chaîne canonique séparée par `/`, sans `.` / `..`, jamais
/// absolue.
pub fn canonical_path(input: &str) -> Result<String, TreeError> {
    if input.is_empty() {
        return Err(TreeError::Empty);
    }

    // Refus du chemin absolu Windows-style (`C:/...`) ou POSIX (`/...`).
    if input.starts_with('/') || input.starts_with('\\') {
        return Err(TreeError::AbsolutePath(input.to_string()));
    }
    if input.len() >= 2 {
        let mut chars = input.chars();
        let c0 = chars.next().unwrap();
        let c1 = chars.next().unwrap();
        if c0.is_ascii_alphabetic() && c1 == ':' {
            return Err(TreeError::AbsolutePath(input.to_string()));
        }
    }

    // Refus caractères interdits.
    for c in input.chars() {
        if c == '\0' || (c.is_control() && c != '\t') {
            return Err(TreeError::ForbiddenChar(c));
        }
    }

    // Normalise séparateurs.
    let normalized = input.replace('\\', "/");

    // Découpe et valide chaque composant.
    let mut out_components: Vec<&str> = Vec::new();
    for component in normalized.split('/') {
        if component.is_empty() || component == "." {
            // `a//b` ou `./a` → on absorbe les composants vides / "."
            continue;
        }
        if component == ".." {
            return Err(TreeError::PathTraversal);
        }
        // Refus composants spéciaux Windows (CON, PRN, etc.) — strict mais simple.
        let lower = component.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "con" | "prn" | "aux" | "nul" | "com1" | "com2" | "lpt1"
        ) {
            return Err(TreeError::InvalidComponent(component.to_string()));
        }
        out_components.push(component);
    }

    if out_components.is_empty() {
        return Err(TreeError::Empty);
    }

    Ok(out_components.join("/"))
}

/// Composant parent d'un chemin canonique. `None` si racine.
#[must_use]
pub fn parent(canonical: &str) -> Option<String> {
    canonical.rfind('/').map(|i| canonical[..i].to_string())
}

/// Nom de fichier (dernier composant).
#[must_use]
pub fn file_name(canonical: &str) -> &str {
    canonical.rsplit('/').next().unwrap_or(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_path() {
        assert_eq!(canonical_path("notes/hello.txt").unwrap(), "notes/hello.txt");
    }

    #[test]
    fn windows_separators_normalized() {
        assert_eq!(canonical_path("a\\b\\c").unwrap(), "a/b/c");
    }

    #[test]
    fn double_slashes_collapsed() {
        assert_eq!(canonical_path("a//b/c").unwrap(), "a/b/c");
    }

    #[test]
    fn dot_components_removed() {
        assert_eq!(canonical_path("./a/./b").unwrap(), "a/b");
    }

    #[test]
    fn parent_traversal_rejected() {
        assert!(matches!(
            canonical_path("a/../b"),
            Err(TreeError::PathTraversal)
        ));
    }

    #[test]
    fn leading_traversal_rejected() {
        assert!(matches!(
            canonical_path("../etc/passwd"),
            Err(TreeError::PathTraversal)
        ));
    }

    #[test]
    fn absolute_unix_rejected() {
        assert!(matches!(
            canonical_path("/etc/passwd"),
            Err(TreeError::AbsolutePath(_))
        ));
    }

    #[test]
    fn absolute_windows_rejected() {
        assert!(matches!(
            canonical_path("C:/data"),
            Err(TreeError::AbsolutePath(_))
        ));
    }

    #[test]
    fn null_byte_rejected() {
        assert!(matches!(
            canonical_path("a\0b"),
            Err(TreeError::ForbiddenChar(_))
        ));
    }

    #[test]
    fn empty_path_rejected() {
        assert!(matches!(canonical_path(""), Err(TreeError::Empty)));
    }

    #[test]
    fn only_dot_rejected() {
        assert!(matches!(canonical_path("."), Err(TreeError::Empty)));
    }

    #[test]
    fn reserved_windows_name_rejected() {
        assert!(matches!(
            canonical_path("con/file"),
            Err(TreeError::InvalidComponent(_))
        ));
    }

    #[test]
    fn parent_works() {
        assert_eq!(parent("a/b/c"), Some("a/b".to_string()));
        assert_eq!(parent("a"), None);
    }

    #[test]
    fn file_name_works() {
        assert_eq!(file_name("a/b/c.txt"), "c.txt");
        assert_eq!(file_name("solo"), "solo");
    }
}
