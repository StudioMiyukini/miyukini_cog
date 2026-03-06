use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::carddav::service::CardDavService;
use crate::common::dav_error::DavError;
use crate::common::path_validator;
use crate::webdav::response::{self, DavProperty, PropfindResponse};

const CARDDAV_PREFIX: &str = "/carddav";

struct CardDavPath<'a> {
    owner_id: &'a str,
    book_name: Option<&'a str>,
    contact_uid: Option<&'a str>,
}

fn parse_carddav_path(uri_path: &str) -> Result<CardDavPath<'_>, DavError> {
    let after_prefix = uri_path
        .strip_prefix(CARDDAV_PREFIX)
        .unwrap_or(uri_path)
        .trim_start_matches('/');

    if after_prefix.is_empty() {
        return Err(DavError::BadRequest("Missing owner in CardDAV path".into()));
    }

    let segments: Vec<&str> = after_prefix.split('/').filter(|s| !s.is_empty()).collect();

    match segments.len() {
        1 => Ok(CardDavPath {
            owner_id: segments[0],
            book_name: None,
            contact_uid: None,
        }),
        2 => Ok(CardDavPath {
            owner_id: segments[0],
            book_name: Some(segments[1]),
            contact_uid: None,
        }),
        3 => {
            let uid = segments[2].strip_suffix(".vcf").unwrap_or(segments[2]);
            Ok(CardDavPath {
                owner_id: segments[0],
                book_name: Some(segments[1]),
                contact_uid: Some(uid),
            })
        }
        _ => Err(DavError::BadRequest("Invalid CardDAV path depth".into())),
    }
}

pub async fn handle_propfind(
    State(service): State<CardDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_carddav_path(&validated)?;

    let _body = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;

    match (parsed.book_name, parsed.contact_uid) {
        (None, _) => {
            let books = service.list_addressbooks(parsed.owner_id)?;
            let mut responses = vec![PropfindResponse {
                href: validated.clone(),
                properties: vec![
                    dav_prop("displayname", Some(parsed.owner_id)),
                    dav_prop("resourcetype", Some("<D:collection/>")),
                ],
                not_found_properties: vec![],
            }];
            for ab in &books {
                let href = format!("{}/{}/", validated.trim_end_matches('/'), ab.display_name);
                responses.push(addressbook_propfind_response(ab, &href));
            }
            let xml = response::build_multistatus(&responses);
            Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
        }
        (Some(book_name), None) => {
            let ab = service
                .get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?
                .ok_or_else(|| DavError::NotFound(format!("Address book '{book_name}' not found")))?;
            let contacts = service.list_contacts(&ab.id.to_string())?;

            let mut responses = vec![addressbook_propfind_response(&ab, &validated)];
            for contact in &contacts {
                let href = format!("{}{}.vcf", ensure_trailing_slash(&validated), contact.uid);
                responses.push(contact_propfind_response(contact, &href));
            }
            let xml = response::build_multistatus(&responses);
            Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
        }
        (Some(book_name), Some(uid)) => {
            let ab = service
                .get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?
                .ok_or_else(|| DavError::NotFound(format!("Address book '{book_name}' not found")))?;
            let contact = service
                .get_contact(&ab.id.to_string(), uid)?
                .ok_or_else(|| DavError::NotFound(format!("Contact '{uid}' not found")))?;
            let responses = vec![contact_propfind_response(&contact, &validated)];
            let xml = response::build_multistatus(&responses);
            Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
        }
    }
}

pub async fn handle_report(
    State(service): State<CardDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_carddav_path(&validated)?;

    let body_bytes = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;
    let body_str = std::str::from_utf8(&body_bytes)
        .map_err(|e| DavError::BadRequest(format!("Invalid UTF-8: {e}")))?;

    let book_name = parsed
        .book_name
        .ok_or_else(|| DavError::BadRequest("REPORT requires addressbook path".into()))?;
    let ab = service
        .get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?
        .ok_or_else(|| DavError::NotFound(format!("Address book '{book_name}' not found")))?;

    let ab_id = ab.id.to_string();

    let contacts = if body_str.contains("addressbook-multiget") {
        let uids = extract_uids_from_hrefs(body_str);
        service.get_contacts_by_uids(&ab_id, &uids)?
    } else {
        service.list_contacts(&ab_id)?
    };

    let base_href = ensure_trailing_slash(&validated);
    let responses: Vec<PropfindResponse> = contacts
        .iter()
        .map(|c| {
            let href = format!("{}{}.vcf", base_href, c.uid);
            contact_report_response(c, &href)
        })
        .collect();

    let xml = response::build_multistatus(&responses);
    Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
}

pub async fn handle_get(
    State(service): State<CardDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_carddav_path(&validated)?;

    let book_name = parsed.book_name.ok_or_else(|| DavError::MethodNotAllowed)?;
    let uid = parsed.contact_uid.ok_or_else(|| DavError::MethodNotAllowed)?;

    let ab = service
        .get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?
        .ok_or_else(|| DavError::NotFound(format!("Address book '{book_name}' not found")))?;
    let contact = service
        .get_contact(&ab.id.to_string(), uid)?
        .ok_or_else(|| DavError::NotFound(format!("Contact '{uid}' not found")))?;

    Ok((
        StatusCode::OK,
        [
            ("content-type", "text/vcard; charset=utf-8"),
            ("etag", &format!("\"{}\"", contact.etag)),
        ],
        contact.vcard_data,
    )
        .into_response())
}

