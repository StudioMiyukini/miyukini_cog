use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::caldav::service::CalDavService;
use crate::common::dav_error::DavError;
use crate::common::path_validator;
use crate::webdav::response::{self, DavProperty, PropfindResponse};

const CALDAV_PREFIX: &str = "/caldav";

/// Path segments after /caldav/:
/// - /caldav/{owner_id}/ -> calendar home
/// - /caldav/{owner_id}/{cal_name}/ -> calendar collection
/// - /caldav/{owner_id}/{cal_name}/{uid}.ics -> event resource
struct CalDavPath<'a> {
    owner_id: &'a str,
    calendar_name: Option<&'a str>,
    event_uid: Option<&'a str>,
}

fn parse_caldav_path(uri_path: &str) -> Result<CalDavPath<'_>, DavError> {
    let after_prefix = uri_path
        .strip_prefix(CALDAV_PREFIX)
        .unwrap_or(uri_path)
        .trim_start_matches('/');

    if after_prefix.is_empty() {
        return Err(DavError::BadRequest("Missing owner in CalDAV path".into()));
    }

    let segments: Vec<&str> = after_prefix.split('/').filter(|s| !s.is_empty()).collect();

    match segments.len() {
        1 => Ok(CalDavPath {
            owner_id: segments[0],
            calendar_name: None,
            event_uid: None,
        }),
        2 => Ok(CalDavPath {
            owner_id: segments[0],
            calendar_name: Some(segments[1]),
            event_uid: None,
        }),
        3 => {
            let uid = segments[2].strip_suffix(".ics").unwrap_or(segments[2]);
            Ok(CalDavPath {
                owner_id: segments[0],
                calendar_name: Some(segments[1]),
                event_uid: Some(uid),
            })
        }
        _ => Err(DavError::BadRequest("Invalid CalDAV path depth".into())),
    }
}

/// Handle CalDAV PROPFIND: calendar home or calendar collection
pub async fn handle_propfind(
    State(service): State<CalDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_caldav_path(&validated)?;

    // Consume body (may contain propfind XML, ignored for now - allprop)
    let _body = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;

    match (parsed.calendar_name, parsed.event_uid) {
        // Calendar home: list calendars
        (None, _) => {
            let calendars = service.list_calendars(parsed.owner_id)?;
            let mut responses = vec![PropfindResponse {
                href: validated.clone(),
                properties: vec![
                    dav_prop("displayname", Some(parsed.owner_id)),
                    dav_prop("resourcetype", Some("<D:collection/><C:calendar/>")),
                ],
                not_found_properties: vec![],
            }];
            for cal in &calendars {
                let href = format!("{}/{}/", validated.trim_end_matches('/'), cal.display_name);
                responses.push(calendar_propfind_response(&cal, &href));
            }
            let xml = response::build_multistatus(&responses);
            Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
        }
        // Calendar collection: list events
        (Some(cal_name), None) => {
            let cal = service
                .get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?
                .ok_or_else(|| DavError::NotFound(format!("Calendar '{cal_name}' not found")))?;
            let events = service.list_events(&cal.id.to_string())?;

            let mut responses = vec![calendar_propfind_response(&cal, &validated)];
            for event in &events {
                let href = format!("{}{}.ics", ensure_trailing_slash(&validated), event.uid);
                responses.push(event_propfind_response(event, &href));
            }
            let xml = response::build_multistatus(&responses);
            Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
        }
        // Single event
        (Some(cal_name), Some(uid)) => {
            let cal = service
                .get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?
                .ok_or_else(|| DavError::NotFound(format!("Calendar '{cal_name}' not found")))?;
            let event = service
                .get_event(&cal.id.to_string(), uid)?
                .ok_or_else(|| DavError::NotFound(format!("Event '{uid}' not found")))?;
            let responses = vec![event_propfind_response(&event, &validated)];
            let xml = response::build_multistatus(&responses);
            Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
        }
    }
}

/// Handle CalDAV REPORT (calendar-query, calendar-multiget)
pub async fn handle_report(
    State(service): State<CalDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_caldav_path(&validated)?;

    let body_bytes = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;
    let body_str = std::str::from_utf8(&body_bytes)
        .map_err(|e| DavError::BadRequest(format!("Invalid UTF-8: {e}")))?;

    let cal_name = parsed
        .calendar_name
        .ok_or_else(|| DavError::BadRequest("REPORT requires a calendar path".into()))?;
    let cal = service
        .get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?
        .ok_or_else(|| DavError::NotFound(format!("Calendar '{cal_name}' not found")))?;

    let cal_id = cal.id.to_string();

    // Detect report type from body
    let events = if body_str.contains("calendar-multiget") {
        // Extract hrefs from multiget body
        let uids = extract_uids_from_hrefs(body_str);
        service.get_events_by_uids(&cal_id, &uids)?
    } else {
        // calendar-query: return all events (simplified - no time-range filter parsing yet)
        service.list_events(&cal_id)?
    };

    let base_href = ensure_trailing_slash(&validated);
    let responses: Vec<PropfindResponse> = events
        .iter()
        .map(|e| {
            let href = format!("{}{}.ics", base_href, e.uid);
            event_report_response(e, &href)
        })
        .collect();

    let xml = response::build_multistatus(&responses);
    Ok((StatusCode::MULTI_STATUS, [("content-type", "application/xml; charset=utf-8")], xml).into_response())
}

