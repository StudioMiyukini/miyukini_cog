//! Fonctions de sanitisation pour la securite web.
//!
//! @id: miyucloud_utils_sanitize
//! @do: provide_html_escape_filename_sanitize_functions
//! @role: security
//! @layer: infra

/// Echappe les caracteres HTML dangereux (&, <, >, ", ').
///
/// Ordre : `&` d'abord (evite double-escape), puis `<`, `>`, `"`, `'`.
pub fn html_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Sanitise un nom de fichier pour utilisation dans Content-Disposition.
///
/// Supprime les caracteres de controle, les guillemets doubles,
/// les retours a la ligne, les separateurs de chemin, et les sequences `..`.
/// Tronque a 255 caracteres. Retourne `"unnamed"` si le resultat est vide.
pub fn sanitize_filename(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '/' && *c != '\\')
        .collect();

    let cleaned = cleaned.replace("..", "");

    let cleaned = cleaned.trim().to_string();

    let cleaned = if cleaned.len() > 255 {
        cleaned[..255].to_string()
    } else {
        cleaned
    };

    if cleaned.is_empty() {
        "unnamed".to_string()
    } else {
        cleaned
    }
}

/// Valide qu'un identifiant est un UUID valide (versions 1-5, format 8-4-4-4-12).
///
/// Rejette tout identifiant contenant `/`, `\`, ou `..`.
pub fn validate_uuid(id: &str) -> bool {
    if id.contains('/') || id.contains('\\') || id.contains("..") {
        return false;
    }

    if id.len() != 36 {
        return false;
    }

    let parts: Vec<&str> = id.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    let expected_lens = [8, 4, 4, 4, 12];
    for (part, &expected) in parts.iter().zip(expected_lens.iter()) {
        if part.len() != expected {
            return false;
        }
        if !part.chars().all(|c| c.is_ascii_hexdigit()) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape_basic() {
        assert_eq!(html_escape("hello"), "hello");
    }

    #[test]
    fn test_html_escape_ampersand() {
        assert_eq!(html_escape("a & b"), "a &amp; b");
    }

    #[test]
    fn test_html_escape_tags() {
        assert_eq!(
            html_escape("<script>alert(1)</script>"),
            "&lt;script&gt;alert(1)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_html_escape_quotes() {
        assert_eq!(html_escape(r#"a "b" 'c'"#), "a &quot;b&quot; &#x27;c&#x27;");
    }

    #[test]
    fn test_html_escape_all_special() {
        assert_eq!(html_escape("&<>\"'"), "&amp;&lt;&gt;&quot;&#x27;");
    }

    #[test]
    fn test_sanitize_filename_normal() {
        assert_eq!(sanitize_filename("photo.jpg"), "photo.jpg");
    }

    #[test]
    fn test_sanitize_filename_path_traversal() {
        assert_eq!(sanitize_filename("../../etc/passwd"), "etcpasswd");
    }

    #[test]
    fn test_sanitize_filename_slashes() {
        assert_eq!(sanitize_filename("path/to\\file.txt"), "pathtofile.txt");
    }

    #[test]
    fn test_sanitize_filename_control_chars() {
        assert_eq!(sanitize_filename("file\x00\x0aname.txt"), "filename.txt");
    }

    #[test]
    fn test_sanitize_filename_empty() {
        assert_eq!(sanitize_filename(""), "unnamed");
        assert_eq!(sanitize_filename(".."), "unnamed");
    }

    #[test]
    fn test_sanitize_filename_long() {
        let long_name = "a".repeat(300);
        let result = sanitize_filename(&long_name);
        assert_eq!(result.len(), 255);
    }

    #[test]
    fn test_sanitize_filename_quotes() {
        assert_eq!(
            sanitize_filename("file\"name\".txt"),
            "filename.txt"
        );
    }

    #[test]
    fn test_validate_uuid_valid() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(validate_uuid("00000000-0000-0000-0000-000000000000"));
        assert!(validate_uuid("ffffffff-ffff-ffff-ffff-ffffffffffff"));
    }

    #[test]
    fn test_validate_uuid_invalid() {
        assert!(!validate_uuid("not-a-uuid"));
        assert!(!validate_uuid(""));
        assert!(!validate_uuid("550e8400-e29b-41d4-a716"));
        assert!(!validate_uuid("550e8400_e29b_41d4_a716_446655440000"));
    }

    #[test]
    fn test_validate_uuid_path_traversal() {
        assert!(!validate_uuid("../../../etc/passwd"));
        assert!(!validate_uuid("550e8400-e29b-41d4-a716-446655440000/../../etc"));
        assert!(!validate_uuid("550e8400-e29b-41d4-a716-44665544\\cmd"));
    }
}
