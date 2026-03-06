use crate::common::dav_error::DavError;

pub fn extract_vcard_fn(vcard_data: &str) -> Option<String> {
    for line in vcard_data.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("FN:") {
            return Some(value.to_string());
        }
    }
    None
}

pub fn extract_vcard_email(vcard_data: &str) -> Option<String> {
    for line in vcard_data.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("EMAIL") {
            if let Some(pos) = trimmed.find(':') {
                return Some(trimmed[pos + 1..].to_string());
            }
        }
    }
    None
}

pub fn extract_vcard_uid(vcard_data: &str) -> Result<String, DavError> {
    for line in vcard_data.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("UID:") {
            return Ok(value.to_string());
        }
    }
    Err(DavError::BadRequest("vCard data missing UID".into()))
}

pub fn validate_vcard(data: &str) -> Result<(), DavError> {
    if !data.contains("BEGIN:VCARD") {
        return Err(DavError::BadRequest("Not a valid vCard (missing BEGIN:VCARD)".into()));
    }
    if !data.contains("END:VCARD") {
        return Err(DavError::BadRequest("Not a valid vCard (missing END:VCARD)".into()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_VCARD: &str = "BEGIN:VCARD\r\nVERSION:3.0\r\nUID:contact-123@miyucloud\r\nFN:Jean Dupont\r\nEMAIL:jean@example.com\r\nEND:VCARD";

    #[test]
    fn test_extract_fn() {
        assert_eq!(extract_vcard_fn(SAMPLE_VCARD), Some("Jean Dupont".into()));
    }

    #[test]
    fn test_extract_uid() {
        assert_eq!(extract_vcard_uid(SAMPLE_VCARD).unwrap(), "contact-123@miyucloud");
    }

    #[test]
    fn test_validate_vcard() {
        assert!(validate_vcard(SAMPLE_VCARD).is_ok());
        assert!(validate_vcard("not a vcard").is_err());
    }
}
