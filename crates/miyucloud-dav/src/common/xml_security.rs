use super::dav_error::DavError;
use quick_xml::events::Event;
use quick_xml::Reader;

/// Maximum XML document size (1 MB)
const MAX_XML_SIZE: usize = 1_048_576;

/// Maximum XML nesting depth
const MAX_XML_DEPTH: usize = 64;

/// Maximum number of XML elements
const MAX_XML_ELEMENTS: usize = 10_000;

/// Validates and parses XML input with security constraints.
/// - Rejects documents exceeding size limit
/// - Rejects excessive nesting depth
/// - Rejects excessive element count
/// - DTD/DOCTYPE declarations are rejected (XXE prevention)
pub fn validate_xml(input: &[u8]) -> Result<(), DavError> {
    if input.len() > MAX_XML_SIZE {
        return Err(DavError::XmlParse(format!(
            "XML document too large: {} bytes (max {MAX_XML_SIZE})",
            input.len()
        )));
    }

    let mut reader = Reader::from_reader(input);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::with_capacity(256);
    let mut depth: usize = 0;
    let mut element_count: usize = 0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => {
                depth += 1;
                element_count += 1;

                if depth > MAX_XML_DEPTH {
                    return Err(DavError::XmlParse(format!(
                        "XML nesting too deep: {depth} (max {MAX_XML_DEPTH})"
                    )));
                }
                if element_count > MAX_XML_ELEMENTS {
                    return Err(DavError::XmlParse(format!(
                        "Too many XML elements: {element_count} (max {MAX_XML_ELEMENTS})"
                    )));
                }
            }
            Ok(Event::End(_)) => {
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Empty(_)) => {
                element_count += 1;
                if element_count > MAX_XML_ELEMENTS {
                    return Err(DavError::XmlParse(format!(
                        "Too many XML elements: {element_count} (max {MAX_XML_ELEMENTS})"
                    )));
                }
            }
            Ok(Event::DocType(_)) => {
                return Err(DavError::XmlParse(
                    "DOCTYPE declarations are forbidden (XXE prevention)".into(),
                ));
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(DavError::XmlParse(format!("XML parse error: {e}"))),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Parses a validated XML body, returning a quick-xml Reader.
/// Always call `validate_xml` before this.
pub fn safe_xml_reader(input: &[u8]) -> Result<Reader<&[u8]>, DavError> {
    validate_xml(input)?;
    let mut reader = Reader::from_reader(input);
    reader.config_mut().trim_text(true);
    Ok(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_xml() {
        let xml = b"<?xml version=\"1.0\"?><root><child/></root>";
        assert!(validate_xml(xml).is_ok());
    }

    #[test]
    fn test_reject_doctype() {
        let xml = b"<?xml version=\"1.0\"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM \"file:///etc/passwd\">]><root/>";
        assert!(validate_xml(xml).is_err());
    }

    #[test]
    fn test_reject_too_large() {
        let xml = vec![b'<'; MAX_XML_SIZE + 1];
        assert!(validate_xml(&xml).is_err());
    }

    #[test]
    fn test_reject_deep_nesting() {
        let mut xml = String::new();
        for _ in 0..=MAX_XML_DEPTH {
            xml.push_str("<a>");
        }
        for _ in 0..=MAX_XML_DEPTH {
            xml.push_str("</a>");
        }
        assert!(validate_xml(xml.as_bytes()).is_err());
    }
}
