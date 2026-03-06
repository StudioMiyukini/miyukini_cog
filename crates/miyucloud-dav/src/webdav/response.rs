use quick_xml::Writer;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::io::Cursor;

/// A WebDAV property with namespace, name and value
#[derive(Debug, Clone)]
pub struct DavProperty {
    pub namespace: String,
    pub name: String,
    pub value: Option<String>,
}

/// A single response in a multistatus
#[derive(Debug)]
pub struct PropfindResponse {
    pub href: String,
    pub properties: Vec<DavProperty>,
    pub not_found_properties: Vec<DavProperty>,
}

/// Build a DAV multistatus XML response
pub fn build_multistatus(responses: &[PropfindResponse]) -> String {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    // XML declaration
    let _ = writer.write_event(Event::Decl(
        quick_xml::events::BytesDecl::new("1.0", Some("utf-8"), None),
    ));

    // <D:multistatus>
    let mut ms = BytesStart::new("D:multistatus");
    ms.push_attribute(("xmlns:D", "DAV:"));
    let _ = writer.write_event(Event::Start(ms));

    for resp in responses {
        // <D:response>
        let _ = writer.write_event(Event::Start(BytesStart::new("D:response")));

        // <D:href>
        let _ = writer.write_event(Event::Start(BytesStart::new("D:href")));
        let _ = writer.write_event(Event::Text(BytesText::new(&resp.href)));
        let _ = writer.write_event(Event::End(BytesEnd::new("D:href")));

        // <D:propstat> for found properties
        if !resp.properties.is_empty() {
            let _ = writer.write_event(Event::Start(BytesStart::new("D:propstat")));
            let _ = writer.write_event(Event::Start(BytesStart::new("D:prop")));

            for prop in &resp.properties {
                let tag = format!("D:{}", prop.name);
                if let Some(ref val) = prop.value {
                    let _ = writer.write_event(Event::Start(BytesStart::new(&*tag)));
                    let _ = writer.write_event(Event::Text(BytesText::new(val)));
                    let _ = writer.write_event(Event::End(BytesEnd::new(&*tag)));
                } else {
                    let _ = writer.write_event(Event::Empty(BytesStart::new(&*tag)));
                }
            }

            let _ = writer.write_event(Event::End(BytesEnd::new("D:prop")));

            // <D:status>
            let _ = writer.write_event(Event::Start(BytesStart::new("D:status")));
            let _ = writer.write_event(Event::Text(BytesText::new("HTTP/1.1 200 OK")));
            let _ = writer.write_event(Event::End(BytesEnd::new("D:status")));

            let _ = writer.write_event(Event::End(BytesEnd::new("D:propstat")));
        }

        // <D:propstat> for not-found properties
        if !resp.not_found_properties.is_empty() {
            let _ = writer.write_event(Event::Start(BytesStart::new("D:propstat")));
            let _ = writer.write_event(Event::Start(BytesStart::new("D:prop")));

            for prop in &resp.not_found_properties {
                let tag = format!("D:{}", prop.name);
                let _ = writer.write_event(Event::Empty(BytesStart::new(&*tag)));
            }

            let _ = writer.write_event(Event::End(BytesEnd::new("D:prop")));
            let _ = writer.write_event(Event::Start(BytesStart::new("D:status")));
            let _ = writer.write_event(Event::Text(BytesText::new("HTTP/1.1 404 Not Found")));
            let _ = writer.write_event(Event::End(BytesEnd::new("D:status")));
            let _ = writer.write_event(Event::End(BytesEnd::new("D:propstat")));
        }

        let _ = writer.write_event(Event::End(BytesEnd::new("D:response")));
    }

    let _ = writer.write_event(Event::End(BytesEnd::new("D:multistatus")));

    let result = writer.into_inner().into_inner();
    String::from_utf8(result).unwrap_or_default()
}
