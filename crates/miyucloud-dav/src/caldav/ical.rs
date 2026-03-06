use crate::common::dav_error::DavError;

/// Minimal iCalendar parser for VCALENDAR/VEVENT extraction
pub fn extract_vevent_summary(ical_data: &str) -> Option<String> {
    for line in ical_data.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("SUMMARY:") {
            return Some(value.to_string());
        }
    }
    None
}

pub fn extract_vevent_dtstart(ical_data: &str) -> Option<String> {
    for line in ical_data.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("DTSTART") {
            if let Some(pos) = trimmed.find(':') {
                return Some(trimmed[pos + 1..].to_string());
            }
        }
    }
    None
}

pub fn extract_vevent_dtend(ical_data: &str) -> Option<String> {
    for line in ical_data.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("DTEND") {
            if let Some(pos) = trimmed.find(':') {
                return Some(trimmed[pos + 1..].to_string());
            }
        }
    }
    None
}

pub fn extract_vevent_uid(ical_data: &str) -> Result<String, DavError> {
    for line in ical_data.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix("UID:") {
            return Ok(value.to_string());
        }
    }
    Err(DavError::BadRequest("iCalendar data missing UID".into()))
}

/// Validate that the input looks like valid iCalendar data
pub fn validate_ical(data: &str) -> Result<(), DavError> {
    if !data.contains("BEGIN:VCALENDAR") {
        return Err(DavError::BadRequest(
            "Not a valid iCalendar document (missing BEGIN:VCALENDAR)".into(),
        ));
    }
    if !data.contains("END:VCALENDAR") {
        return Err(DavError::BadRequest(
            "Not a valid iCalendar document (missing END:VCALENDAR)".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_VEVENT: &str = "BEGIN:VCALENDAR\r\nVERSION:2.0\r\nBEGIN:VEVENT\r\nUID:test-uid-123@miyucloud\r\nSUMMARY:Team Meeting\r\nDTSTART:20260306T100000Z\r\nDTEND:20260306T110000Z\r\nEND:VEVENT\r\nEND:VCALENDAR";

    #[test]
    fn test_extract_summary() {
        assert_eq!(extract_vevent_summary(SAMPLE_VEVENT), Some("Team Meeting".into()));
    }

    #[test]
    fn test_extract_uid() {
        assert_eq!(extract_vevent_uid(SAMPLE_VEVENT).unwrap(), "test-uid-123@miyucloud");
    }

    #[test]
    fn test_validate_ical() {
        assert!(validate_ical(SAMPLE_VEVENT).is_ok());
        assert!(validate_ical("not ical data").is_err());
    }
}