/// Handle GET: download event .ics
pub async fn handle_get(
    State(service): State<CalDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_caldav_path(&validated)?;

    let cal_name = parsed
        .calendar_name
        .ok_or_else(|| DavError::MethodNotAllowed)?;
    let uid = parsed
        .event_uid
        .ok_or_else(|| DavError::MethodNotAllowed)?;

    let cal = service
        .get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?
        .ok_or_else(|| DavError::NotFound(format!("Calendar '{cal_name}' not found")))?;
    let event = service
        .get_event(&cal.id.to_string(), uid)?
        .ok_or_else(|| DavError::NotFound(format!("Event '{uid}' not found")))?;

    Ok((
        StatusCode::OK,
        [
            ("content-type", "text/calendar; charset=utf-8"),
            ("etag", &format!("\"{}\"", event.etag)),
        ],
        event.ical_data,
    )
        .into_response())
}

/// Handle PUT: create or update event
pub async fn handle_put(
    State(service): State<CalDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_caldav_path(&validated)?;

    let cal_name = parsed
        .calendar_name
        .ok_or_else(|| DavError::BadRequest("PUT requires calendar/event path".into()))?;
    let uid = parsed
        .event_uid
        .ok_or_else(|| DavError::BadRequest("PUT requires event UID in path".into()))?;

    let body_bytes = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;
    let ical_data = std::str::from_utf8(&body_bytes)
        .map_err(|e| DavError::BadRequest(format!("Invalid UTF-8: {e}")))?;

    let cal = service
        .get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?
        .ok_or_else(|| DavError::NotFound(format!("Calendar '{cal_name}' not found")))?;

    let cal_id = cal.id.to_string();
    let existing = service.get_event(&cal_id, uid)?;

    let (event, status) = if existing.is_some() {
        (service.update_event(&cal_id, uid, ical_data)?, StatusCode::NO_CONTENT)
    } else {
        (service.create_event(&cal_id, ical_data)?, StatusCode::CREATED)
    };

    Ok((
        status,
        [("etag", format!("\"{}\"", event.etag).as_str())],
    )
        .into_response())
}

/// Handle DELETE: delete calendar or event
pub async fn handle_delete(
    State(service): State<CalDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_caldav_path(&validated)?;

    let cal_name = parsed
        .calendar_name
        .ok_or_else(|| DavError::Forbidden("Cannot delete calendar home".into()))?;

    let cal = service
        .get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?
        .ok_or_else(|| DavError::NotFound(format!("Calendar '{cal_name}' not found")))?;

    match parsed.event_uid {
        Some(uid) => {
            service.delete_event(&cal.id.to_string(), uid)?;
        }
        None => {
            service.delete_calendar(&cal.id.to_string())?;
        }
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}

/// Handle MKCALENDAR: create a new calendar
pub async fn handle_mkcalendar(
    State(service): State<CalDavService>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated = path_validator::validate_dav_path(&path)?;
    let parsed = parse_caldav_path(&validated)?;

    let cal_name = parsed
        .calendar_name
        .ok_or_else(|| DavError::BadRequest("MKCALENDAR requires calendar name in path".into()))?;

    // Check if already exists
    if service.get_calendar_by_owner_and_name(parsed.owner_id, cal_name)?.is_some() {
        return Err(DavError::MethodNotAllowed);
    }

    let _body = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;

    service.create_calendar(parsed.owner_id, cal_name)?;
    Ok(StatusCode::CREATED.into_response())
}

/// Handle OPTIONS
pub async fn handle_options() -> Response {
    (
        StatusCode::OK,
        [
            ("allow", "OPTIONS, GET, HEAD, PUT, DELETE, PROPFIND, REPORT, MKCALENDAR"),
            ("dav", "1, 2, calendar-access"),
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

fn calendar_propfind_response(
    cal: &crate::caldav::types::Calendar,
    href: &str,
) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: vec![
            dav_prop("displayname", Some(&cal.display_name)),
            dav_prop("resourcetype", Some("<D:collection/><C:calendar/>")),
            DavProperty {
                namespace: "urn:ietf:params:xml:ns:caldav".into(),
                name: "getctag".into(),
                value: Some(cal.ctag.clone()),
            },
        ],
        not_found_properties: vec![],
    }
}

fn event_propfind_response(
    event: &crate::caldav::types::CalendarEvent,
    href: &str,
) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: vec![
            dav_prop("getetag", Some(&format!("\"{}\"", event.etag))),
            dav_prop("getcontenttype", Some("text/calendar; charset=utf-8")),
        ],
        not_found_properties: vec![],
    }
}

fn event_report_response(
    event: &crate::caldav::types::CalendarEvent,
    href: &str,
) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: vec![
            dav_prop("getetag", Some(&format!("\"{}\"", event.etag))),
            DavProperty {
                namespace: "urn:ietf:params:xml:ns:caldav".into(),
                name: "calendar-data".into(),
                value: Some(event.ical_data.clone()),
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
    // Simple extraction: find <D:href>.../{uid}.ics</D:href> patterns
    let mut uids = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(href_content) = extract_between(trimmed, "<D:href>", "</D:href>")
            .or_else(|| extract_between(trimmed, "<href>", "</href>"))
        {
            if let Some(uid) = href_content
                .rsplit('/')
                .next()
                .and_then(|s| s.strip_suffix(".ics"))
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
