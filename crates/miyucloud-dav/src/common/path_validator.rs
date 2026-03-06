use super::dav_error::DavError;

/// Validates and normalizes a WebDAV path.
/// Rejects path traversal attempts, null bytes, and invalid characters.
pub fn validate_dav_path(path: &str) -> Result<String, DavError> {
    // Reject null bytes
    if path.contains('\0') {
        return Err(DavError::PathValidation(
            "Null bytes not allowed in paths".into(),
        ));
    }

    // Percent-decode for validation
    let decoded = percent_encoding::percent_decode_str(path)
        .decode_utf8()
        .map_err(|e| DavError::PathValidation(format!("Invalid UTF-8 in path: {e}")))?;

    // Reject path traversal
    for segment in decoded.split('/') {
        if segment == ".." {
            return Err(DavError::PathValidation(
                "Path traversal (..) not allowed".into(),
            ));
        }
        if segment == "." {
            continue;
        }
        // Reject backslashes (Windows path traversal)
        if segment.contains('\\') {
            return Err(DavError::PathValidation(
                "Backslashes not allowed in paths".into(),
            ));
        }
    }

    // Normalize: remove double slashes, ensure leading slash
    let normalized = normalize_path(&decoded);

    // Reject overly long paths
    if normalized.len() > 4096 {
        return Err(DavError::PathValidation("Path too long (max 4096)".into()));
    }

    Ok(normalized)
}

fn normalize_path(path: &str) -> String {
    let mut segments: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        segments.push(segment);
    }

    if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    }
}

/// Extracts the user-relative path from a full DAV request path.
/// E.g. `/dav/files/user123/Documents/file.txt` -> `Documents/file.txt`
pub fn extract_relative_path(full_path: &str, prefix: &str) -> Result<String, DavError> {
    let validated = validate_dav_path(full_path)?;
    let stripped = validated
        .strip_prefix(prefix)
        .unwrap_or(&validated)
        .trim_start_matches('/');

    Ok(stripped.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_path() {
        assert_eq!(validate_dav_path("/files/doc.txt").unwrap(), "/files/doc.txt");
    }

    #[test]
    fn test_traversal_rejected() {
        assert!(validate_dav_path("/files/../etc/passwd").is_err());
        assert!(validate_dav_path("/files/..%2F..%2Fetc%2Fpasswd").is_err());
    }

    #[test]
    fn test_null_bytes_rejected() {
        assert!(validate_dav_path("/files/\0evil").is_err());
    }

    #[test]
    fn test_backslash_rejected() {
        assert!(validate_dav_path("/files/evil\\path").is_err());
    }

    #[test]
    fn test_normalize_double_slash() {
        assert_eq!(validate_dav_path("/files//doc.txt").unwrap(), "/files/doc.txt");
    }

    #[test]
    fn test_root_path() {
        assert_eq!(validate_dav_path("/").unwrap(), "/");
    }

    #[test]
    fn test_extract_relative() {
        assert_eq!(
            extract_relative_path("/dav/files/user/Documents/file.txt", "/dav/files/user").unwrap(),
            "Documents/file.txt"
        );
    }
}