pub async fn handle_put(
    State(service): State<CardDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_carddav_path(&validated)?;

    let book_name = parsed
        .book_name
        .ok_or_else(|| DavError::BadRequest("PUT requires addressbook/contact path".into()))?;
    let uid = parsed
        .contact_uid
        .ok_or_else(|| DavError::BadRequest("PUT requires contact UID in path".into()))?;

    let body_bytes = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;
    let vcard_data = std::str::from_utf8(&body_bytes)
        .map_err(|e| DavError::BadRequest(format!("Invalid UTF-8: {e}")))?;

    let ab = service
        .get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?
        .ok_or_else(|| DavError::NotFound(format!("Address book '{book_name}' not found")))?;

    let ab_id = ab.id.to_string();
    let existing = service.get_contact(&ab_id, uid)?;

    let (contact, status) = if existing.is_some() {
        (service.update_contact(&ab_id, uid, vcard_data)?, StatusCode::NO_CONTENT)
    } else {
        (service.create_contact(&ab_id, vcard_data)?, StatusCode::CREATED)
    };

    Ok((
        status,
        [("etag", format!("\"{}\"", contact.etag).as_str())],
    )
        .into_response())
}

pub async fn handle_delete(
    State(service): State<CardDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_carddav_path(&validated)?;

    let book_name = parsed
        .book_name
        .ok_or_else(|| DavError::Forbidden("Cannot delete addressbook home".into()))?;

    let ab = service
        .get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?
        .ok_or_else(|| DavError::NotFound(format!("Address book '{book_name}' not found")))?;

    match parsed.contact_uid {
        Some(uid) => service.delete_contact(&ab.id.to_string(), uid)?,
        None => service.delete_addressbook(&ab.id.to_string())?,
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub async fn handle_mkcol(
    State(service): State<CardDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_carddav_path(&validated)?;

    let book_name = parsed
        .book_name
        .ok_or_else(|| DavError::BadRequest("MKCOL requires addressbook name in path".into()))?;

    if service.get_addressbook_by_owner_and_name(parsed.owner_id, book_name)?.is_some() {
        return Err(DavError::MethodNotAllowed);
    }

    let _body = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;

    service.create_addressbook(parsed.owner_id, book_name)?;
    Ok(StatusCode::CREATED.into_response())
}

pub async fn handle_options() -> Response {
    (
        StatusCode::OK,
        [
            ("allow", "OPTIONS, GET, HEAD, PUT, DELETE, PROPFIND, REPORT, MKCOL"),
            ("dav", "1, 2, addressbook"),
        ],
        Body::empty(),
    )
        .into_response()
}

// -- Helpers --

fn dav_prop(name: &str, value: Option<&str>) -> DavProperty {
    DavProperty {
        namespace: "DAV:".into(),
        name: name.into(),
        value: value.map(String::from),
    }
}

fn addressbook_propfind_response(
    ab: &crate::carddav::types::AddressBook,
    href: &str,
) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: vec![
            dav_prop("displayname", Some(&ab.display_name)),
            dav_prop("resourcetype", Some("<D:collection/><CR:addressbook/>")),
            DavProperty {
                namespace: "urn:ietf:params:xml:ns:carddav".into(),
                name: "getctag".into(),
                value: Some(ab.ctag.clone()),
            },
        ],
        not_found_properties: vec![],
    }
}

fn contact_propfind_response(
    contact: &crate::carddav::types::Contact,
    href: &str,
) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: vec![
            dav_prop("getetag", Some(&format!("\"{}\"", contact.etag))),
            dav_prop("getcontenttype", Some("text/vcard; charset=utf-8")),
        ],
        not_found_properties: vec![],
    }
}

fn contact_report_response(
    contact: &crate::carddav::types::Contact,
    href: &str,
) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: vec![
            dav_prop("getetag", Some(&format!("\"{}\"", contact.etag))),
            DavProperty {
                namespace: "urn:ietf:params:xml:ns:carddav".into(),
                name: "address-data".into(),
                value: Some(contact.vcard_data.clone()),
            },
        ],
        not_found_properties: vec![],
    }
}

fn ensure_trailing_slash(path: &str) -> String {
    if path.ends_with('/') {
        path.to_string()
    } else {
        format!("{path}/")
    }
}

fn extract_uids_from_hrefs(body: &str) -> Vec<String> {
    let mut uids = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(href_content) = extract_between(trimmed, "<D:href>", "</D:href>")
            .or_else(|| extract_between(trimmed, "<href>", "</href>"))
        {
            if let Some(uid) = href_content
                .rsplit('/')
                .next()
                .and_then(|s| s.strip_suffix(".vcf"))
            {
                uids.push(uid.to_string());
            }
        }
    }
    uids
}

fn extract_between<'a>(s: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_pos = s.find(start)? + start.len();
    let end_pos = s[start_pos..].find(end)? + start_pos;
    Some(&s[start_pos..end_pos])
}
